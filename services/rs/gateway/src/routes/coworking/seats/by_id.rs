use crate::{
    auth::middleware::admin_auth_middleware,
    models::{
        dto::{Seat, SeatUpdate},
        url::SeatPath,
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
use protos::coworking::{coworking_client::CoworkingClient, SeatRequest};
use tonic::{transport::Channel, Request};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{seat_id}")
            .wrap(default_cors())
            .wrap(from_fn(admin_auth_middleware))
            .service(get_handler)
            .service(patch_handler)
            .service(delete_handler),
    );
}

#[utoipa::path(
    tag = "coworkings",
    operation_id = "get_seat_by_id",
    description = "Get seat by ID",
    security(
        ("admin" = [])
    ),
    params(
        ("seat_id" = Uuid, description = "Seat ID")
    ),
    responses(
        (status = 200, body = Seat),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("")]
async fn get_handler(
    coworking_client: Data<CoworkingClient<Channel>>,
    Path(path): Path<SeatPath>,
) -> Result<Json<Seat>, ApiError> {
    let request = Request::new(SeatRequest {
        id: path.seat_id.to_string(),
    });

    let response = (&**coworking_client)
        .clone()
        .get_seat(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "coworkings",
    operation_id = "edit_seat_by_id",
    description = "Edit seat by ID",
    security(
        ("admin" = [])
    ),
    params(
        ("seat_id" = Uuid, description = "Seat ID")
    ),
    responses(
        (status = 200, description = "Seat was successfully edited", body = Seat),
        (status = 404, body = ApiErrorModel),
        (status = 400, description = "Invalid body", body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[patch("")]
pub async fn patch_handler(
    coworking_client: Data<CoworkingClient<Channel>>,
    Path(path): Path<SeatPath>,
    Json(body): Json<SeatUpdate>,
) -> Result<Json<Seat>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let request = Request::new(body.into_proto(path.seat_id));

    let response = (&**coworking_client)
        .clone()
        .update_seat(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "coworkings",
    operation_id = "delete_seat_by_id",
    description = "Delete seat by ID",
    security(
        ("admin" = [])
    ),
    params(
        ("seat_id" = Uuid, description = "Seat ID")
    ),
    responses(
        (status = 204),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[delete("")]
pub async fn delete_handler(
    coworking_client: Data<CoworkingClient<Channel>>,
    Path(path): Path<SeatPath>,
) -> Result<HttpResponse, ApiError> {
    let request = Request::new(SeatRequest {
        id: path.seat_id.to_string(),
    });

    (&**coworking_client)
        .clone()
        .delete_seat(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(HttpResponse::NoContent().into())
}
