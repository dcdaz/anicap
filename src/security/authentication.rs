use actix_web::{
    dev, error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest,
};
use futures::future::{err, ok, Ready};

use crate::models::Claims;
use crate::utils::{pool_handler, SqlPool, SqlPooledConnection};

pub struct AuthenticatedRequest {
    pub user_id: i16,
    pub connection: SqlPooledConnection,
}

impl FromRequest for AuthenticatedRequest {
    type Error = Error;
    type Future = Ready<Result<AuthenticatedRequest, Error>>;

    fn from_request(http_request: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let token_cookie = http_request.cookie("token");

        match token_cookie {
            Some(cookie) => {
                let token = cookie.value();
                let pool_from_app_data = http_request.app_data::<web::Data<SqlPool>>();
                let connection = pool_handler(pool_from_app_data);
                if Claims::is_valid_token(token) {
                    let decoded_token = Claims::decode_token(token);
                    ok(AuthenticatedRequest {
                        user_id: decoded_token.unwrap().claims.id,
                        connection: connection.unwrap(),
                    })
                } else {
                    err(ErrorUnauthorized("Expired token"))
                }
            }
            None => err(ErrorUnauthorized("No token provided")),
        }
    }
}
