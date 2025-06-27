use utoipa::ToSchema;
use serde::Serialize;

#[derive(Serialize, ToSchema)]
pub struct ErrorMessage {
    pub cause: String,
    pub message: String,
}

impl ErrorMessage {
    pub fn new(cause: &str, message: &str) -> Self {
        ErrorMessage {
            cause: cause.to_string(),
            message: message.to_string(),
        }
    }
}
