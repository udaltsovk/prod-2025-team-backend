use actix_web::{
    post,
    web::{Data, Json},
};
use protos::client::client_client::ClientClient;
use tonic::{transport::Channel, Request};
use validator::Validate;

use crate::{
    models::{dto::Credentials, response::ClientAuthResponse, ApiError as ApiErrorModel},
    routes::ApiError,
    utils::{services::ServiceError, validation::validation_errors_to_err},
};

#[utoipa::path(
    tag = "clients",
    operation_id = "client_login",
    description = "Log into client account",
    responses(
        (status = 200, description = "Logged in successfuly", body = ClientAuthResponse),
        (status = 400, description = "Request body isn't valid", body = ApiErrorModel)
    ),
)]
#[post("/login")]
pub async fn post_handler(
    client_client: Data<ClientClient<Channel>>,
    Json(body): Json<Credentials>,
) -> Result<Json<ClientAuthResponse>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let request = Request::new(body.into());

    let response = (&**client_client)
        .clone()
        .login(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}
