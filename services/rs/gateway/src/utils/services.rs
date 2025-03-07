use std::error::Error;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use tonic::{Code, Status};

use crate::models::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("This route isn't implemented yet...")]
    ServiceNotImplemented,

    #[error("Deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    AlreadyExists(String),

    #[error("Authentication error: {0}")]
    Unauthorized(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Error while validating input: {0}")]
    Validation(String),

    #[error("Something vent wrong...")]
    Unknown,
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        self.code()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiError {
            error: self.error_name(),
            description: self.to_string(),
        })
    }
}

impl ServiceError {
    pub fn code(&self) -> StatusCode {
        match self {
            Self::ServiceNotImplemented => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Json(..) => StatusCode::BAD_REQUEST,
            Self::Forbidden(..) => StatusCode::FORBIDDEN,
            Self::NotFound(..) => StatusCode::NOT_FOUND,
            Self::AlreadyExists(..) => StatusCode::CONFLICT,
            Self::Unauthorized(..) => StatusCode::UNAUTHORIZED,
            Self::InvalidInput(..) => StatusCode::BAD_REQUEST,
            Self::Validation(..) => StatusCode::BAD_REQUEST,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_name(&self) -> &'static str {
        match self {
            Self::ServiceNotImplemented => "service_not_implemented",
            Self::Json(..) => "json_error",
            Self::Forbidden(..) => "forbidden",
            Self::NotFound(..) => "not_found",
            Self::AlreadyExists(..) => "already_exists",
            Self::Unauthorized(..) => "unauthorized",
            Self::InvalidInput(..) => "invalid_input",
            Self::Validation(..) => "invalid_input",
            Self::Unknown => "unknown_error",
        }
    }
}

impl From<Status> for ServiceError {
    fn from(status: Status) -> Self {
        match (
            status.code(),
            status.message(),
            status.details(),
            status.metadata(),
            status.source(),
        ) {
            (Code::PermissionDenied, message, ..) => Self::Forbidden(message.to_string()),
            (Code::AlreadyExists, message, ..) => Self::AlreadyExists(message.to_string()),
            (Code::NotFound, message, ..) => Self::NotFound(message.to_string()),
            (Code::Unauthenticated, message, ..) => Self::Unauthorized(message.to_string()),
            (Code::InvalidArgument, message, ..) => Self::InvalidInput(message.to_string()),
            (Code::Unimplemented, ..) => Self::ServiceNotImplemented,
            (Code::Internal, ..) => Self::Unknown,
            (Code::DeadlineExceeded, ..) => Self::Unknown,
            (Code::Unknown, message, ..) => {
                log::debug!("{message:#?}");
                Self::Unknown
            }
            _ => {
                log::error!("{:?}", status);
                unreachable!()
            }
        }
    }
}
