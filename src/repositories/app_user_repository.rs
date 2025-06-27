use crate::schema::app_user::dsl::*;
use crate::{
    controllers::{AppUserTokenResponse, LoginAppUserRequest},
    models::{AppUser, Claims, NewAppUser},
    utils::{ServerError, SqlConnection},
};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn register(
    connection: &mut SqlConnection,
    new_user: NewAppUser,
) -> Result<usize, ServerError> {
    insert_into(app_user)
        .values(&new_user)
        .execute(connection)
        .map_err(|error| ServerError::InsertFailure(error.to_string()))
}

pub fn login(
    connection: &mut SqlConnection,
    login_app_user: LoginAppUserRequest,
) -> Result<AppUserTokenResponse, ServerError> {
    use data_encoding::BASE64;
    let logged_app_user = app_user
        .filter(username.eq(login_app_user.username.clone()))
        .filter(password.eq(BASE64.encode(login_app_user.password.as_bytes())))
        .first::<AppUser>(connection)
        .map_err(|_| ServerError::ObjectNotFound(login_app_user.username.to_string()));

    logged_app_user.and_then(Claims::create_token)
}
