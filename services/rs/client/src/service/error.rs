use tonic::{Code, Status};

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("{0}")]
    Service(#[from] Status),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Hasher error: {0}")]
    Hasher(#[from] argon2::password_hash::Error),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Client was not found")]
    NotFound,

    #[error("Client with this email already exists")]
    AlreadyExists,
}

impl Into<Status> for ServiceError {
    fn into(self) -> Status {
        if let Self::Service(status) = self {
            return status;
        }

        let code: Code = match self {
            Self::Service(..) => unreachable!(),
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
