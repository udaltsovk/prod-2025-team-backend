use error::ServiceError;
use protos::{
    client::client_client::ClientClient,
    coworking::coworking_client::CoworkingClient,
    mail::mail_client::MailClient,
    notification::{notification_server::Notification, ScheduleRequest},
    reservation::reservation_client::ReservationClient,
};
use service_helper::response::ServiceResult;
use tonic::{async_trait, transport::Channel, Request, Response};

mod error;
mod schedule;

pub struct NotificationService {
    pub client_client: ClientClient<Channel>,
    pub coworking_client: CoworkingClient<Channel>,
    pub reservation_client: ReservationClient<Channel>,
    pub mail_client: MailClient<Channel>,
}
impl NotificationService {
    pub fn new(
        client_client: ClientClient<Channel>,
        coworking_client: CoworkingClient<Channel>,
        reservation_client: ReservationClient<Channel>,
        mail_client: MailClient<Channel>,
    ) -> Self {
        Self {
            client_client,
            coworking_client,
            reservation_client,
            mail_client,
        }
    }
}

#[async_trait]
impl Notification for NotificationService {
    async fn schedule(&self, request: Request<ScheduleRequest>) -> ServiceResult<()> {
        schedule::handle(
            request.into_inner(),
            &self.client_client,
            &self.coworking_client,
            &self.reservation_client,
            &self.mail_client,
        )
        .await
        .map(Response::new)
        .map_err(ServiceError::into)
    }
}
