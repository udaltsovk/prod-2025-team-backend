use crate::{
    auth::middleware::{admin_auth_middleware, any_auth_middleware, AuthEntity},
    models::{
        dto::{Reservation, ReservationUpdate},
        url::ReservationPath,
        ApiError as ApiErrorModel,
    },
    routes::ApiError,
    utils::{cors::default_cors, services::ServiceError, validation::validation_errors_to_err},
};
use actix_web::{
    delete, get,
    middleware::from_fn,
    patch,
    web::{Data, Json, ReqData},
    HttpResponse,
};
use actix_web_lab::extract::Path;
use protos::reservation::{reservation_client::ReservationClient, DeleteRequest, GetByIdRequest};
use tonic::{transport::Channel, Request};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

mod confirm;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{reservation_id}")
            .wrap(default_cors())
            .wrap(from_fn(any_auth_middleware))
            .service(get_handler)
            .service(patch_handler)
            .service(delete_handler)
            .service(
                scope("")
                    .wrap(default_cors())
                    .wrap(from_fn(admin_auth_middleware))
                    .service(confirm::post_handler),
            ),
    );
}

#[utoipa::path(
    tag = "reservations",
    operation_id = "get_reservation",
    description = "Get reservation by ID",
    security(
        ("admin" = []),
        ("client" = [])
    ),
    params(
        ("reservation_id" = Uuid, description = "Reservation ID")
    ),
    responses(
        (status = 200, body = Reservation),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("")]
async fn get_handler(
    reservation_client: Data<ReservationClient<Channel>>,
    entity: ReqData<AuthEntity>,
    Path(path): Path<ReservationPath>,
) -> Result<Json<Reservation>, ApiError> {
    let (id, is_admin) = match entity.into_inner() {
        AuthEntity::Client(client) => (client.id, false),
        AuthEntity::Admin(admin) => (admin.id, true),
    };

    let request = Request::new(GetByIdRequest {
        id: path.reservation_id.to_string(),
        client_id: id.to_string(),
        is_admin,
    });

    let response = (&**reservation_client)
        .clone()
        .get_by_id(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "reservations",
    operation_id = "reschedule_reservation",
    description = "Edit reservation by ID",
    security(
        ("admin" = []),
        ("client" = [])
    ),
    params(
        ("reservation_id" = Uuid, description = "Reservation ID")
    ),
    responses(
        (status = 200, body = Reservation),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[patch("")]
async fn patch_handler(
    reservation_client: Data<ReservationClient<Channel>>,
    entity: ReqData<AuthEntity>,
    Path(path): Path<ReservationPath>,
    Json(body): Json<ReservationUpdate>,
) -> Result<Json<Reservation>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let (id, is_admin) = match entity.into_inner() {
        AuthEntity::Client(client) => (client.id, false),
        AuthEntity::Admin(admin) => (admin.id, true),
    };

    let request = Request::new(body.into_proto(path.reservation_id, id, is_admin));

    let response = (&**reservation_client)
        .clone()
        .update(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "reservations",
    operation_id = "cancel_reservation",
    description = "Cancel reservation by ID",
    security(
        ("admin" = []),
        ("client" = [])
    ),
    params(
        ("reservation_id" = Uuid, description = "Reservation ID")
    ),
    responses(
        (status = 204),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[delete("")]
async fn delete_handler(
    reservation_client: Data<ReservationClient<Channel>>,
    entity: ReqData<AuthEntity>,
    Path(path): Path<ReservationPath>,
) -> Result<HttpResponse, ApiError> {
    let (id, is_admin) = match entity.into_inner() {
        AuthEntity::Client(client) => (client.id, false),
        AuthEntity::Admin(admin) => (admin.id, true),
    };

    let request = Request::new(DeleteRequest {
        id: path.reservation_id.to_string(),
        client_id: id.to_string(),
        is_admin,
    });

    (&**reservation_client)
        .clone()
        .delete(request)
        .await
        .map_err(ServiceError::from)?;

    Ok(HttpResponse::NoContent().into())
}
