use client::{config, service::ClientService};
use env_logger::Env;
use protos::{
    client::client_server::ClientServer, reservation::reservation_client::ReservationClient,
};
use service_helper::make_url;
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

    let reservation_client =
        ReservationClient::connect(make_url(&config::RESERVATION_SERVICE_ADDRESS))
            .await
            .expect("Failed to connect to the reservation service")
            .send_compressed(CompressionEncoding::Zstd)
            .accept_compressed(CompressionEncoding::Zstd);

    let service = ClientServer::new(ClientService::new(pool, reservation_client))
        .send_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Zstd);

    log::info!(
        "Started client service on {}",
        config::SERVICE_ADDRESS.as_str()
    );

    Server::builder()
        .add_service(service)
        .serve(config::SERVICE_ADDRESS.parse()?)
        .await?;

    Ok(())
}
