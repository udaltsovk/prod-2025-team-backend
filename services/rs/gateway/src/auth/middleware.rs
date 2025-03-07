use crate::{
    models::dto::{Admin, Client},
    routes::ApiError,
    utils::services::ServiceError,
};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::{HeaderValue, AUTHORIZATION},
    middleware::Next,
    web::Data,
    Error, HttpMessage,
};
use jsonwebtoken::decode_header;
use protos::client::ValidateTokenRequest as ValidateClientTokenRequest;
use protos::{
    admin::{admin_client::AdminClient, ValidateTokenRequest as ValidateAdminTokenRequest},
    client::client_client::ClientClient,
};
use tonic::{transport::Channel, Request};

use super::AuthenticationError;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Any,
    Client,
    Admin,
}

#[derive(Clone, Debug)]
pub enum AuthEntity {
    Client(Client),
    Admin(Admin),
}
impl AuthEntity {
    pub fn into_client(self) -> Result<Client, AuthenticationError> {
        match self {
            AuthEntity::Admin(..) => Err(AuthenticationError::IcorrectTokenType)?,
            AuthEntity::Client(client) => Ok(client),
        }
    }

    pub fn into_admin(self) -> Result<Admin, AuthenticationError> {
        match self {
            AuthEntity::Client(..) => Err(AuthenticationError::IcorrectTokenType)?,
            AuthEntity::Admin(admin) => Ok(admin),
        }
    }
}

pub async fn any_auth_middleware(
    admin_client: Data<AdminClient<Channel>>,
    client_client: Data<ClientClient<Channel>>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    auth_middleware(TokenType::Any, admin_client, client_client, req, next).await
}

pub async fn client_auth_middleware(
    admin_client: Data<AdminClient<Channel>>,
    client_client: Data<ClientClient<Channel>>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    auth_middleware(TokenType::Client, admin_client, client_client, req, next).await
}

pub async fn admin_auth_middleware(
    admin_client: Data<AdminClient<Channel>>,
    client_client: Data<ClientClient<Channel>>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    auth_middleware(TokenType::Admin, admin_client, client_client, req, next).await
}

pub async fn auth_middleware(
    access_level: TokenType,
    admin_client: Data<AdminClient<Channel>>,
    client_client: Data<ClientClient<Channel>>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let token = extract_auth_from_authorization_header(&req)?;

    let token_type = match decode_header(&token).map(|h| h.kid.map(|k| k.clone())) {
        Ok(Some(token_type)) if &token_type == "admin" => {
            let mut service_client = (&**admin_client).clone();

            let request = Request::new(ValidateAdminTokenRequest { token });

            let admin: Admin = service_client
                .validate_token(request)
                .await
                .map_err(ServiceError::from)?
                .into_inner()
                .into();

            req.extensions_mut()
                .insert(AuthEntity::Admin(admin.clone()));

            TokenType::Admin
        }
        Ok(Some(token_type)) if &token_type == "client" => {
            let mut service_client = (&**client_client).clone();

            let request = Request::new(ValidateClientTokenRequest { token });

            let client: Client = service_client
                .validate_token(request)
                .await
                .map_err(ServiceError::from)?
                .into_inner()
                .into();

            req.extensions_mut()
                .insert(AuthEntity::Client(client.clone()));

            TokenType::Client
        }
        _ => return Err(AuthenticationError::InvalidCredentials)?,
    };

    match access_level {
        TokenType::Any => (),
        level => {
            if level != token_type {
                return Err(ApiError::NotFound)?;
            }
        }
    }

    next.call(req).await
}

pub fn extract_auth_from_authorization_header(
    req: &ServiceRequest,
) -> Result<String, AuthenticationError> {
    let headers = req.headers();
    let token_val: Option<&HeaderValue> = headers.get(AUTHORIZATION);
    let token_val = token_val
        .ok_or_else(|| AuthenticationError::NoAuthorizationHeader)?
        .to_str()
        .map_err(|_| AuthenticationError::InvalidCredentials)?;

    if let Some(token) = token_val.strip_prefix("Bearer ") {
        Ok(token.to_string())
    } else {
        Err(AuthenticationError::InvalidAuthMethod)
    }
}
