use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::utils::ServerError;

#[derive(Validate, Deserialize, ToSchema)]
#[serde(default)]
pub struct SerieTypeRequest {
    #[validate(length(min = 1, max = 255, code = "too_long", message = "Genre name is too long"))]
    #[serde(deserialize_with = "detrim::string_non_empty")]
    pub name: String,
}

impl Default for SerieTypeRequest {
    fn default() -> Self {
        SerieTypeRequest { 
            name: String::from(""),
        }
    }
}

impl SerieTypeRequest {
    pub fn validate_request(&self) {
        match self.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(ServerError::ObjectValidationError(e.to_string())),
        }
        .unwrap()
    }
}
