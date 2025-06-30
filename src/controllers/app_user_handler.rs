use super::handler_types::ServerResponse;
use crate::configuration::SERVER_CONFIG;
use crate::controllers::{AppUserRequest, AppUserTokenResponse, ErrorMessage, LoginAppUserRequest};
use crate::security::AuthenticatedRequest;
use crate::services::app_user_service;
use crate::utils::{pool_handler, SqlPool};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::{get, post, web, HttpResponse};

#[utoipa::path(
    post,
    path = "/appuser/register",
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
#[post("/appuser/register")]
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
    path = "/appuser/login",
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
#[post("/appuser/login")]
pub async fn login(
    pool: web::Data<SqlPool>,
    request: web::Json<LoginAppUserRequest>,
) -> ServerResponse {
    let connection = pool_handler(Some(&pool));
    app_user_service::login(&mut connection.unwrap(), request.into_inner())
        .map(|token| {
            let cookie = Cookie::build("token", token.clone().access_token)
                .path("/")
                .max_age(Duration::minutes(SERVER_CONFIG.token.duration.into()))
                .http_only(true)
                .finish();

            HttpResponse::Ok()
            .cookie(cookie)
            .json(token)
    })
}


#[utoipa::path(
    get,
    path = "/appuser/logout",
    tag = "AppUser",
    description = "Logout user from session",
    security(("token" = [])),
    responses(
        (status = 204),
        (status = 401, body = ErrorMessage),
)
)]
#[get("/appuser/logout")]
pub async fn logout(_: AuthenticatedRequest) -> ServerResponse {
    let cookie  = Cookie::build("token", "")
        .path("/")
        .max_age(Duration::new(-1, 0))
        .http_only(true)
        .finish();
    Ok(HttpResponse::NoContent().cookie(cookie).finish())
}