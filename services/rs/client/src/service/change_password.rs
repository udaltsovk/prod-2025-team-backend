use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::Utc;
use protos::client::{AuthResponse, ChangePasswordRequest};
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::db::DBClient, utils::token};

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

    let client = DBClient::patch(
        id,
        None,
        None,
        None,
        None,
        Some(&password_hash),
        Some(Utc::now()),
        None,
        None,
        None,
        &mut transaction,
    )
    .await?;

    transaction.commit().await?;

    match client {
        None => Err(ServiceError::NotFound),
        Some(client) => Ok(AuthResponse {
            token: token::new(client.id),
            client: client.into(),
        }),
    }
}
