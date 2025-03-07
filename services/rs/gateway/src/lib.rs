use actix_web::web::{get, Data, JsonConfig, PathConfig};
use env_config::environment_variables;
use log::info;
use protos::{
    admin::admin_client::AdminClient, client::client_client::ClientClient,
    coworking::coworking_client::CoworkingClient, image::image_client::ImageClient,
    mail::mail_client::MailClient, notification::notification_client::NotificationClient,
    reservation::reservation_client::ReservationClient,
    seat_lock::seat_lock_client::SeatLockClient,
};
use service_helper::make_url;
use tonic::{codec::CompressionEncoding, transport::Channel};
use utils::openapi::Swagger;
use utoipa::openapi::OpenApi as OpenApiStruct;
use utoipa::OpenApi;
use utoipa_actix_web::service_config::ServiceConfig;
use uuid::Uuid;

use crate::routes::{not_found, ApiError};

pub mod auth;
pub mod models;
pub mod routes;
pub mod utils;

environment_variables! {
    SERVER_ADDRESS: String = "0.0.0.0:8080",
    COWORKING_ID: String = "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    // сервисы
    ADMIN_SERVICE_ADDRESS: String = "localhost:50051",
    CLIENT_SERVICE_ADDRESS: String = "localhost:50052",
    COWORKING_SERVICE_ADDRESS: String = "localhost:50053",
    RESERVATION_SERVICE_ADDRESS: String = "localhost:50054",
    SEAT_LOCK_SERVICE_ADDRESS: String = "localhost:50055",
    // доп. фичи
    MAIL_SERVICE_ADDRESS: String = "localhost:50056",
    NOTIFICATION_SERVICE_ADDRESS: String = "localhost:50057",
    IMAGE_SERVICE_ADDRESS: String = "localhost:50058"
}

#[derive(Clone)]
pub struct SolutionConfig {
    pub openapi: OpenApiStruct,
    pub coworking_id: Uuid,
    pub admin: AdminClient<Channel>,
    pub client: ClientClient<Channel>,
    pub coworking: CoworkingClient<Channel>,
    pub reservation: ReservationClient<Channel>,
    // pub seat_lock: SeatLockClient<Channel>,
    // pub mail: MailClient<Channel>,
    pub notification: NotificationClient<Channel>,
    // pub image: ImageClient<Channel>,
}

pub async fn app_setup() -> SolutionConfig {
    config::init();
    info!("Starting gateway on {}", config::SERVER_ADDRESS.as_str());

    let admin_client = AdminClient::connect(make_url(&config::ADMIN_SERVICE_ADDRESS))
        .await
        .expect("Failed to connect to the admin service")
        .send_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Zstd);

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

    let seat_lock_client = SeatLockClient::connect(make_url(&config::SEAT_LOCK_SERVICE_ADDRESS))
        .await
        .expect("Failed to connect to the seat lock service")
        .send_compressed(CompressionEncoding::Zstd)
        .accept_compressed(CompressionEncoding::Zstd);

    // let mail_client = MailClient::connect(make_url(&config::MAIL_SERVICE_ADDRESS))
    //     .await
    //     .expect("Failed to connect to the mail service")
    //     .send_compressed(CompressionEncoding::Zstd)
    //     .accept_compressed(CompressionEncoding::Zstd);

    let notification_client =
        NotificationClient::connect(make_url(&config::NOTIFICATION_SERVICE_ADDRESS))
            .await
            .expect("Failed to connect to the notification service")
            .send_compressed(CompressionEncoding::Zstd)
            .accept_compressed(CompressionEncoding::Zstd);

    // let image_client = ImageClient::connect(make_url(&config::IMAGE_SERVICE_ADDRESS))
    //     .await
    //     .expect("Failed to connect to the image service")
    //     .send_compressed(CompressionEncoding::Zstd)
    //     .accept_compressed(CompressionEncoding::Zstd);

    SolutionConfig {
        openapi: Swagger::openapi(),
        coworking_id: Uuid::parse_str(config::COWORKING_ID.as_str()).unwrap(),
        admin: admin_client,
        client: client_client,
        coworking: coworking_client,
        reservation: reservation_client,
        // seat_lock: seat_lock_client,
        // mail: mail_client,
        notification: notification_client,
        // image: image_client,
    }
}

impl SolutionConfig {
    pub fn build(self) -> impl FnOnce(&mut ServiceConfig) {
        move |cfg: &mut ServiceConfig| {
            cfg.app_data(
                PathConfig::default()
                    .error_handler(|err, _req| ApiError::Validation(err.to_string()).into()),
            )
            .app_data(
                JsonConfig::default()
                    .error_handler(|err, _req| ApiError::Validation(err.to_string()).into()),
            )
            .app_data(Data::new(self.coworking_id))
            .app_data(Data::new(self.admin.clone()))
            .app_data(Data::new(self.client.clone()))
            .app_data(Data::new(self.coworking.clone()))
            .app_data(Data::new(self.reservation.clone()))
            // .app_data(Data::new(self.seat_lock.clone()))
            // .app_data(Data::new(self.mail.clone()))
            .app_data(Data::new(self.notification.clone()))
            // .app_data(Data::new(self.image.clone()))
            .configure(routes::config)
            .default_service(get().to(not_found));
        }
    }
}
