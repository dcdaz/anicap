use actix_web::{get, post, put, delete, web, HttpResponse};

use super::handler_types::ServerResponse;
use crate::controllers::ErrorMessage;
use crate::controllers::QueryParam;
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
    let request = request.into_inner();
    request.validate_request();
    serie_service::add_new_serie(
        authenticated_request,
        request
    ).map(|_| HttpResponse::Created().finish())
}

#[utoipa::path(
    get,
    path = "/serie",
    tag = "Series",
    description = "Search series",
    params(QueryParam),
    security(("token" = [])),
    responses(
        (status = 200, body = Vec<SerieResponse>),
        (status = 401, body = ErrorMessage),
    )
)]
#[get("/serie")]
pub async fn search_series(
    authenticated_request: AuthenticatedRequest,
    // query_param: actix_web::Result<web::Query<QueryParam>, actix_web::error::Error>,
    web::Query(query_param): web::Query<QueryParam>,
) -> ServerResponse {
    serie_service::search_series(authenticated_request, query_param)
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
    serie_id: web::Path<i16>,
) -> ServerResponse {
    serie_service::get_serie_by_id(authenticated_request, serie_id.into_inner())
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
        (status = 404),
        (status = 400, body = ErrorMessage),
        (status = 401, body = ErrorMessage),
    )
)]
#[put("/serie/{serie_id}")]
pub async fn update_serie(
    authenticated_request: AuthenticatedRequest,
    serie_id: web::Path<i16>,
    request: web::Json<SerieRequest>,
) -> ServerResponse {
    let request = request.into_inner();
    request.validate_request();
    serie_service::update_serie(
        authenticated_request,
        serie_id.into_inner(),
        request,
    )
    .map(|_| HttpResponse::NoContent().finish())
}

#[utoipa::path(
    delete,
    path = "/serie/{serie_id}",
    tag = "Series",
    description= "Delete a serie",
    params(("serie_id" = i16, Path, description = "Id of a serie")),
    security(("token" = [])),
    responses(
        (status = 204),
        (status = 404),
    )
)]
#[delete("/serie/{serie_id}")]
pub async fn delete_serie(
    authenticated_request: AuthenticatedRequest,
    serie_id: web::Path<i16>,
) -> ServerResponse {
    serie_service::delete_serie(
        authenticated_request,
        serie_id.into_inner()
    )
    .map(|_| HttpResponse::NoContent().finish())
}
