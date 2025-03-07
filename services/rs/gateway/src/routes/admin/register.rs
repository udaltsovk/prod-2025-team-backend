use actix_web::{
    post,
    web::{Data, Json},
};
use protos::admin::admin_client::AdminClient;
use tonic::{transport::Channel, Request};
use validator::Validate;

use crate::{
    models::{dto::AdminForm, response::AdminAuthResponse, ApiError as ApiErrorModel},
    routes::ApiError,
    utils::{services::ServiceError, validation::validation_errors_to_err},
};

#[utoipa::path(
    tag = "admins",
    operation_id = "register_admin",
    description = "Register new admin account",
    security(
        ("admin" = []) 
    ),
    responses(
        (status = 200, description = "Registration is successful", body = AdminAuthResponse),
        (status = 400, description = "Password is too weak", body = ApiErrorModel),
        (status = 400, description = "Request body isn't valid", body = ApiErrorModel)
    ),
)]
#[post("/register")]
pub async fn post_handler(
    admin_client: Data<AdminClient<Channel>>,
    Json(body): Json<AdminForm>,
) -> Result<Json<AdminAuthResponse>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let request = Request::new(body.into());

    let response = (&**admin_client)
        .clone()
        .register(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}
