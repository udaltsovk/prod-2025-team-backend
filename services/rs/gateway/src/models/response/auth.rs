use protos::admin::AuthResponse as AdminAuthResponseProto;
use protos::client::AuthResponse as ClientAuthResponseProto;
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::dto::{Admin, Client};

#[derive(Serialize, ToSchema, Debug)]
pub struct ClientAuthResponse {
    pub token: String,
    pub client: Client,
}
impl From<ClientAuthResponseProto> for ClientAuthResponse {
    fn from(resp: ClientAuthResponseProto) -> Self {
        Self {
            token: resp.token,
            client: resp.client.into(),
        }
    }
}

#[derive(Serialize, ToSchema, Debug)]
pub struct AdminAuthResponse {
    pub token: String,
    pub admin: Admin,
}
impl From<AdminAuthResponseProto> for AdminAuthResponse {
    fn from(resp: AdminAuthResponseProto) -> Self {
        Self {
            token: resp.token,
            admin: resp.admin.into(),
        }
    }
}
