use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::Utc;
use protos::admin::{AuthResponse, RegisterRequest};
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::db::DBAdmin, utils::token};

use super::error::ServiceError;

pub async fn handle(req: RegisterRequest, pool: &PgPool) -> Result<AuthResponse, ServiceError> {
    if DBAdmin::get_by_email(&req.email, pool).await?.is_some() {
        return Err(ServiceError::AlreadyExists)?;
    }

    let id = Uuid::now_v7();
    let hasher = Argon2::default();
    let salt = SaltString::generate(&mut ChaCha20Rng::from_entropy());
    let password_hash = hasher
        .hash_password(req.password.as_bytes(), &salt)?
        .to_string();

    let mut transaction = pool.begin().await?;

    let admin = DBAdmin {
        id,
        email: req.email,
        password_hash,
        last_password_change: Utc::now(),
        deleted: false,
    }
    .insert(&mut transaction)
    .await?;

    transaction.commit().await?;

    Ok(AuthResponse {
        token: token::new(admin.id),
        admin: admin.into(),
    })
}
