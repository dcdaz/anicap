use super::handler_types::ServerResponse;
use crate::controllers::{AppUserRequest, LoginAppUserRequest};
use crate::services::app_user_service;
use crate::utils::{pool_handler, SqlPool};
use actix_web::{post, web, HttpResponse};

#[post("/register")]
pub async fn register(
    pool: web::Data<SqlPool>,
    request: web::Json<AppUserRequest>,
) -> ServerResponse {
    let connection = pool_handler(Some(&pool));
    app_user_service::register(&mut connection.unwrap(), request.into_inner())
        .map(|_| HttpResponse::Created().finish())
}

#[post("login")]
pub async fn login(
    pool: web::Data<SqlPool>,
    request: web::Json<LoginAppUserRequest>,
) -> ServerResponse {
    let connection = pool_handler(Some(&pool));
    app_user_service::login(&mut connection.unwrap(), request.into_inner())
        .map(|token| HttpResponse::Ok().json(token))
}
