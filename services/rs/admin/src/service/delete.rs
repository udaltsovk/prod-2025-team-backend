use protos::admin::AdminRequest;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::DBAdmin;

use super::error::ServiceError;

pub async fn handle(req: AdminRequest, pool: &PgPool) -> Result<(), ServiceError> {
    let id = Uuid::parse_str(&req.id).unwrap();

    let mut transaction = pool.begin().await?;

    DBAdmin::delete(id, &mut transaction).await?;

    transaction.commit().await?;

    Ok(())
}
