use chrono::Utc;
use protos::client::{ClientResponse, ValidateTokenRequest};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::db::DBClient, utils::token};

use super::error::ServiceError;

pub async fn handle(
    req: ValidateTokenRequest,
    pool: &PgPool,
) -> Result<ClientResponse, ServiceError> {
    let claims = match token::parse(req.token) {
        None => return Err(ServiceError::InvalidToken),
        Some(claims) => claims,
    };

    if claims.iat >= Utc::now().timestamp() as usize {
        return Err(ServiceError::InvalidToken);
    }

    let id = Uuid::parse_str(&claims.sub).map_err(|_| ServiceError::InvalidToken)?;

    let client = DBClient::get_by_id(id, pool).await?;

    match client {
        None => Err(ServiceError::InvalidToken),
        Some(client) => Ok(client.into()),
    }
}
