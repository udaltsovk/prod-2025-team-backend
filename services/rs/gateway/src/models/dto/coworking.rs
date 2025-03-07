use protos::coworking::{CoworkingResponse, UpdateCoworkingRequest};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, ToSchema, Debug)]
pub struct CreateCoworking {
    #[validate(length(min = 1, max = 50))]
    pub name: String,

    #[validate(length(min = 1, max = 100))]
    pub address: String,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct CoworkingSeatStats {
    #[schema(format = UInt64, minimum = 0, examples(0))]
    pub available: u64,

    #[schema(format = UInt64, minimum = 0, examples(0))]
    pub total: u64,
}
impl From<CoworkingResponse> for CoworkingSeatStats {
    fn from(resp: CoworkingResponse) -> Self {
        Self {
            available: resp.available_seats,
            total: resp.total_seats,
        }
    }
}

#[derive(Serialize, ToSchema, Debug)]
pub struct Coworking {
    pub id: Uuid,

    #[serde(flatten)]
    pub inner: CreateCoworking,

    pub seats: CoworkingSeatStats,
}
impl From<CoworkingResponse> for Coworking {
    fn from(resp: CoworkingResponse) -> Self {
        Self {
            id: Uuid::parse_str(&resp.id.to_string()).unwrap(),
            inner: CreateCoworking {
                name: resp.name.clone(),
                address: resp.address.clone(),
            },
            seats: resp.into(),
        }
    }
}

#[derive(Deserialize, Validate, ToSchema, Debug)]
pub struct CoworkingUpdate {
    #[validate(length(min = 1, max = 50))]
    #[schema(min_length = 1, max_length = 50)]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 100))]
    #[schema(min_length = 1, max_length = 100)]
    pub address: Option<String>,
}
impl CoworkingUpdate {
    pub fn into_proto(self, id: Uuid) -> UpdateCoworkingRequest {
        UpdateCoworkingRequest {
            id: id.to_string(),
            name: self.name,
            address: self.address,
        }
    }
}
