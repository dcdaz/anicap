use serde::Serialize;

#[derive(Serialize)]
pub struct AppUserTokenResponse {
    pub token_type: String,
    pub access_token: String,
}
