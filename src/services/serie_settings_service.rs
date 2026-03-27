use crate::controllers::{SerieGenreRequest, SerieGenreResponse, SerieTypeRequest, SerieTypeResponse};
use crate::models::NewSerieGenre;
use crate::{models::NewSerieType, security::AuthenticatedRequest, utils::ServerError};
use crate::repositories::serie_settings_repository;

pub fn add_new_serie_genre(
    mut authenticated_request: AuthenticatedRequest,
    request: SerieGenreRequest,
) -> Result<usize, ServerError> {
    serie_settings_repository::add_new_serie_genre(
        &mut authenticated_request.connection,
        NewSerieGenre::from_request(authenticated_request.user_id, request),
    )
}

pub fn search_serie_genres(
    mut authenticated_request: AuthenticatedRequest,
) -> Result<Vec<SerieGenreResponse>, ServerError> {
    serie_settings_repository::search_serie_genres(
        &mut authenticated_request.connection,
        authenticated_request.user_id,
    )
    .map(SerieGenreResponse::from_serie_genres)
}

pub fn delete_serie_genre(
    mut authenticated_request: AuthenticatedRequest,
    serie_genre_id: i16,
) -> Result<usize, ServerError> {
    serie_settings_repository::delete_serie_genre(
        &mut authenticated_request.connection,
        serie_genre_id
    )
}

pub fn add_new_serie_type(
    mut authenticated_request: AuthenticatedRequest,
    request: SerieTypeRequest,
) -> Result<usize, ServerError> {
    serie_settings_repository::add_new_serie_type(
        &mut authenticated_request.connection,
        NewSerieType::from_request(authenticated_request.user_id, request),
    )
}

pub fn search_serie_types(
    mut authenticated_request: AuthenticatedRequest,
) -> Result<Vec<SerieTypeResponse>, ServerError> {
    serie_settings_repository::search_serie_types(
        &mut authenticated_request.connection,
        authenticated_request.user_id,
    )
    .map(SerieTypeResponse::from_serie_genres)
}

pub fn delete_serie_type(
    mut authenticated_request: AuthenticatedRequest,
    serie_type_id: i16,
) -> Result<usize, ServerError> {
    serie_settings_repository::delete_serie_type(
        &mut authenticated_request.connection,
        serie_type_id
    )
}