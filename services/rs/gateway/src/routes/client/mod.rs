use crate::{
    auth::middleware::{client_auth_middleware, AuthEntity},
    models::{
        dto::{Client, ClientUpdate},
        ApiError as ApiErrorModel,
    },
    utils::{cors::default_cors, services::ServiceError, validation::validation_errors_to_err},
};
use actix_web::{
    delete, get,
    middleware::from_fn,
    patch,
    web::Json,
    web::{Data, ReqData},
    HttpResponse,
};
use protos::client::{client_client::ClientClient, ClientRequest};
use tonic::{transport::Channel, Request};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

use super::ApiError;

mod by_id;
mod login;
mod password;
mod register;
mod reservations;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/client")
            .wrap(default_cors())
            .service(register::post_handler)
            .service(login::post_handler)
            .service(
                scope("")
                    .wrap(default_cors())
                    .wrap(from_fn(client_auth_middleware))
                    .service(get_handler)
                    .service(patch_handler)
                    .service(delete_handler)
                    .service(password::put_handler)
                    .service(reservations::get_handler)
                    .configure(by_id::config),
            ),
    );
}

#[utoipa::path(
    tag = "clients",
    operation_id = "get_current_client_account",
    description = "Get current client account",
    security(
        ("client" = [])
    ),
    responses(
        (status = 200, body = Client),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("")]
async fn get_handler(
    client_client: Data<ClientClient<Channel>>,
    entity: ReqData<AuthEntity>,
) -> Result<Json<Client>, ApiError> {
    let client = entity.into_inner().into_client()?;

    let request = Request::new(ClientRequest {
        id: client.id.to_string(),
    });

    let response = (&**client_client)
        .clone()
        .get(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "clients",
    operation_id = "edit_client_account",
    description = "Edit client account",
    security(
        ("client" = [])
    ),
    responses(
        (status = 200, description = "Profile was successfully edited", body = Client),
        (status = 400, description = "Invalid body", body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[patch("")]
async fn patch_handler(
    client_client: Data<ClientClient<Channel>>,
    entity: ReqData<AuthEntity>,
    Json(body): Json<ClientUpdate>,
) -> Result<Json<Client>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let client = entity.into_inner().into_client()?;

    let request = Request::new(body.into_proto(client.id));

    let response = (&**client_client)
        .clone()
        .edit(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "clients",
    operation_id = "delete_client_account",
    description = "Delete client account",
    security(
        ("client" = [])
    ),
    responses(
        (status = 204),
        (status = 400, description = "Invalid password", body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[delete("")]
async fn delete_handler(
    client_client: Data<ClientClient<Channel>>,
    entity: ReqData<AuthEntity>,
) -> Result<HttpResponse, ApiError> {
    let client = entity.into_inner().into_client()?;

    let request = Request::new(ClientRequest {
        id: client.id.to_string(),
    });

    (&**client_client)
        .clone()
        .delete(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(HttpResponse::NoContent().into())
}
