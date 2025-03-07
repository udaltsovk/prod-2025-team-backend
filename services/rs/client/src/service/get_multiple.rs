use protos::{
    client::{ClientResponse, ClientsRequest, ClientsResponse},
    reservation::{reservation_client::ReservationClient, GetRequest},
};
use sqlx::PgPool;
use tonic::transport::Channel;
use uuid::Uuid;

use crate::models::db::DBClient;

use super::error::ServiceError;

pub async fn handle(
    req: ClientsRequest,
    pool: &PgPool,
    reservation_client: &ReservationClient<Channel>,
) -> Result<ClientsResponse, ServiceError> {
    let ids = reservation_client
        .clone()
        .get_visited(GetRequest {
            limit: req.limit,
            offset: req.offset,
            day: req.day,
            month: req.month,
            year: req.year,
        })
        .await?
        .into_inner()
        .reservations
        .iter()
        .map(|r| Uuid::parse_str(&r.client_id).unwrap())
        .collect();

    let clients: Vec<ClientResponse> = DBClient::get_multiple(ids, pool)
        .await?
        .iter()
        .map(|client| (*client).clone().into())
        .collect();

    Ok(ClientsResponse { clients })
}
