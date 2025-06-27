use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct AppUserTokenResponse {
    pub token_type: String,
    pub access_token: String,
}
