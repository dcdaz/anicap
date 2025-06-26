use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ServerError {
    #[error("{0} Bad Request")]
    InsertFailure(String),
    #[error("{0} Validation Error")]
    ObjectValidationError(String),
    #[error("{0} Not found")]
    ObjectNotFound(String),
    #[error("{0} Token Creation Error")]
    TokenCreationError(String),
}

#[derive(Serialize)]
struct ErrorMessage {
    cause: String,
    message: String,
}

impl ErrorMessage {
    fn new(cause: &str, message: &str) -> Self {
        ErrorMessage {
            cause: cause.to_string(),
            message: message.to_string(),
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServerError::InsertFailure(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("Insert Failure", message)),
            ServerError::ObjectValidationError(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("Object Validation Error", message)),
            ServerError::ObjectNotFound(ref message) => HttpResponse::NotFound()
                .json(ErrorMessage::new("Object Not Found", message)),
            ServerError::TokenCreationError(ref message) => HttpResponse::InternalServerError()
                .json(ErrorMessage::new("Token Creation Error", message)),
        }
    }
}
