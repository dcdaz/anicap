use crate::{
    controllers::{AppUserRequest, AppUserTokenResponse, LoginAppUserRequest},
    models::NewAppUser,
    repositories::app_user_repository,
    utils::{ServerError, SqlConnection},
};

pub fn register(
    connection: &mut SqlConnection,
    request: AppUserRequest,
) -> Result<usize, ServerError> {
    app_user_repository::register(connection, NewAppUser::from_request(request))
}

pub fn login(
    connection: &mut SqlConnection,
    request: LoginAppUserRequest,
) -> Result<AppUserTokenResponse, ServerError> {
    app_user_repository::login(connection, request)
}
