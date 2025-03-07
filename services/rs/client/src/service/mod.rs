use error::ServiceError;
use protos::{
    client::{
        client_server::Client, AuthResponse, ChangePasswordRequest, ClientRequest, ClientResponse,
        ClientsRequest, ClientsResponse, EditRequest, LoginRequest, RegisterRequest,
        ValidateTokenRequest,
    },
    reservation::reservation_client::ReservationClient,
};
use service_helper::response::ServiceResult;
use sqlx::{Pool, Postgres};
use tonic::{async_trait, transport::Channel, Request, Response};

mod change_password;
mod delete;
mod edit;
mod error;
mod get;
mod get_multiple;
mod login;
mod register;
mod validate_token;

pub struct ClientService {
    pub postgres_pool: Pool<Postgres>,
    pub reservation_client: ReservationClient<Channel>,
}
impl ClientService {
    pub fn new(pool: Pool<Postgres>, reservation_client: ReservationClient<Channel>) -> Self {
        Self {
            postgres_pool: pool,
            reservation_client,
        }
    }
}

#[async_trait]
impl Client for ClientService {
    async fn register(&self, request: Request<RegisterRequest>) -> ServiceResult<AuthResponse> {
        register::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn login(&self, request: Request<LoginRequest>) -> ServiceResult<AuthResponse> {
        login::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn validate_token(
        &self,
        request: Request<ValidateTokenRequest>,
    ) -> ServiceResult<ClientResponse> {
        validate_token::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> ServiceResult<AuthResponse> {
        change_password::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn get(&self, request: Request<ClientRequest>) -> ServiceResult<ClientResponse> {
        get::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn get_multiple(
        &self,
        request: Request<ClientsRequest>,
    ) -> ServiceResult<ClientsResponse> {
        get_multiple::handle(
            request.into_inner(),
            &self.postgres_pool,
            &self.reservation_client,
        )
        .await
        .map(Response::new)
        .map_err(ServiceError::into)
    }

    async fn edit(&self, request: Request<EditRequest>) -> ServiceResult<ClientResponse> {
        edit::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }

    async fn delete(&self, request: Request<ClientRequest>) -> ServiceResult<()> {
        delete::handle(request.into_inner(), &self.postgres_pool)
            .await
            .map(Response::new)
            .map_err(ServiceError::into)
    }
}
