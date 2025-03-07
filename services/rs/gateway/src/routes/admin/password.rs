use crate::{
    auth::middleware::AuthEntity,
    models::{
        request::ChangePasswordRequest, response::AdminAuthResponse, ApiError as ApiErrorModel,
    },
    routes::ApiError,
    utils::{services::ServiceError, validation::validation_errors_to_err},
};
use actix_web::{
    put,
    web::{Data, Json, ReqData},
};
use protos::admin::{
    admin_client::AdminClient, ChangePasswordRequest as ProtoChangePasswordRequest,
};
use tonic::{transport::Channel, Request};
use validator::Validate;

#[utoipa::path(
    tag = "admins",
    operation_id = "change_admin_password",
    description = "Change admin account password",
    security(
        ("admin" = [])
    ),
    responses(
        (status = 200, description = "Password was successfully changed", body = AdminAuthResponse),
        (status = 400, description = "Password is too weak", body = ApiErrorModel),
        (status = 400, description = "New password confirmation failed", body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[put("/password")]
pub async fn put_handler(
    admin_client: Data<AdminClient<Channel>>,
    entity: ReqData<AuthEntity>,
    body: Json<ChangePasswordRequest>,
) -> Result<Json<AdminAuthResponse>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let client = entity.into_inner().into_admin()?;

    let request = Request::new(ProtoChangePasswordRequest {
        id: client.id.to_string(),
        current_password: body.current_password.clone(),
        new_password: body.new_password.clone(),
    });

    let response = (&**admin_client)
        .clone()
        .change_password(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}
