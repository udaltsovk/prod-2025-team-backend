use protos::admin::{AdminResponse, EditRequest, RegisterRequest};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use super::Credentials;

#[derive(Deserialize, Serialize, Validate, ToSchema, Debug)]
pub struct AdminForm {
    #[serde(flatten)]
    #[validate(nested)]
    pub credentials: Credentials,
}
impl Into<RegisterRequest> for AdminForm {
    fn into(self) -> RegisterRequest {
        RegisterRequest {
            email: self.credentials.email,
            password: self.credentials.password,
        }
    }
}

#[derive(Serialize, ToSchema, Clone, Debug)]
pub struct Admin {
    pub id: Uuid,

    #[schema(format = Email)]
    pub email: String,
}
impl From<AdminResponse> for Admin {
    fn from(resp: AdminResponse) -> Self {
        Self {
            id: Uuid::parse_str(&resp.id).unwrap(),
            email: resp.email,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema, Debug)]
pub struct AdminUpdate {
    #[validate(email, length(min = 6, max = 50))]
    #[schema(format = Email, min_length = 6, max_length = 50)]
    pub email: Option<String>,
}
impl AdminUpdate {
    pub fn into_proto(self, id: Uuid) -> EditRequest {
        EditRequest {
            id: id.to_string(),
            email: self.email,
        }
    }
}
