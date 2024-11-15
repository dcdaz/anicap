use actix_web::{
    HttpResponse,
    web,
    get,
    post,
    put
};

use super::controller_types::ServerResponse;
use crate::models::user_serie::{UserSerie, NewUserSerie};
use crate::security::authentication::AuthenticatedRequest;

#[post("/serie")]
pub async fn insert_serie(
    mut authenticated_request: AuthenticatedRequest,
    mut new_user_serie: web::Json<NewUserSerie>
) -> ServerResponse {
    new_user_serie.user_id = authenticated_request.user_id;
    UserSerie::add_new_serie(
        &mut authenticated_request.connection,
        new_user_serie.into_inner()
    ).map(|_| {
        HttpResponse::Created().finish()
    })
}

#[get("/serie")]
pub async fn get_series(mut authenticated_request: AuthenticatedRequest) -> ServerResponse {
    UserSerie::get_series(
        &mut authenticated_request.connection,
        authenticated_request.user_id
    ).map(|response| { HttpResponse::Ok().json(response) })
}

#[put("/serie/{user_serie_id}")]
pub async fn update_serie(
    mut authenticated_request: AuthenticatedRequest,
    dynamic_path: web::Path<(i16,)>,
    updated_user_serie: web::Json<NewUserSerie>
) -> ServerResponse {
    UserSerie::update_serie(
        &mut authenticated_request.connection,
        dynamic_path.into_inner().0,
        updated_user_serie.into_inner()
    ).map(|_| { HttpResponse::Ok().finish() })
}
