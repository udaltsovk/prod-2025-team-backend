use actix_web::{
    post,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use protos::reservation::{reservation_client::ReservationClient, UpdateRequest};
use tonic::{transport::Channel, Request};

use crate::{
    auth::middleware::AuthEntity,
    models::{dto::Reservation, url::ReservationPath, ApiError as ApiErrorModel},
    routes::ApiError,
    utils::services::ServiceError,
};

#[utoipa::path(
    tag = "reservations",
    operation_id = "confirm_reservation",
    description = "Confirm reservation by ID",
    security(
        ("admin" = [])
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
#[post("/confirm")]
async fn post_handler(
    reservation_client: Data<ReservationClient<Channel>>,
    entity: ReqData<AuthEntity>,
    Path(path): Path<ReservationPath>,
) -> Result<Json<Reservation>, ApiError> {
    let (id, is_admin) = match entity.into_inner() {
        AuthEntity::Client(client) => (client.id, false),
        AuthEntity::Admin(admin) => (admin.id, true),
    };

    let request = Request::new(UpdateRequest {
        id: path.reservation_id.to_string(),
        client_id: id.to_string(),
        is_admin,
        seat_id: None,
        starts_at: None,
        ends_at: None,
        is_canceled: None,
        is_visited: Some(true),
    });

    let response = (&**reservation_client)
        .clone()
        .update(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}
