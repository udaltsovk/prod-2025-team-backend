use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::Utc;
use protos::client::{AuthResponse, RegisterRequest};
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::db::DBClient, utils::token};

use super::error::ServiceError;

pub async fn handle(req: RegisterRequest, pool: &PgPool) -> Result<AuthResponse, ServiceError> {
    if DBClient::get_by_email(&req.meta.email, pool)
        .await?
        .is_some()
    {
        return Err(ServiceError::AlreadyExists)?;
    }

    let id = Uuid::now_v7();
    let hasher = Argon2::default();
    let salt = SaltString::generate(&mut ChaCha20Rng::from_entropy());
    let password_hash = hasher
        .hash_password(req.password.as_bytes(), &salt)?
        .to_string();

    let mut transaction = pool.begin().await?;

    let client = DBClient {
        id,
        name: req.meta.name,
        surname: req.meta.surname,
        patronymic: req.meta.patronymic,
        email: req.meta.email,
        password_hash,
        last_password_change: Utc::now(),
        send_notifications: req.meta.send_notifications,
        is_internal: req.meta.is_internal,
        verified: false,
        deleted: false,
    }
    .insert(&mut transaction)
    .await?;

    transaction.commit().await?;

    Ok(AuthResponse {
        token: token::new(client.id),
        client: client.into(),
    })
}
