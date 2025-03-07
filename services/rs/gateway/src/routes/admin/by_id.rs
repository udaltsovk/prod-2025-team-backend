use crate::{
    auth::middleware::admin_auth_middleware,
    models::{
        dto::{Admin, AdminUpdate},
        url::AdminPath,
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
use protos::admin::{admin_client::AdminClient, AdminRequest};
use tonic::{transport::Channel, Request};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{admin_id}")
            .wrap(default_cors())
            .wrap(from_fn(admin_auth_middleware))
            .service(get_handler)
            .service(patch_handler)
            .service(delete_handler),
    );
}

#[utoipa::path(
    tag = "admins",
    operation_id = "get_admin_by_id",
    description = "Get admin by ID",
    security(
        ("admin" = [])
    ),
    params(
        ("admin_id" = Uuid, description = "Admin ID")
    ),
    responses(
        (status = 200, body = Admin),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("")]
async fn get_handler(
    admin_client: Data<AdminClient<Channel>>,
    Path(path): Path<AdminPath>,
) -> Result<Json<Admin>, ApiError> {
    let request = Request::new(AdminRequest {
        id: path.admin_id.to_string(),
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
    operation_id = "edit_admin_by_id",
    description = "Edit admin by ID",
    security(
        ("admin" = [])
    ),
    params(
        ("admin_id" = Uuid, description = "Admin ID")
    ),
    responses(
        (status = 200, description = "Profile was successfully edited", body = Admin),
        (status = 404, body = ApiErrorModel),
        (status = 400, description = "Invalid body", body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[patch("")]
pub async fn patch_handler(
    admin_client: Data<AdminClient<Channel>>,
    Path(path): Path<AdminPath>,
    Json(body): Json<AdminUpdate>,
) -> Result<Json<Admin>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let request = Request::new(body.into_proto(path.admin_id));

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
    operation_id = "delete_admin_by_id",
    description = "Delete admin by ID",
    security(
        ("admin" = [])
    ),
    params(
        ("admin_id" = Uuid, description = "Admin ID")
    ),
    responses(
        (status = 204),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[delete("")]
pub async fn delete_handler(
    admin_client: Data<AdminClient<Channel>>,
    Path(path): Path<AdminPath>,
) -> Result<HttpResponse, ApiError> {
    let request = Request::new(AdminRequest {
        id: path.admin_id.to_string(),
    });

    (&**admin_client)
        .clone()
        .delete(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(HttpResponse::NoContent().into())
}
