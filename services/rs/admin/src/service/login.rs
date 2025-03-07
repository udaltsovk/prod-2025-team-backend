use argon2::{Argon2, PasswordHash, PasswordVerifier};
use protos::admin::{AuthResponse, LoginRequest};
use sqlx::PgPool;

use crate::{models::db::DBAdmin, utils::token};

use super::error::ServiceError;

pub async fn handle(req: LoginRequest, pool: &PgPool) -> Result<AuthResponse, ServiceError> {
    let admin = match DBAdmin::get_by_email(&req.email, pool).await? {
        None => return Err(ServiceError::InvalidCredentials),
        Some(admin) => admin,
    };

    let hasher = Argon2::default();
    hasher
        .verify_password(
            req.password.as_bytes(),
            &PasswordHash::new(&admin.password_hash)?,
        )
        .map_err(|_| ServiceError::InvalidCredentials)?;

    Ok(AuthResponse {
        token: token::new(admin.id),
        admin: admin.into(),
    })
}
