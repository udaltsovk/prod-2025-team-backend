use crate::{
    auth::middleware::{admin_auth_middleware, AuthEntity},
    models::{
        dto::{Admin, AdminUpdate},
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
use protos::admin::{admin_client::AdminClient, AdminRequest};
use tonic::{transport::Channel, Request};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

use super::ApiError;

mod by_id;
mod login;
mod password;
mod register;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/admin")
            .wrap(default_cors())
            .service(login::post_handler)
            .service(
                scope("")
                    .wrap(default_cors())
                    .wrap(from_fn(admin_auth_middleware))
                    .service(register::post_handler)
                    .service(get_handler)
                    .service(patch_handler)
                    .service(delete_handler)
                    .service(password::put_handler)
                    .configure(by_id::config),
            ),
    );
}

#[utoipa::path(
    tag = "admins",
    operation_id = "get_current_admin_account",
    description = "Get current admin account",
    security(
        ("admin" = [])
    ),
    responses(
        (status = 200, body = Admin),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("")]
async fn get_handler(
    admin_client: Data<AdminClient<Channel>>,
    entity: ReqData<AuthEntity>,
) -> Result<Json<Admin>, ApiError> {
    let admin = entity.into_inner().into_admin()?;

    let request = Request::new(AdminRequest {
        id: admin.id.to_string(),
    });

    let response = (&**admin_client)
        .clone()
        .get(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "admins",
    operation_id = "edit_admin_account",
    description = "Edit client account",
    security(
        ("admin" = [])
    ),
    responses(
        (status = 200, description = "Profile was successfully edited", body = Admin),
        (status = 400, description = "Invalid body", body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[patch("")]
async fn patch_handler(
    admin_client: Data<AdminClient<Channel>>,
    entity: ReqData<AuthEntity>,
    Json(body): Json<AdminUpdate>,
) -> Result<Json<Admin>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let admin = entity.into_inner().into_admin()?;

    let request = Request::new(body.into_proto(admin.id));

    let response = (&**admin_client)
        .clone()
        .edit(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "admins",
    operation_id = "delete_admin_account",
    description = "Delete admin account",
    security(
        ("admin" = [])
    ),
    responses(
        (status = 204),
        (status = 400, description = "Invalid password", body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[delete("")]
async fn delete_handler(
    admin_client: Data<AdminClient<Channel>>,
    entity: ReqData<AuthEntity>,
) -> Result<HttpResponse, ApiError> {
    let admin = entity.into_inner().into_admin()?;

    let request = Request::new(AdminRequest {
        id: admin.id.to_string(),
    });

    (&**admin_client)
        .clone()
        .delete(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(HttpResponse::NoContent().into())
}
