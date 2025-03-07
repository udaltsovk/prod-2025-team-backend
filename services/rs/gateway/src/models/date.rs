use serde::Deserialize;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

use crate::utils::is_leap_year;

#[derive(Deserialize, Validate, ToSchema, Debug)]
#[validate(schema(function = "Self::validate_custom"))]
pub struct Date {
    #[validate(range(min = 1, max = 31))]
    #[schema(format = UInt32, minimum = 1, maximum = 31, example = 1)]
    pub day: u32,

    #[validate(range(min = 1, max = 12))]
    #[schema(format = UInt32, minimum = 1, maximum = 12, example = 1)]
    pub month: u32,

    #[validate(range(min = 2025))]
    #[schema(format = UInt64, minimum = 2025, example = 2025)]
    pub year: u64,
}
impl Date {
    fn validate_custom(&self) -> Result<(), ValidationError> {
        match (self.year, self.month, self.day) {
            (year, month, day) if is_leap_year(year) && month == 2 && day > 29 => {
                Err(ValidationError::new(
                    "`day` field must be between 1 and 29 for a leap year february",
                ))
            }
            (.., month, day) if month == 2 && day > 28 => Err(ValidationError::new(
                "`day` field must be between 1 and 28 for february",
            )),
            (.., month, day)
                if month == 4 || month == 6 || month == 9 || month == 11 && day > 30 =>
            {
                Err(ValidationError::new(
                    "`day` field must be between 1 and 30 for april | june | september | november",
                ))
            }

            _ => Ok(()),
        }
    }
}
