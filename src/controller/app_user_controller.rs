use actix_web::{
    HttpResponse,
    web,
    post
};
use crate::utils::database_utils::{
    SqlPool,
    pool_handler
};
use crate::models::app_user::{
    AppUser,
    NewAppUser,
    LoginAppUser
};
use super::controller_types::ServerResponse;

#[post("/register")]
pub async fn register(
    pool: web::Data<SqlPool>,
    app_user: web::Json<NewAppUser>
) -> ServerResponse {
    let connection = pool_handler(Some(&pool));
    AppUser::register(&mut connection.unwrap(), app_user.into_inner()).map(|_| {
        HttpResponse::Created().finish()
    })
}


#[post("login")]
pub async fn login(
    pool: web::Data<SqlPool>,
    login_app_user: web::Json<LoginAppUser>
) -> ServerResponse {
    let connection = pool_handler(Some(&pool));
    AppUser::login(&mut connection.unwrap(), login_app_user.into_inner()).map(|token| {
        HttpResponse::Ok().json(token)
    })
}
