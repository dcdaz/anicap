use super::handler_types::ServerResponse;
use crate::controllers::{AppUserRequest, AppUserTokenResponse, ErrorMessage, LoginAppUserRequest};
use crate::services::app_user_service;
use crate::utils::{pool_handler, SqlPool};
use actix_web::{post, web, HttpResponse};

#[utoipa::path(
    post,
    path = "/register",
    tag = "AppUser",
    description = "Register new user in the app",
    request_body(
        content = AppUserRequest,
        example = json!({"first_name": "John","last_name": "Doe","username": "john","email": "johndoe@example.com","password": "password123"})
    ),
    responses(
        (status = 201),
        (status = 400, body = ErrorMessage),
    )
)]
#[post("/register")]
pub async fn register(
    pool: web::Data<SqlPool>,
    request: web::Json<AppUserRequest>,
) -> ServerResponse {
    let connection = pool_handler(Some(&pool));
    app_user_service::register(&mut connection.unwrap(), request.into_inner())
        .map(|_| HttpResponse::Created().finish())
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "AppUser",
    description = "Login user and returns a Bearer token",
    request_body(
        content = LoginAppUserRequest,
        example = json!({"username": "john","password": "password123"})
    ),
    responses(
        (status = 200, body = AppUserTokenResponse),
        (status = 400, body = ErrorMessage),
    )
)]
#[post("login")]
pub async fn login(
    pool: web::Data<SqlPool>,
    request: web::Json<LoginAppUserRequest>,
) -> ServerResponse {
    let connection = pool_handler(Some(&pool));
    app_user_service::login(&mut connection.unwrap(), request.into_inner())
        .map(|token| HttpResponse::Ok().json(token))
}
