use crate::utils::ServerError;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
#[serde(rename_all="camelCase")]
pub struct AppUserRequest {    
    #[validate(length(min = 1, max = 100, code = "too_long", message = "First Name is too long"))]
    #[serde(deserialize_with = "detrim::string_non_empty")]
    pub first_name: String,
    #[validate(length(min = 1, max = 100, code = "too_long", message = "Last Name is too long"))]
    #[serde(deserialize_with = "detrim::string_non_empty")]
    pub last_name: String,
    #[validate(length(min = 1, max = 100, code = "too_long", message = "User Name is too long"))]
    #[serde(deserialize_with = "detrim::string_non_empty")]
    pub username: String,
    #[validate(length(min = 1, max = 255, code = "too_long", message = "Email is too long"))]
    #[serde(deserialize_with = "detrim::string_non_empty")]
    pub email: String,
    #[validate(length(min = 1, max = 255, code = "too_long", message = "Password is too long"))]
    #[serde(deserialize_with = "detrim::string_non_empty")]
    pub password: String,
}

impl AppUserRequest {
    pub fn validate_request(&self) {
        match self.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(ServerError::ObjectValidationError(e.to_string())),
        }
        .unwrap()
    }
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct LoginAppUserRequest {
    #[validate(length(min = 1, max = 100, code = "too_long", message = "First Name is too long"))]
    pub username: String,
    #[validate(length(min = 1, max = 255, code = "too_long", message = "Password is too long"))]
    pub password: String,
}

impl LoginAppUserRequest {
    pub fn validate_request(&self) {
        match self.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(ServerError::ObjectValidationError(e.to_string())),
        }
        .unwrap()
    }
}
