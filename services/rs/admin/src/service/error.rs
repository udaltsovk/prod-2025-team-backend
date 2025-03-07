use tonic::{Code, Status};

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Hasher error: {0}")]
    Hasher(#[from] argon2::password_hash::Error),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Admin was not found")]
    NotFound,

    #[error("Admin with this email already exists")]
    AlreadyExists,
}

impl Into<Status> for ServiceError {
    fn into(self) -> Status {
        let code: Code = match self {
            Self::Database(..) => Code::Internal,
            Self::Hasher(..) => Code::Internal,
            Self::InvalidCredentials => Code::Unauthenticated,
            Self::InvalidToken => Code::Unauthenticated,
            Self::NotFound => Code::NotFound,
            Self::AlreadyExists => Code::AlreadyExists,
        };

        Status::new(code, self.to_string())
    }
}
