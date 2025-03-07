use passwords::{analyzer::analyze, scorer::score};
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

use crate::routes::ApiError;

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if score(&analyze(password)) < 57.0 {
        Err(ValidationError::new("Password is too weak"))
    } else {
        Ok(())
    }
}

pub fn validation_errors_to_err(errors: ValidationErrors) -> ApiError {
    ApiError::InvalidInput(validation_errors_to_string(errors, None))
}

pub fn validation_errors_to_string(errors: ValidationErrors, adder: Option<String>) -> String {
    let mut output = String::new();

    let map = errors.into_errors();

    let key_option = map.keys().next();

    if let Some(field) = key_option {
        if let Some(error) = map.get(field) {
            return match error {
                ValidationErrorsKind::Struct(errors) => {
                    validation_errors_to_string(*errors.clone(), Some(format!("of item {field}")))
                }
                ValidationErrorsKind::List(list) => {
                    if let Some((index, errors)) = list.iter().next() {
                        output.push_str(&validation_errors_to_string(
                            *errors.clone(),
                            Some(format!("of list {field} with index {index}")),
                        ));
                    }

                    output
                }
                ValidationErrorsKind::Field(errors) => {
                    if let Some(error) = errors.first() {
                        if let Some(adder) = adder {
                            output.push_str(&format!(
                                "Field {} {} failed validation with error: {}",
                                field, adder, error.code
                            ));
                        } else {
                            output.push_str(&format!(
                                "Field {} failed validation with error: {}",
                                field, error.code
                            ));
                        }
                    }

                    output
                }
            };
        }
    }

    String::new()
}
