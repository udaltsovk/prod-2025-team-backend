use chrono::{DateTime, TimeDelta, Timelike, Utc};
use convertions::{datetime_into_timestamp, timestamp_into_datetime};
use protos::reservation::{
    CreateRequest, ReservationResponse, ReservationsResponse, UpdateRequest,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Validate, ToSchema, Debug)]
#[validate(schema(function = "Self::validate_custom"))]
pub struct CreateReservation {
    pub seat_id: Uuid,

    #[schema(format = DateTime)]
    pub starts_at: DateTime<Utc>,

    #[schema(format = DateTime)]
    pub ends_at: DateTime<Utc>,
}
impl CreateReservation {
    fn validate_custom(&self) -> Result<(), ValidationError> {
        match (self.starts_at, self.ends_at) {
            (starts, ..) if starts.hour() < 8 || starts.hour() >= 21 => Err(ValidationError::new(
                "`starts_at` hour must be between 8 and 21",
            )),
            (.., ends) if ends.hour() < 8 || ends.hour() >= 21 => Err(ValidationError::new(
                "`ends_at` hour must be between 8 and 21",
            )),
            (starts, ends) if ends - starts < TimeDelta::minutes(15) => Err(ValidationError::new(
                "`starts_at` must be earlier than `ends_at` by at least 15 minutes",
            )),
            (starts, ends) if ends - starts >= TimeDelta::hours(13) => Err(ValidationError::new(
                "`starts_at` must be earlier than `ends_at` by at most 13 hours",
            )),
            _ => Ok(()),
        }
    }

    pub fn into_proto(self, client_id: Uuid, is_admin: bool) -> CreateRequest {
        CreateRequest {
            client_id: client_id.to_string(),
            is_admin,
            seat_id: self.seat_id.to_string(),
            starts_at: datetime_into_timestamp(self.starts_at),
            ends_at: datetime_into_timestamp(self.ends_at),
        }
    }
}
impl From<ReservationResponse> for CreateReservation {
    fn from(resp: ReservationResponse) -> Self {
        Self {
            seat_id: Uuid::parse_str(&resp.client_id.to_string()).unwrap(),
            starts_at: timestamp_into_datetime(resp.starts_at).unwrap(),
            ends_at: timestamp_into_datetime(resp.ends_at).unwrap(),
        }
    }
}

#[derive(Serialize, ToSchema, Debug)]
pub struct Reservation {
    pub id: Uuid,
    pub client_id: Uuid,

    #[serde(flatten)]
    pub inner: CreateReservation,

    #[schema(default = false, examples(false, true))]
    pub cancelled: bool,
}
impl From<ReservationResponse> for Reservation {
    fn from(resp: ReservationResponse) -> Self {
        Self {
            id: Uuid::parse_str(&resp.id).unwrap(),
            client_id: Uuid::parse_str(&resp.id).unwrap(),
            inner: resp.clone().into(),
            cancelled: resp.is_canceled,
        }
    }
}
impl Reservation {
    pub fn vec_from_proto(resp: ReservationsResponse) -> Vec<Self> {
        resp.reservations
            .iter()
            .map(|r| Self::from(r.clone()))
            .collect()
    }
}

#[derive(Deserialize, Validate, ToSchema, Debug)]
#[validate(schema(function = "Self::validate_custom"))]
pub struct ReservationUpdate {
    pub seat_id: Option<Uuid>,

    #[schema(format = DateTime)]
    pub starts_at: Option<DateTime<Utc>>,

    #[schema(format = DateTime)]
    pub ends_at: Option<DateTime<Utc>>,
}
impl ReservationUpdate {
    fn validate_custom(&self) -> Result<(), ValidationError> {
        match (self.starts_at, self.ends_at) {
            (Some(starts), ..) if starts.hour() < 8 || starts.hour() >= 21 => Err(
                ValidationError::new("`starts_at` hour must be between 8 and 21"),
            ),
            (.., Some(ends)) if ends.hour() < 8 || ends.hour() >= 21 => Err(ValidationError::new(
                "`ends_at` hour must be between 8 and 21",
            )),
            (Some(starts), Some(ends)) if ends - starts < TimeDelta::minutes(15) => {
                Err(ValidationError::new(
                    "`starts_at` must be earlier than `ends_at` at least by 15 minutes",
                ))
            }
            (Some(starts), Some(ends)) if ends - starts >= TimeDelta::hours(13) => {
                Err(ValidationError::new(
                    "`starts_at` must be earlier than `ends_at` by at most 13 hours",
                ))
            }
            _ => Ok(()),
        }
    }
    pub fn into_proto(
        self,
        reservation_id: Uuid,
        entity_id: Uuid,
        is_admin: bool,
    ) -> UpdateRequest {
        UpdateRequest {
            id: reservation_id.to_string(),
            client_id: entity_id.to_string(),
            is_admin,
            seat_id: self.seat_id.map(|i| i.to_string()),
            starts_at: self.starts_at.map(datetime_into_timestamp),
            ends_at: self.ends_at.map(datetime_into_timestamp),
            is_canceled: None,
            is_visited: None,
        }
    }
}
