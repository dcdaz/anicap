use crate::utils::ServerError;
use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
#[serde(default)]
pub struct SerieRequest {
    #[validate(length(min = 1, max = 255, code = "too_long", message = "Name is too long"))]
    pub name: String,
    #[validate(range(min = 0, code = "invalid_value", message = "Season must be 0 or greater"))]
    pub season: i16,
    #[validate(range(min = 0, code = "invalid_value", message = "Chapter must be 0 or greater"))]
    pub chapter: i16,
    #[validate(range(min = 0.0, max = 10.0, code = "out_of_range", message = "Score must be between 0.0 and 10.0"))]
    pub score: f32,
}

impl Default for SerieRequest {
    fn default() -> Self {
        SerieRequest {
            name: String::from(""),
            season: 0,
            chapter: 0,
            score: 0.0,
        }
    }
}

impl SerieRequest {
    pub fn validate_request(&self) {
        match self.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(ServerError::ObjectValidationError(e.to_string())),
        }
        .unwrap()
    }
}
