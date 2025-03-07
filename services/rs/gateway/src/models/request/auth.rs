use crate::utils::validation::validate_password;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema, Debug)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 8, max = 100), custom(function = "validate_password"))]
    #[schema(format = Password, min_length = 8, max_length = 100)]
    pub current_password: String,

    #[validate(length(min = 8, max = 100), custom(function = "validate_password"))]
    #[schema(format = Password, min_length = 8, max_length = 100)]
    pub new_password: String,

    #[validate(
        length(min = 8, max = 100),
        must_match(other = "new_password"),
        custom(function = "validate_password")
    )]
    #[schema(format = Password, min_length = 8, max_length = 100)]
    pub new_password_confirm: String,
}
