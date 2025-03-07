use protos::client::{ClientMeta, ClientResponse, EditRequest, RegisterRequest};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use super::Credentials;

#[derive(Deserialize, Serialize, Validate, ToSchema, Clone, Debug)]
pub struct ClientDisplay {
    #[validate(length(min = 1, max = 15))]
    #[schema(min_length = 1, max_length = 15)]
    pub name: String,

    #[validate(length(min = 1, max = 15))]
    #[schema(min_length = 1, max_length = 15)]
    pub surname: String,

    #[validate(length(min = 0, max = 20))]
    #[schema(min_length = 0, max_length = 20)]
    pub patronymic: String,
}
impl ClientDisplay {
    pub fn into_meta(self, email: &str, send_notifications: bool) -> ClientMeta {
        ClientMeta {
            name: self.name,
            surname: self.surname,
            patronymic: self.patronymic,
            email: email.to_string(),
            is_internal: false,
            send_notifications,
        }
    }
}
impl From<ClientMeta> for ClientDisplay {
    fn from(meta: ClientMeta) -> Self {
        Self {
            name: meta.name,
            surname: meta.surname,
            patronymic: meta.patronymic,
        }
    }
}

#[derive(Deserialize, Serialize, Validate, ToSchema, Debug)]
pub struct ClientForm {
    #[serde(flatten)]
    #[validate(nested)]
    pub display: ClientDisplay,

    #[serde(flatten)]
    #[validate(nested)]
    pub credentials: Credentials,

    #[schema(default = false, examples(false, true))]
    pub send_notifications: bool,
}
impl Into<RegisterRequest> for ClientForm {
    fn into(self) -> RegisterRequest {
        RegisterRequest {
            meta: self
                .display
                .into_meta(&self.credentials.email, self.send_notifications),
            password: self.credentials.password,
        }
    }
}

#[derive(Serialize, ToSchema, Clone, Debug)]
pub struct Client {
    pub id: Uuid,

    #[serde(flatten)]
    pub display: ClientDisplay,

    #[schema(format = Email)]
    pub email: String,

    #[schema(default = true, examples(false, true))]
    pub send_notifications: bool,

    #[schema(default = false, examples(false, true))]
    pub is_internal: bool,

    pub verified: bool,
}
impl From<ClientResponse> for Client {
    fn from(resp: ClientResponse) -> Self {
        Self {
            id: Uuid::parse_str(&resp.id).unwrap(),
            display: resp.meta.clone().into(),
            email: resp.meta.email,
            send_notifications: resp.meta.send_notifications,
            is_internal: resp.meta.is_internal,
            verified: resp.verified,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema, Debug)]
pub struct ClientUpdate {
    #[validate(length(min = 1, max = 15))]
    #[schema(min_length = 1, max_length = 15)]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 15))]
    #[schema(min_length = 1, max_length = 15)]
    pub surname: Option<String>,

    #[validate(length(min = 0, max = 20))]
    #[schema(min_length = 0, max_length = 20)]
    pub patronymic: Option<String>,

    #[validate(email, length(min = 6, max = 50))]
    #[schema(format = Email, min_length = 6, max_length = 50)]
    pub email: Option<String>,

    #[schema(default = true, examples(false, true))]
    pub send_notifications: Option<bool>,
}
impl ClientUpdate {
    pub fn into_proto(self, id: Uuid) -> EditRequest {
        EditRequest {
            id: id.to_string(),
            name: self.name,
            surname: self.surname,
            patronymic: self.patronymic,
            email: self.email,
            send_notifications: self.send_notifications,
            is_internal: None,
            verified: None,
        }
    }
}
