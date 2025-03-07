use protos::client::{ClientResponse, EditRequest};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::DBClient;

use super::error::ServiceError;

pub async fn handle(req: EditRequest, pool: &PgPool) -> Result<ClientResponse, ServiceError> {
    let id = Uuid::parse_str(&req.id).unwrap();

    let mut transaction = pool.begin().await?;

    let client = DBClient::patch(
        id,
        req.name.as_deref(),
        req.surname.as_deref(),
        req.patronymic.as_deref(),
        req.email.as_deref(),
        None,
        None,
        req.send_notifications,
        req.is_internal,
        req.verified,
        &mut transaction,
    )
    .await?;

    transaction.commit().await?;

    match client {
        None => Err(ServiceError::NotFound),
        Some(client) => Ok(client.into()),
    }
}
