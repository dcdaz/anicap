use actix_web::{
    dev, error::ErrorUnauthorized, http::header::HeaderValue, web, Error, FromRequest, HttpRequest,
};
use futures::future::{err, ok, Ready};

use crate::model::Claims;
use crate::utils::{pool_handler, SqlPool, SqlPooledConnection};

pub struct AuthenticatedRequest {
    pub user_id: i16,
    pub connection: SqlPooledConnection,
}

impl FromRequest for AuthenticatedRequest {
    type Error = Error;
    type Future = Ready<Result<AuthenticatedRequest, Error>>;

    fn from_request(http_request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth = http_request.headers().get("Authorization");
        match auth {
            Some(_) => {
                let token = get_token_from_auth_header(auth);
                let pool_from_app_data = http_request.app_data::<web::Data<SqlPool>>();
                let connection = pool_handler(pool_from_app_data);
                if Claims::is_valid_token(token) {
                    let decoded_token = Claims::decode_token(token);
                    match decoded_token {
                        Ok(_) => ok(AuthenticatedRequest {
                            user_id: decoded_token.unwrap().claims.id,
                            connection: connection.unwrap(),
                        }),
                        Err(_) => err(ErrorUnauthorized("Invalid or Expired token")),
                    }
                } else {
                    err(ErrorUnauthorized("Expired token"))
                }
            }
            None => err(ErrorUnauthorized("No token provided")),
        }
    }
}

fn get_token_from_auth_header(auth: Option<&HeaderValue>) -> &str {
    let splitted_header_token: Vec<&str> =
        auth.unwrap().to_str().unwrap().split("Bearer").collect();
    splitted_header_token[1].trim()
}
