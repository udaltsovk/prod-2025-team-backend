use protos::admin::{AdminRequest, AdminResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::db::DBAdmin;

use super::error::ServiceError;

pub async fn handle(req: AdminRequest, pool: &PgPool) -> Result<AdminResponse, ServiceError> {
    let id = Uuid::parse_str(&req.id).unwrap();

    match DBAdmin::get_by_id(id, pool).await? {
        None => Err(ServiceError::NotFound),
        Some(admin) => Ok(admin.into()),
    }
}
