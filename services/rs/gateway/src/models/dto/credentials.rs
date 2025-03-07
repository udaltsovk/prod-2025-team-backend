use crate::utils::validation::validate_password;
use protos::admin::LoginRequest as AdminLoginRequest;
use protos::client::LoginRequest as ClientLoginRequest;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, ToSchema, Debug)]
pub struct Credentials {
    #[validate(length(min = 0, max = 50))]
    #[schema(format = Email, min_length = 6, max_length = 50)]
    pub email: String,

    #[validate(length(min = 8, max = 100), custom(function = "validate_password"))]
    #[schema(format = Password, min_length = 8, max_length = 100)]
    pub password: String,
}
impl Into<AdminLoginRequest> for Credentials {
    fn into(self) -> AdminLoginRequest {
        AdminLoginRequest {
            email: self.email,
            password: self.password,
        }
    }
}
impl Into<ClientLoginRequest> for Credentials {
    fn into(self) -> ClientLoginRequest {
        ClientLoginRequest {
            email: self.email,
            password: self.password,
        }
    }
}
