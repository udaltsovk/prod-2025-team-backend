use chrono::Utc;
use protos::admin::{AdminResponse, ValidateTokenRequest};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::db::DBAdmin, utils::token};

use super::error::ServiceError;

pub async fn handle(
    req: ValidateTokenRequest,
    pool: &PgPool,
) -> Result<AdminResponse, ServiceError> {
    let claims = match token::parse(req.token) {
        None => return Err(ServiceError::InvalidToken),
        Some(claims) => claims,
    };

    if claims.iat >= Utc::now().timestamp() as usize {
        return Err(ServiceError::InvalidToken);
    }

    let id = Uuid::parse_str(&claims.sub).map_err(|_| ServiceError::InvalidToken)?;

    let admin = DBAdmin::get_by_id(id, pool).await?;

    match admin {
        None => Err(ServiceError::InvalidToken),
        Some(admin) => Ok(admin.into()),
    }
}
