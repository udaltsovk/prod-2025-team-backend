use actix_web::{
    get,
    web::{Data, Json, Query, ReqData},
};
use protos::reservation::reservation_client::ReservationClient;
use tonic::{transport::Channel, Request};
use validator::Validate;

use crate::{
    auth::middleware::AuthEntity,
    models::{dto::Reservation, url::Pagination, ApiError as ApiErrorModel},
    routes::ApiError,
    utils::{services::ServiceError, validation::validation_errors_to_err},
};

#[utoipa::path(
    tag = "clients",
    operation_id = "get_client_reservations",
    description = "Fetches client reservations",
    security(
        ("client" = [])
    ),
    params(
        Pagination
    ),
    responses(
        (status = 200, body = Vec<Reservation>),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("/reservations")]
async fn get_handler(
    reservation_client: Data<ReservationClient<Channel>>,
    entity: ReqData<AuthEntity>,
    Query(query): Query<Pagination>,
) -> Result<Json<Vec<Reservation>>, ApiError> {
    query.validate().map_err(validation_errors_to_err)?;

    let client = entity.into_inner().into_client()?;

    let request = Request::new(query.into_proto(client.id, false));

    let response = (&**reservation_client)
        .clone()
        .get_by_client(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(Reservation::vec_from_proto(response)))
}
