use actix_web::{body::EitherBody, web::Json, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

mod date;
pub mod dto;
pub mod request;
pub mod response;
pub mod url;

pub use date::Date;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ApiError<'a> {
    pub error: &'a str,
    pub description: String,
}

#[derive(Serialize, Debug)]
pub struct EmptyResponse {
    status: String,
}

impl Default for EmptyResponse {
    fn default() -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
}

impl Responder for EmptyResponse {
    type Body = EitherBody<String>;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        Json(self).respond_to(req)
    }
}
