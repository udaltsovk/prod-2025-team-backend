use actix_web::{App, HttpServer};
use actix_web_lab::middleware::CatchPanic;
use env_logger::Env;
use gateway::{
    app_setup, config,
    utils::{logger::CustomLogger, openapi::Swagger},
};
use std::io::Result;
use utoipa_actix_web::AppExt;

#[actix_rt::main]
async fn main() -> Result<()> {
    let default_log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::init_from_env(Env::default().default_filter_or(default_log_level));

    let config = app_setup().await;

    HttpServer::new(move || {
        App::new()
            .wrap(CatchPanic::default())
            .wrap(CustomLogger::new())
            .into_utoipa_app()
            .openapi(config.openapi.clone())
            .configure(config.clone().build())
            .openapi_service(Swagger::ui_service)
            .into_app()
    })
    .bind(config::SERVER_ADDRESS.as_str())?
    .run()
    .await
}
