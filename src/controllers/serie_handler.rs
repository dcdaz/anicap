use actix_web::{get, post, put, web, HttpResponse};

use super::handler_types::ServerResponse;
use crate::controllers::ErrorMessage;
use crate::controllers::SerieRequest;
use crate::controllers::SerieResponse;
use crate::security::AuthenticatedRequest;
use crate::services::serie_service;

#[utoipa::path(
    post,
    path = "/serie",
    tag = "Series",
    description = "Add a new serie",
    security(("token" = [])),
    request_body(
        content = SerieRequest,
        example = json!({"name": "Sample Serie", "season": 2, "chapter": 0, "score": 0})
    ),
    responses(
        (status = 201),
        (status = 400, body = ErrorMessage),
        (status = 401, body = ErrorMessage),
    )
)]
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
    tag = "Series",
    description = "Get all series",
    security(("token" = [])),
    responses(
        (status = 200, description= "Get all series", body = Vec<SerieResponse>),
        (status = 401, description= "Unauthorized", body = ErrorMessage),
    )
)]
#[get("/serie")]
pub async fn get_all_series(authenticated_request: AuthenticatedRequest) -> ServerResponse {
    serie_service::get_all_series(authenticated_request)
        .map(|series| HttpResponse::Ok().json(series))
}

#[utoipa::path(
    get,
    path = "/serie/{serie_id}",
    tag = "Series",
    description = "Get a serie by its id",
    params(("serie_id" = i16, Path, description = "Id of a serie")),
    security(("token" = [])),
    responses(
        (status = 200, body = SerieResponse),
        (status = 401, body = ErrorMessage),
        (status = 404, body = ErrorMessage),
    )
)]
#[get("/serie/{serie_id}")]
pub async fn get_serie_by_id(
    authenticated_request: AuthenticatedRequest,
    serie_id: web::Path<(i16,)>,
) -> ServerResponse {
    serie_service::get_serie_by_id(authenticated_request, serie_id.into_inner().0)
        .map(|serie| HttpResponse::Ok().json(serie))
}

#[utoipa::path(
    put,
    path = "/serie/{serie_id}",
    tag = "Series",
    description= "Update a serie",
    params(("serie_id" = i16, Path, description = "Id of a serie")),
    request_body(
        content = SerieRequest,
        example = json!({"name": "Sample Serie", "season": 2, "chapter": 0, "score": 0})
    ),
    security(("token" = [])),
    responses(
        (status = 204),
        (status = 400, body = ErrorMessage),
        (status = 401, body = ErrorMessage),
    )
)]
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
    .map(|_| HttpResponse::NoContent().finish())
}
