use actix_web::{get, HttpResponse};

#[utoipa::path(
    tag = "health",
    operation_id = "health",
    description = "Test that API is working",
    responses(
        (status = 200, description = "API is working"),
    ),
)]
#[get("/health")]
pub async fn get_handler() -> HttpResponse {
    HttpResponse::Ok().into()
}
