use protos::client::{ClientRequest, ClientResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::DBClient;

use super::error::ServiceError;

pub async fn handle(req: ClientRequest, pool: &PgPool) -> Result<ClientResponse, ServiceError> {
    let id = Uuid::parse_str(&req.id).unwrap();

    match DBClient::get_by_id(id, pool).await? {
        None => Err(ServiceError::NotFound),
        Some(client) => Ok(client.into()),
    }
}
