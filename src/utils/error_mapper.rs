use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;
use thiserror::Error;

use crate::controllers::ErrorMessage;

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
    #[error("{0} Token Expired Error")]
    TokenExpiredError(String),
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
            ServerError::TokenExpiredError(ref message) => {
                HttpResponse::Unauthorized().json(ErrorMessage::new("Token Expired Error", message))
            }
        }
    }
}
