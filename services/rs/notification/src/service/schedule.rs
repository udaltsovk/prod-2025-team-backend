use std::time::{Duration, SystemTime};

use super::error::ServiceError;
use crate::config;
use chrono::FixedOffset;
use convertions::timestamp_into_datetime;
use protos::{
    client::{client_client::ClientClient, ClientMeta, ClientRequest, ClientResponse},
    coworking::{
        coworking_client::CoworkingClient, CoworkingResponse, GetCoworkingByIdRequest, SeatRequest,
        SeatResponse,
    },
    mail::{mail_client::MailClient, SendRequest},
    notification::ScheduleRequest,
    reservation::{reservation_client::ReservationClient, GetByIdRequest, ReservationResponse},
};
use tokio::time::Instant;
use tonic::transport::Channel;

pub async fn handle(
    req: ScheduleRequest,
    client_client: &ClientClient<Channel>,
    coworking_client: &CoworkingClient<Channel>,
    reservation_client: &ReservationClient<Channel>,
    mail_client: &MailClient<Channel>,
) -> Result<(), ServiceError> {
    let client_client = client_client.clone();
    let coworking_client = coworking_client.clone();
    let reservation_client = reservation_client.clone();
    let mail_client = mail_client.clone();

    let target_time = SystemTime::UNIX_EPOCH + Duration::from_secs(req.send_at.seconds as u64);
    let sleep_duration = target_time
        .duration_since(SystemTime::now())
        .unwrap_or(Duration::ZERO);

    tokio::spawn(async move {
        tokio::time::sleep_until(Instant::now() + sleep_duration).await;
        task(
            req,
            client_client,
            coworking_client,
            reservation_client,
            mail_client,
        )
        .await
    })
    .await??;

    Ok(())
}

async fn task(
    req: ScheduleRequest,
    mut client_client: ClientClient<Channel>,
    mut coworking_client: CoworkingClient<Channel>,
    mut reservation_client: ReservationClient<Channel>,
    mut mail_client: MailClient<Channel>,
) -> Result<(), ServiceError> {
    let ClientResponse {
        id: client_id,
        meta: ClientMeta { name, email, .. },
        ..
    } = client_client
        .get(ClientRequest { id: req.id.clone() })
        .await?
        .into_inner();

    let CoworkingResponse {
        name: coworking_name,
        address: coworking_address,
        ..
    } = coworking_client
        .get_by_id(GetCoworkingByIdRequest {
            id: config::COWORKING_ID.to_string(),
        })
        .await?
        .into_inner();

    let ReservationResponse {
        starts_at,
        ends_at,
        seat_id,
        ..
    } = reservation_client
        .get_by_id(GetByIdRequest {
            id: req.id,
            client_id,
            is_admin: false,
        })
        .await?
        .into_inner();

    let timezone = FixedOffset::west_opt(60 * 60 * 3).unwrap();
    let start_time = timestamp_into_datetime(starts_at)
        .unwrap()
        .with_timezone(&timezone)
        .naive_local();
    let end_time = timestamp_into_datetime(ends_at)
        .unwrap()
        .with_timezone(&timezone)
        .naive_local();

    let SeatResponse {
        number,
        r#type,
        cost,
        features,
        ..
    } = coworking_client
        .get_seat(SeatRequest { id: seat_id })
        .await?
        .into_inner();

    mail_client
        .send(SendRequest {
            to: vec![email],
            subject: Some(format!(
                "Напоминание: Ваша бронь места в коворкинге через 3 часа"
            )),
            body: Some(format!(
                r#"Здравствуйте, {name}!

Напоминаем, что ваша бронь коворкинга начнется через 3 часа:

📍 Локация: {coworking_name}
📌 Адрес: {coworking_address}
🕒 Время: {}, {} – {}

💺 Ваше место:
Номер: {number}
Тип: {type}
Особенности: {}
Стоимость: {cost}

До встречи!
{coworking_name}"#,
                start_time.date(),
                start_time.time(),
                end_time.time(),
                features.join(", ")
            )),
        })
        .await?;
    Ok(())
}
