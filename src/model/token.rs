use chrono::Duration;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

use super::app_user::{AppUser, AppUserToken};
use crate::utils::ServerError;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: i16,
    username: String,
    exp: i64,
}

impl Claims {
    pub fn create_token(app_user: AppUser) -> Result<AppUserToken, ServerError> {
        let claims = Self::with_app_user(&app_user);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(Self::get_jwt_secret_key().as_bytes()),
        )
        .map_err(|error| ServerError::TokenCreationError(error.to_string()));

        Ok(AppUserToken {
            token_type: "Bearer".into(),
            access_token: token.unwrap(),
        })
    }

    pub fn decode_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(Self::get_jwt_secret_key().as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
    }

    pub fn is_valid_token(token: &str) -> bool {
        let decoded_token = Self::decode_token(token).unwrap().claims;
        if decoded_token.exp > chrono::Local::now().timestamp() {
            return true;
        }
        false
    }

    fn with_app_user(app_user: &AppUser) -> Self {
        use chrono::Local;
        let token_duration = crate::configuration::SERVER_CONFIG.token.duration;

        Claims {
            id: app_user.id,
            username: app_user.username.to_owned(),
            exp: (Local::now() + Duration::minutes(token_duration.into())).timestamp(),
        }
    }

    fn get_jwt_secret_key() -> String {
        crate::configuration::SERVER_CONFIG.clone().token.jwt_secret
    }
}
