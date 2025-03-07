use crate::{
    auth::middleware::AuthEntity,
    models::{
        request::ChangePasswordRequest, response::ClientAuthResponse, ApiError as ApiErrorModel,
    },
    routes::ApiError,
    utils::{services::ServiceError, validation::validation_errors_to_err},
};
use actix_web::{
    put,
    web::{Data, Json, ReqData},
};
use protos::client::{
    client_client::ClientClient, ChangePasswordRequest as ProtoChangePasswordRequest,
};
use tonic::{transport::Channel, Request};
use validator::Validate;

#[utoipa::path(
    tag = "clients",
    operation_id = "change_client_password",
    description = "Change client account password",
    security(
        ("client" = [])
    ),
    responses(
        (status = 200, description = "Password was successfully changed", body = ClientAuthResponse),
        (status = 400, description = "Password is too weak", body = ApiErrorModel),
        (status = 400, description = "New password confirmation failed", body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[put("/password")]
pub async fn put_handler(
    client_client: Data<ClientClient<Channel>>,
    entity: ReqData<AuthEntity>,
    body: Json<ChangePasswordRequest>,
) -> Result<Json<ClientAuthResponse>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let client = entity.into_inner().into_client()?;

    let request = Request::new(ProtoChangePasswordRequest {
        id: client.id.to_string(),
        current_password: body.current_password.clone(),
        new_password: body.new_password.clone(),
    });

    let response = (&**client_client)
        .clone()
        .change_password(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}
