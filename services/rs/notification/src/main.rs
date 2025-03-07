use env_logger::Env;
use notification::{config, service::NotificationService};
use protos::{
    client::client_client::ClientClient, coworking::coworking_client::CoworkingClient,
    mail::mail_client::MailClient, notification::notification_server::NotificationServer,
    reservation::reservation_client::ReservationClient,
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

    let client_client = ClientClient::connect(make_url(&config::CLIENT_SERVICE_ADDRESS))
        .await
        .expect("Failed to connect to the client service")
        .send_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Zstd);

    let coworking_client = CoworkingClient::connect(make_url(&config::COWORKING_SERVICE_ADDRESS))
        .await
        .expect("Failed to connect to the coworking service")
        .send_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Zstd);

    let reservation_client =
        ReservationClient::connect(make_url(&config::RESERVATION_SERVICE_ADDRESS))
            .await
            .expect("Failed to connect to the reservation service")
            .send_compressed(CompressionEncoding::Zstd)
            .accept_compressed(CompressionEncoding::Zstd);

    let mail_client = MailClient::connect(make_url(&config::MAIL_SERVICE_ADDRESS))
        .await
        .expect("Failed to connect to the mail service")
        .send_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Zstd);

    let service = NotificationServer::new(NotificationService::new(
        client_client,
        coworking_client,
        reservation_client,
        mail_client,
    ))
    .send_compressed(CompressionEncoding::Zstd)
    .accept_compressed(CompressionEncoding::Zstd);

    log::info!(
        "Started notification service on {}",
        config::SERVICE_ADDRESS.as_str()
    );

    Server::builder()
        .add_service(service)
        .serve(config::SERVICE_ADDRESS.parse()?)
        .await?;

    Ok(())
}
