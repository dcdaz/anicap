use actix_web::{get, post, put, web, HttpResponse};

use super::controller_types::ServerResponse;
use crate::models::serie::{NewSerie, Serie};
use crate::security::authentication::AuthenticatedRequest;

#[post("/serie")]
pub async fn insert_serie(
    mut authenticated_request: AuthenticatedRequest,
    mut new_serie: web::Json<NewSerie>,
) -> ServerResponse {
    new_serie.user_id = authenticated_request.user_id;
    Serie::add_new_serie(
        &mut authenticated_request.connection,
        new_serie.into_inner(),
    )
    .map(|_| HttpResponse::Created().finish())
}

#[get("/serie")]
pub async fn get_series(mut authenticated_request: AuthenticatedRequest) -> ServerResponse {
    Serie::get_series(
        &mut authenticated_request.connection,
        authenticated_request.user_id,
    )
    .map(|response| HttpResponse::Ok().json(response))
}

#[get("/serie/{serie_id}")]
pub async fn get_serie_by_id(
    mut authenticated_request: AuthenticatedRequest,
    serie_id: web::Path<(i16,)>,
) -> ServerResponse {
    Serie::get_serie_by_id(
        &mut authenticated_request.connection,
        authenticated_request.user_id,
        serie_id.into_inner().0,
    )
    .map(|response| HttpResponse::Ok().json(response))
}

#[put("/serie/{serie_id}")]
pub async fn update_serie(
    mut authenticated_request: AuthenticatedRequest,
    serie_id: web::Path<(i16,)>,
    updated_serie: web::Json<NewSerie>,
) -> ServerResponse {
    Serie::update_serie(
        &mut authenticated_request.connection,
        serie_id.into_inner().0,
        updated_serie.into_inner(),
    )
    .map(|_| HttpResponse::Ok().finish())
}
