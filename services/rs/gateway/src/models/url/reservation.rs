use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct ReservationPath {
    pub reservation_id: Uuid,
}
