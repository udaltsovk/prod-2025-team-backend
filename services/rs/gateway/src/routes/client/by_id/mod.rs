use crate::{
    auth::middleware::admin_auth_middleware,
    models::{
        dto::{Client, ClientUpdate},
        url::ClientPath,
        ApiError as ApiErrorModel,
    },
    routes::ApiError,
    utils::{cors::default_cors, services::ServiceError, validation::validation_errors_to_err},
};
use actix_web::{
    delete, get,
    middleware::from_fn,
    patch,
    web::{Data, Json},
    HttpResponse,
};
use actix_web_lab::extract::Path;
use protos::client::{client_client::ClientClient, ClientRequest};
use tonic::{transport::Channel, Request};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

mod reservations;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{client_id}")
            .wrap(default_cors())
            .wrap(from_fn(admin_auth_middleware))
            .service(get_handler)
            .service(patch_handler)
            .service(delete_handler)
            .service(reservations::get_handler),
    );
}

#[utoipa::path(
    tag = "clients",
    operation_id = "get_client_by_id",
    description = "Get client by ID",
    security(
        ("admin" = [])
    ),
    params(
        ("client_id" = Uuid, description = "Client ID")
    ),
    responses(
        (status = 200, body = Client),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("")]
async fn get_handler(
    client_client: Data<ClientClient<Channel>>,
    Path(path): Path<ClientPath>,
) -> Result<Json<Client>, ApiError> {
    let request = Request::new(ClientRequest {
        id: path.client_id.to_string(),
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
    operation_id = "edit_client_by_id",
    description = "Edit client by ID",
    security(
        ("admin" = [])
    ),
    params(
        ("client_id" = Uuid, description = "Client ID")
    ),
    responses(
        (status = 200, description = "Profile was successfully edited", body = Client),
        (status = 404, body = ApiErrorModel),
        (status = 400, description = "Invalid body", body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[patch("")]
pub async fn patch_handler(
    client_client: Data<ClientClient<Channel>>,
    Path(path): Path<ClientPath>,
    Json(body): Json<ClientUpdate>,
) -> Result<Json<Client>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let request = Request::new(body.into_proto(path.client_id));

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
    operation_id = "delete_client_by_id",
    description = "Delete client by ID",
    security(
        ("admin" = [])
    ),
    params(
        ("client_id" = Uuid, description = "Client ID")
    ),
    responses(
        (status = 204),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[delete("")]
pub async fn delete_handler(
    client_client: Data<ClientClient<Channel>>,
    Path(path): Path<ClientPath>,
) -> Result<HttpResponse, ApiError> {
    let request = Request::new(ClientRequest {
        id: path.client_id.to_string(),
    });

    (&**client_client)
        .clone()
        .delete(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(HttpResponse::NoContent().into())
}
