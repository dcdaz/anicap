use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Serialize, ToSchema)]
#[serde(rename_all="camelCase")]
pub struct AppUserTokenResponse {
    pub token_type: String,
    pub access_token: String,
}
