#![allow(unused)] // TODO delete this when have a settings page for user
use crate::controllers::AppUserRequest;
use crate::schema::app_user;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct AppUser {
    pub id: i16,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub register_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_user)]
pub struct NewAppUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl NewAppUser {
    pub fn from_request(request: AppUserRequest) -> Self {
        use data_encoding::BASE64;
        NewAppUser {
            first_name: request.first_name,
            last_name: request.last_name,
            username: request.username,
            email: request.email,
            password: BASE64.encode(request.password.as_bytes()),
        }
    }
}
