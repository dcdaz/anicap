use super::controller_types::ServerResponse;
use crate::models::app_user::{AppUser, LoginAppUser, NewAppUser};
use crate::utils::database_utils::{pool_handler, SqlPool};
use actix_web::{post, web, HttpResponse};

#[post("/register")]
pub async fn register(pool: web::Data<SqlPool>, app_user: web::Json<NewAppUser>) -> ServerResponse {
    let connection = pool_handler(Some(&pool));
    AppUser::register(&mut connection.unwrap(), app_user.into_inner())
        .map(|_| HttpResponse::Created().finish())
}

#[post("login")]
pub async fn login(
    pool: web::Data<SqlPool>,
    login_app_user: web::Json<LoginAppUser>,
) -> ServerResponse {
    let connection = pool_handler(Some(&pool));
    AppUser::login(&mut connection.unwrap(), login_app_user.into_inner())
        .map(|token| HttpResponse::Ok().json(token))
}
