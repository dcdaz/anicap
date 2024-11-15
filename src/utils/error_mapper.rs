use serde::Serialize;
use actix_web::{HttpResponse, error::ResponseError};
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ServerError {
    #[error("{0} Bad Request")]
    InsertFailure(String),
    #[error("{0} Not found")]
    ObjectNotFound(String),
    #[error("{0} Error Retrieving from DB")]
    ErrorRetrievingData(String),
    #[error("{0} TokenCreationError")]
    TokenCreationError(String),
    #[error("{0} BadQueryParameters")]
    BadQueryParameters(String),
}

#[derive(Serialize)]
struct ErrorMessage {
    cause: String,
    message: String
}

impl ErrorMessage {

    fn new(cause: &str, message: &str) -> Self {
        ErrorMessage {
            cause: cause.to_string(),
            message: message.to_string()
        }
    }
}


impl ResponseError for ServerError {

    fn error_response(&self) -> HttpResponse {
        match *self {
            ServerError::InsertFailure(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("InsertFailure", message)),
            ServerError::ObjectNotFound(ref message) => HttpResponse::NotFound()
                .json(ErrorMessage::new("ObjectNotFound", message)),
            ServerError::ErrorRetrievingData(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("ErrorRetrievingData", message)),
            ServerError::TokenCreationError(ref message) => HttpResponse::InternalServerError()
                .json(ErrorMessage::new("TokenCreationError", message)),
            ServerError::BadQueryParameters(ref message) => HttpResponse::BadRequest()
                .json(ErrorMessage::new("BadQueryParameters", message))
        }
    }
}
