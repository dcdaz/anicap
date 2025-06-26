use actix_web::HttpResponse;

use crate::utils::ServerError;

pub type ServerResponse = Result<HttpResponse, ServerError>;
