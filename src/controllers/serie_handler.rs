use actix_web::{get, post, put, web, HttpResponse};

use super::handler_types::ServerResponse;
use crate::controllers::SerieRequest;
use crate::controllers::SerieResponse;
use crate::security::AuthenticatedRequest;
use crate::services::serie_service;

#[post("/serie")]
pub async fn insert_serie(
    authenticated_request: AuthenticatedRequest,
    request: web::Json<SerieRequest>,
) -> ServerResponse {
    serie_service::add_new_serie(authenticated_request, request.into_inner())
        .map(|_| HttpResponse::Created().finish())
}

#[utoipa::path(
    get,
    path = "/serie",
    tag = "Get all series",
    responses(
        (status = 200, description= "Authenticated User", body = SerieResponse),
    )
)]
#[get("/serie")]
pub async fn get_all_series(authenticated_request: AuthenticatedRequest) -> ServerResponse {
    serie_service::get_all_series(authenticated_request)
        .map(|series| HttpResponse::Ok().json(series))
}

#[get("/serie/{serie_id}")]
pub async fn get_serie_by_id(
    authenticated_request: AuthenticatedRequest,
    serie_id: web::Path<(i16,)>,
) -> ServerResponse {
    serie_service::get_serie_by_id(authenticated_request, serie_id.into_inner().0)
        .map(|serie| HttpResponse::Ok().json(serie))
}

#[put("/serie/{serie_id}")]
pub async fn update_serie(
    authenticated_request: AuthenticatedRequest,
    serie_id: web::Path<(i16,)>,
    request: web::Json<SerieRequest>,
) -> ServerResponse {
    serie_service::update_serie(
        authenticated_request,
        serie_id.into_inner().0,
        request.into_inner(),
    )
    .map(|_| HttpResponse::Ok().finish())
}
