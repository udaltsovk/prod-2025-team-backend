use protos::coworking::{CreateSeatRequest, SeatResponse, SeatsResponse, UpdateSeatRequest};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Display, ToSchema, Clone, Debug)]
pub enum SeatType {
    OpenSpace,
    Cabin(u64),
    Room(u64),
}
impl SeatType {
    fn validate_custom(&self) -> Result<(), ValidationError> {
        match self {
            Self::Cabin(capacity) if *capacity < 1 || 2 < *capacity => Err(ValidationError::new(
                "field value `capacity` for seat type `cabin` must be between 1 and 2",
            )),
            Self::Room(capacity) if *capacity < 4 || 12 < *capacity => Err(ValidationError::new(
                "field value `capacity` for seat type `room` must be between 4 and 12",
            )),
            _ => Ok(()),
        }
    }

    fn from_resp(resp: &SeatResponse) -> Option<Self> {
        match resp.r#type.as_str() {
            "OpenSpace" => Some(Self::OpenSpace),
            "Cabin" => Some(Self::Cabin(resp.capacity)),
            "Room" => Some(Self::Room(resp.capacity)),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize, Display, ToSchema, Debug)]
pub enum SeatFeature {
    Monitor,
    Laptop,
}
impl SeatFeature {
    pub fn from_resp(feature: &str) -> Option<Self> {
        match feature {
            "Monitor" => Some(Self::Monitor),
            "Laptop" => Some(Self::Laptop),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize, Validate, ToSchema, Debug)]
pub struct CreateSeat {
    #[validate(custom(function = "SeatType::validate_custom"))]
    pub r#type: SeatType,

    #[validate(range(min = 1, max = 1000))]
    #[schema(format = UInt64, minimum = 1, maximum = 1000, examples(1))]
    pub number: u64,

    #[schema(min_items = 0, max_items = 24)]
    pub features: Vec<SeatFeature>,

    #[validate(range(exclusive_min = 0.0, max = 1_000_000.0))]
    #[schema(format = Double, exclusive_minimum = 0.0, maximum = 1_000_000.0, examples(100.0))]
    pub cost: f64,
}
impl Into<CreateSeatRequest> for CreateSeat {
    fn into(self) -> CreateSeatRequest {
        CreateSeatRequest {
            r#type: self.r#type.to_string(),
            number: self.number,
            capacity: match self.r#type {
                SeatType::OpenSpace => 1,
                SeatType::Room(capacity) | SeatType::Cabin(capacity) => capacity,
            },
            features: self.features.iter().map(|f| f.to_string()).collect(),
            cost: self.cost,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct Seat {
    pub id: Uuid,

    #[serde(flatten)]
    pub inner: CreateSeat,
}
impl From<SeatResponse> for Seat {
    fn from(resp: SeatResponse) -> Self {
        Self {
            id: Uuid::parse_str(&resp.id).unwrap(),
            inner: CreateSeat {
                r#type: SeatType::from_resp(&resp).unwrap(),
                number: resp.number,
                features: resp
                    .features
                    .iter()
                    .map(|f| SeatFeature::from_resp(&f.clone()).unwrap())
                    .collect(),
                cost: resp.cost,
            },
        }
    }
}
impl Seat {
    pub fn vec_from_proto(resp: SeatsResponse) -> Vec<Self> {
        resp.seats.iter().map(|s| Seat::from(s.clone())).collect()
    }
}

#[derive(Deserialize, Serialize, Validate, ToSchema, Debug)]
pub struct SeatUpdate {
    #[validate(custom(function = "SeatType::validate_custom"))]
    pub r#type: Option<SeatType>,

    #[validate(range(min = 1, max = 1000))]
    #[schema(format = UInt64, minimum = 1, maximum = 1000, examples(1))]
    pub number: Option<u64>,

    pub features: Vec<SeatFeature>,

    #[validate(range(exclusive_min = 0.0, max = 1_000_000.0))]
    #[schema(format = Double, exclusive_minimum = 0.0, maximum = 1_000_000.0, examples(100.0))]
    pub cost: Option<f64>,
}
impl SeatUpdate {
    pub fn into_proto(self, id: Uuid) -> UpdateSeatRequest {
        UpdateSeatRequest {
            id: id.to_string(),
            r#type: self.r#type.clone().map(|t| t.to_string()),
            capacity: self.r#type.map(|t| match t {
                SeatType::OpenSpace => 1,
                SeatType::Cabin(capacity) | SeatType::Room(capacity) => capacity,
            }),
            number: self.number,
            features: self.features.iter().map(|f| f.to_string()).collect(),
            cost: self.cost,
        }
    }
}
