use actix_web::{
    post,
    web::{Data, Json},
};
use protos::client::client_client::ClientClient;
use tonic::{transport::Channel, Request};
use validator::Validate;

use crate::{
    models::{dto::ClientForm, response::ClientAuthResponse, ApiError as ApiErrorModel},
    routes::ApiError,
    utils::{services::ServiceError, validation::validation_errors_to_err},
};

#[utoipa::path(
    tag = "clients",
    operation_id = "register_client",
    description = "Register new client account",
    responses(
        (status = 200, description = "Registration is successful", body = ClientAuthResponse),
        (status = 400, description = "Password is too weak", body = ApiErrorModel),
        (status = 400, description = "Request body isn't valid", body = ApiErrorModel)
    ),
)]
#[post("/register")]
pub async fn post_handler(
    client_client: Data<ClientClient<Channel>>,
    Json(body): Json<ClientForm>,
) -> Result<Json<ClientAuthResponse>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let request = Request::new(body.into());

    let response = (&**client_client)
        .clone()
        .register(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}
