use super::ApiError;
use crate::{
    auth::middleware::admin_auth_middleware,
    models::{
        dto::{Coworking, CoworkingUpdate},
        ApiError as ApiErrorModel,
    },
    utils::{cors::default_cors, services::ServiceError, validation::validation_errors_to_err},
};
use actix_web::{
    get,
    middleware::from_fn,
    patch,
    web::{Data, Json},
};
use protos::coworking::{coworking_client::CoworkingClient, GetCoworkingByIdRequest};
use tonic::{transport::Channel, Request};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use uuid::Uuid;
use validator::Validate;

mod clients;
mod seats;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/coworking")
            .wrap(default_cors())
            .service(get_handler)
            .configure(seats::config)
            .service(
                scope("")
                    .wrap(default_cors())
                    .wrap(from_fn(admin_auth_middleware))
                    .service(patch_handler)
                    .service(clients::get_handler),
            ),
    );
}

#[utoipa::path(
    tag = "coworkings",
    operation_id = "get_coworking",
    description = "Get info about coworking",
    responses(
        (status = 200, body = Coworking),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("")]
async fn get_handler(
    coworking_client: Data<CoworkingClient<Channel>>,
    coworking_id: Data<Uuid>,
) -> Result<Json<Coworking>, ApiError> {
    let request = Request::new(GetCoworkingByIdRequest {
        id: coworking_id.to_string(),
    });

    let response = (&**coworking_client)
        .clone()
        .get_by_id(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}

#[utoipa::path(
    tag = "coworkings",
    operation_id = "edit_coworking",
    description = "Edit coworking info",
    security(
        ("admin" = []),
        ("client" = [])
    ),
    responses(
        (status = 200, body = Coworking),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[patch("")]
async fn patch_handler(
    coworking_client: Data<CoworkingClient<Channel>>,
    coworking_id: Data<Uuid>,
    Json(body): Json<CoworkingUpdate>,
) -> Result<Json<Coworking>, ApiError> {
    body.validate().map_err(validation_errors_to_err)?;

    let request = Request::new(body.into_proto(**coworking_id));

    let response = (&**coworking_client)
        .clone()
        .update(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(response.into()))
}
