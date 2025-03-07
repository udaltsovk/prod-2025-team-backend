use actix_web::{get, HttpResponse};

#[utoipa::path(
    tag = "health",
    operation_id = "ping",
    description = "Test that API is working",
    responses(
        (status = 200, description = "API is working"),
    ),
)]
#[get("/ping")]
pub async fn get_handler() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("content-type", "text/html"))
        .body(
            r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Pong!</title>
                </head>
                <body>
                    <img alt="Средний балл п*т*н*ст*в второго этапа PROD: 8,979274611" src="https://i.imgur.com/rwpaNH5.png" />
                </body>
            </html>
        "#,
        )
}
