use protos::client::ClientRequest;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::DBClient;

use super::error::ServiceError;

pub async fn handle(req: ClientRequest, pool: &PgPool) -> Result<(), ServiceError> {
    let id = Uuid::parse_str(&req.id).unwrap();

    let mut transaction = pool.begin().await?;

    DBClient::delete(id, &mut transaction).await?;

    transaction.commit().await?;

    Ok(())
}
