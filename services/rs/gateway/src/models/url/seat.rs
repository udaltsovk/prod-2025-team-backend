use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct SeatPath {
    pub seat_id: Uuid,
}
