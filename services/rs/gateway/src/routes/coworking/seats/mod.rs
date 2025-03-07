use crate::{
    auth::middleware::admin_auth_middleware,
    models::{
        dto::{CreateSeat, Seat},
        url::Pagination,
        ApiError as ApiErrorModel,
    },
    routes::ApiError,
    utils::{cors::default_cors, services::ServiceError, validation::validation_errors_to_err},
};
use actix_web::{
    get,
    middleware::from_fn,
    post,
    web::{Data, Json},
};
use actix_web_lab::extract::Query;
use protos::coworking::coworking_client::CoworkingClient;
use tonic::{transport::Channel, Request};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

mod by_id;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/seats")
            .wrap(default_cors())
            .service(get_handler)
            .service(
                scope("")
                    .wrap(default_cors())
                    .wrap(from_fn(admin_auth_middleware))
                    .service(post_handler)
                    .configure(by_id::config),
            ),
    );
}

#[utoipa::path(
    tag = "coworkings",
    operation_id = "create_seat",
    description = "Creates a new seat in the coworking",
    security(
        ("admin" = []),
    ),
    responses(
        (status = 201, body = Seat),
        (status = 404, body = ApiErrorModel),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[post("")]
async fn post_handler(
    coworking_client: Data<CoworkingClient<Channel>>,
    Json(body): Json<CreateSeat>,
) -> Result<Json<Seat>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let request = Request::new(body.into());

    let response = (&**coworking_client)
        .clone()
        .create_seat(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "coworkings",
    operation_id = "get_coworking_seats",
    description = "Get info about coworking seats",
    params(
        Pagination,
    ),
    responses(
        (status = 200, body = Vec<Seat>),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("")]
async fn get_handler(
    coworking_client: Data<CoworkingClient<Channel>>,
    Query(query): Query<Pagination>,
) -> Result<Json<Vec<Seat>>, ApiError> {
    let request = Request::new(query.into());

    let response = (&**coworking_client)
        .clone()
        .get_seats(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(Seat::vec_from_proto(response)))
}
