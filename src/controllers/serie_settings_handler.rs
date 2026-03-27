use actix_web::{HttpResponse, get, post, delete, web};

use crate::{controllers::{ErrorMessage, SerieGenreRequest, SerieGenreResponse, SerieTypeRequest, SerieTypeResponse, handler_types::ServerResponse}, security::AuthenticatedRequest, services::serie_settings_service};

#[utoipa::path(
    post,
    path = "/serie/settings/genre",
    tag = "Series Settings",
    description = "Add a new serie genre",
    security(("token" = [])),
    request_body(
        content = SerieGenreRequest,
        example = json!({"name": "Fantasy"})
    ),
    responses(
        (status = 201),
        (status = 400, body = ErrorMessage),
        (status = 401, body = ErrorMessage),
    )
)]
#[post("serie/settings/genre")]
pub async fn insert_serie_genre(
    authenticated_request: AuthenticatedRequest,
    request: web::Json<SerieGenreRequest>,
) -> ServerResponse {
    let request = request.into_inner();
    request.validate_request();
    serie_settings_service::add_new_serie_genre(
        authenticated_request,
        request
    ).map(|_| HttpResponse::Created().finish())
}

#[utoipa::path(
    get,
    path = "/serie/settings/genre",
    tag = "Series Settings",
    description = "Search serie genres",
    security(("token" = [])),
    responses(
        (status = 200, body = Vec<SerieGenreResponse>),
        (status = 401, body = ErrorMessage),
    )
)]
#[get("/serie/settings/genre")]
pub async fn search_serie_genres(
    authenticated_request: AuthenticatedRequest,
) -> ServerResponse {
    serie_settings_service::search_serie_genres(authenticated_request)
        .map(|series| HttpResponse::Ok().json(series))
}

#[utoipa::path(
    delete,
    path = "/serie/settings/genre/{serie_genre_id}",
    tag = "Series Settings",
    description= "Delete a serie genre",
    params(("serie_genre_id" = i16, Path, description = "Id of a serie genre")),
    security(("token" = [])),
    responses(
        (status = 204),
        (status = 404),
    )
)]
#[delete("/serie/settings/genre/{serie_genre_id}")]
pub async fn delete_serie_genre(
    authenticated_request: AuthenticatedRequest,
    serie_genre_id: web::Path<i16>,
) -> ServerResponse {
    serie_settings_service::delete_serie_genre(
        authenticated_request,
        serie_genre_id.into_inner()
    ).map(|_| HttpResponse::NoContent().finish())
}

#[utoipa::path(
    post,
    path = "/serie/settings/type",
    tag = "Series Settings",
    description = "Add a new serie type",
    security(("token" = [])),
    request_body(
        content = SerieTypeRequest,
        example = json!({"name": "Anime"})
    ),
    responses(
        (status = 201),
        (status = 400, body = ErrorMessage),
        (status = 401, body = ErrorMessage),
    )
)]
#[post("serie/settings/type")]
pub async fn insert_serie_type(
    authenticated_request: AuthenticatedRequest,
    request: web::Json<SerieTypeRequest>,
) -> ServerResponse {
    let request = request.into_inner();
    request.validate_request();
    serie_settings_service::add_new_serie_type(
        authenticated_request,
        request
    ).map(|_| HttpResponse::Created().finish())
}

#[utoipa::path(
    get,
    path = "/serie/settings/type",
    tag = "Series Settings",
    description = "Search serie types",
    security(("token" = [])),
    responses(
        (status = 200, body = Vec<SerieTypeResponse>),
        (status = 401, body = ErrorMessage),
    )
)]
#[get("/serie/settings/genre")]
pub async fn search_serie_types(
    authenticated_request: AuthenticatedRequest,
) -> ServerResponse {
    serie_settings_service::search_serie_types(authenticated_request)
        .map(|series| HttpResponse::Ok().json(series))
}


#[utoipa::path(
    delete,
    path = "/serie/settings/type/{serie_type_id}",
    tag = "Series Settings",
    description= "Delete a serie type",
    params(("serie_type_id" = i16, Path, description = "Id of a serie type")),
    security(("token" = [])),
    responses(
        (status = 204),
        (status = 404),
    )
)]
#[delete("/serie/settings/type/{serie_type_id}")]
pub async fn delete_serie_type(
    authenticated_request: AuthenticatedRequest,
    serie_type_id: web::Path<i16>,
) -> ServerResponse {
    serie_settings_service::delete_serie_type(
        authenticated_request,
        serie_type_id.into_inner()
    ).map(|_| HttpResponse::NoContent().finish())
}
