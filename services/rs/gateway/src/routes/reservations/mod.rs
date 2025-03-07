use actix_web::{
    get,
    middleware::from_fn,
    post,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Query;
use prost_types::Timestamp;
use protos::{
    notification::{notification_client::NotificationClient, ScheduleRequest},
    reservation::reservation_client::ReservationClient,
};
use tonic::{transport::Channel, Request};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

use crate::{
    auth::middleware::{any_auth_middleware, AuthEntity},
    models::{
        dto::{CreateReservation, Reservation},
        url::ByDateWithPaginationQuery,
        ApiError as ApiErrorModel,
    },
    utils::{cors::default_cors, services::ServiceError, validation::validation_errors_to_err},
};

use super::ApiError;

mod by_id;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/reservations")
            .wrap(default_cors())
            .wrap(from_fn(any_auth_middleware))
            .service(post_handler)
            .configure(by_id::config),
    );
}

#[utoipa::path(
    tag = "reservations",
    operation_id = "create_reservation",
    description = "Creates a new reservation",
    security(
        ("admin" = []),
        ("client" = [])
    ),
    responses(
        (status = 200, body = Reservation),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[post("")]
async fn post_handler(
    reservation_client: Data<ReservationClient<Channel>>,
    notification_client: Data<NotificationClient<Channel>>,
    entity: ReqData<AuthEntity>,
    Json(body): Json<CreateReservation>,
) -> Result<Json<Reservation>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let (id, is_admin, send_notification) = match entity.into_inner() {
        AuthEntity::Client(client) => (client.id, false, client.send_notifications),
        AuthEntity::Admin(admin) => (admin.id, true, false),
    };

    let request = Request::new(body.into_proto(id, is_admin));

    let response = (&**reservation_client)
        .clone()
        .create(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    if send_notification {
        let send_at = Timestamp {
            seconds: response.starts_at.seconds - 3 * 60 * 60,
            nanos: 0,
        };

        (&**notification_client)
            .clone()
            .schedule(ScheduleRequest {
                id: id.to_string(),
                reservation_id: response.id.clone(),
                send_at,
            })
            .await
            .map_err(ServiceError::from)?;
    }

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "reservations",
    operation_id = "get_reservations",
    description = "Fetches reservations",
    security(
        ("client" = [])
    ),
    params(
        ByDateWithPaginationQuery
    ),
    responses(
        (status = 200, body = Vec<Reservation>),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("")]
async fn get_handler(
    reservation_client: Data<ReservationClient<Channel>>,
    Query(query): Query<ByDateWithPaginationQuery>,
) -> Result<Json<Vec<Reservation>>, ApiError> {
    query.validate().map_err(validation_errors_to_err)?;

    let request = Request::new(query.into());

    let response = (&**reservation_client)
        .clone()
        .get(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(Reservation::vec_from_proto(response)))
}
