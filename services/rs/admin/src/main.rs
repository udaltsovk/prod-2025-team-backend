use admin::{config, service::AdminService};
use env_logger::Env;
use protos::admin::admin_server::AdminServer;
use tonic::{codec::CompressionEncoding, transport::Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::init_from_env(Env::default().default_filter_or(default_log_level));
    config::init();

    postgres_helper::migrate!(&config::DATABASE_URL, "./db/migrations");

    let pool = postgres_helper::connect(&config::DATABASE_URL)
        .await
        .expect("Database connection failed");

    let service = AdminServer::new(AdminService::new(pool))
        .send_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Zstd);

    log::info!(
        "Started admin service on {}",
        config::SERVICE_ADDRESS.as_str()
    );

    Server::builder()
        .add_service(service)
        .serve(config::SERVICE_ADDRESS.parse()?)
        .await?;

    Ok(())
}
