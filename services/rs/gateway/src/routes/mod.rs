use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

mod admin;
mod client;
mod coworking;
mod health;
mod not_found;
mod ping;
mod reservations;

use crate::{
    auth::AuthenticationError,
    utils::{cors::default_cors, services::ServiceError},
};

pub use self::not_found::not_found;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .wrap(default_cors())
            .service(ping::get_handler)
            .service(health::get_handler)
            .configure(admin::config)
            .configure(client::config)
            .configure(coworking::config)
            .configure(reservations::config),
    );
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("{0}")]
    Service(#[from] ServiceError),

    #[error("Resource not found")]
    NotFound,

    #[error("You're not allowed to do this")]
    NotOwner,

    #[error("Deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Error while validating input: {0}")]
    Validation(String),

    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),
}

impl ApiError {
    pub fn as_api_error<'a>(&self) -> crate::models::ApiError<'a> {
        crate::models::ApiError {
            error: match self {
                Self::Service(err) => err.error_name(),
                Self::NotFound => "not_found",
                Self::NotOwner => "not_owner",
                Self::Json(..) => "json_error",
                Self::InvalidInput(..) => "invalid_input",
                Self::Validation(..) => "invalid_input",
                Self::Authentication(err) => err.error_name(),
            },
            description: self.to_string(),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Service(err) => err.code(),
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::NotOwner => StatusCode::FORBIDDEN,
            Self::Json(..) => StatusCode::BAD_REQUEST,
            Self::InvalidInput(..) => StatusCode::BAD_REQUEST,
            Self::Validation(..) => StatusCode::BAD_REQUEST,
            Self::Authentication(err) => err.status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}
