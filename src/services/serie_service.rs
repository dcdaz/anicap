use crate::controllers::{SerieRequest, SerieResponse};
use crate::models::NewSerie;
use crate::repositories::serie_repository;
use crate::security::AuthenticatedRequest;
use crate::utils::ServerError;

pub fn add_new_serie(
    mut authenticated_request: AuthenticatedRequest,
    request: SerieRequest,
) -> Result<usize, ServerError> {
    request.validate_request();
    serie_repository::add_new_serie(
        &mut authenticated_request.connection,
        NewSerie::from_request(authenticated_request.user_id, request),
    )
}

pub fn get_all_series(
    mut authenticated_request: AuthenticatedRequest,
) -> Result<Vec<SerieResponse>, ServerError> {
    serie_repository::get_all_series(
        &mut authenticated_request.connection,
        authenticated_request.user_id,
    )
    .map(SerieResponse::from_series)
}

pub fn get_serie_by_id(
    mut authenticated_request: AuthenticatedRequest,
    serie_id: i16,
) -> Result<SerieResponse, ServerError> {
    serie_repository::get_serie_by_id(
        &mut authenticated_request.connection,
        authenticated_request.user_id,
        serie_id,
    )
    .map(SerieResponse::from_serie)
}

pub fn update_serie(
    mut authenticated_request: AuthenticatedRequest,
    serie_id: i16,
    request: SerieRequest,
) -> Result<usize, ServerError> {
    request.validate_request();
    serie_repository::update_serie(
        &mut authenticated_request.connection,
        serie_id,
        NewSerie::from_request(authenticated_request.user_id, request),
    )
}
