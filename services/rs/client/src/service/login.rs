use argon2::{Argon2, PasswordHash, PasswordVerifier};
use protos::client::{AuthResponse, LoginRequest};
use sqlx::PgPool;

use crate::{models::db::DBClient, utils::token};

use super::error::ServiceError;

pub async fn handle(req: LoginRequest, pool: &PgPool) -> Result<AuthResponse, ServiceError> {
    let client = match DBClient::get_by_email(&req.email, pool).await? {
        None => return Err(ServiceError::InvalidCredentials),
        Some(client) => client,
    };

    let hasher = Argon2::default();
    hasher
        .verify_password(
            req.password.as_bytes(),
            &PasswordHash::new(&client.password_hash)?,
        )
        .map_err(|_| ServiceError::InvalidCredentials)?;

    Ok(AuthResponse {
        token: token::new(client.id),
        client: client.into(),
    })
}
