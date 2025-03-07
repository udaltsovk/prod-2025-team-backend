use protos::admin::{AdminResponse, EditRequest};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::DBAdmin;

use super::error::ServiceError;

pub async fn handle(req: EditRequest, pool: &PgPool) -> Result<AdminResponse, ServiceError> {
    let id = Uuid::parse_str(&req.id).unwrap();

    let mut transaction = pool.begin().await?;

    let admin = DBAdmin::patch(id, req.email.as_deref(), None, None, &mut transaction).await?;

    transaction.commit().await?;

    match admin {
        None => Err(ServiceError::NotFound),
        Some(admin) => Ok(admin.into()),
    }
}
