use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::Utc;
use protos::admin::{AuthResponse, ChangePasswordRequest};
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::db::DBAdmin, utils::token};

use super::error::ServiceError;

pub async fn handle(
    req: ChangePasswordRequest,
    pool: &PgPool,
) -> Result<AuthResponse, ServiceError> {
    let id = Uuid::parse_str(&req.id).unwrap();

    let hasher = Argon2::default();
    let salt = SaltString::generate(&mut ChaCha20Rng::from_entropy());
    let password_hash = hasher
        .hash_password(req.new_password.as_bytes(), &salt)?
        .to_string();

    let mut transaction = pool.begin().await?;

    let admin = DBAdmin::patch(
        id,
        None,
        Some(&password_hash),
        Some(Utc::now()),
        &mut transaction,
    )
    .await?;

    transaction.commit().await?;

    match admin {
        None => Err(ServiceError::NotFound),
        Some(admin) => Ok(AuthResponse {
            token: token::new(admin.id),
            admin: admin.into(),
        }),
    }
}
