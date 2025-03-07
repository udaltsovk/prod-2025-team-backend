use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

use crate::models::ApiError;

pub mod middleware;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("Error while parsing JSON: {0}")]
    SerDe(#[from] serde_json::Error),

    #[error("Missing Authorization Header")]
    NoAuthorizationHeader,

    #[error("Invalid Authentication Credentials")]
    InvalidCredentials,

    #[error("Authentication method was not valid")]
    InvalidAuthMethod,

    #[error("Incorrect token type")]
    IcorrectTokenType,

    #[error("User email/account is already registered")]
    DuplicateUser,
}

impl ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::SerDe(..) => StatusCode::BAD_REQUEST,
            Self::NoAuthorizationHeader => StatusCode::UNAUTHORIZED,
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::InvalidAuthMethod => StatusCode::UNAUTHORIZED,
            Self::IcorrectTokenType => StatusCode::FORBIDDEN,
            Self::DuplicateUser => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiError {
            error: self.error_name(),
            description: self.to_string(),
        })
    }
}

impl AuthenticationError {
    pub fn error_name(&self) -> &'static str {
        match self {
            Self::SerDe(..) => "invalid_input",
            Self::NoAuthorizationHeader => "missing_authorization_header",
            Self::InvalidCredentials => "invalid_credentials",
            Self::InvalidAuthMethod => "invalid_auth_method",
            Self::IcorrectTokenType => "incorrect_token_type",
            Self::DuplicateUser => "duplicate_user",
        }
    }
}
