use actix_web::{
    get,
    web::{Data, Json},
};
use actix_web_lab::extract::Query;
use protos::client::client_client::ClientClient;
use tonic::{transport::Channel, Request};

use crate::{
    models::{dto::Client, url::ByDateWithPaginationQuery, ApiError as ApiErrorModel},
    routes::ApiError,
    utils::services::ServiceError,
};

#[utoipa::path(
    tag = "coworkings",
    operation_id = "get_clients",
    description = "Get clients in the coworking",
    params(
        ByDateWithPaginationQuery
    ),
    responses(
        (status = 200, body = Vec<Client>),
        (status = 401, description = "Unauthorized", body = ApiErrorModel)
    ),
)]
#[get("/clients")]
async fn get_handler(
    client_client: Data<ClientClient<Channel>>,
    Query(query): Query<ByDateWithPaginationQuery>,
) -> Result<Json<Vec<Client>>, ApiError> {
    let request = Request::new(query.into());

    let response = (&**client_client)
        .clone()
        .get_multiple(request)
        .await
        .map_err(ServiceError::from)?
        .into_inner();

    Ok(Json(
        response
            .clients
            .iter()
            .map(|c| Client::from(c.clone()))
            .collect(),
    ))
}
