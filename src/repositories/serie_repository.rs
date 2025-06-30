use crate::models::{NewSerie, Serie};
use crate::schema::serie;
use crate::schema::serie::dsl::*;
use crate::utils::{ServerError, SqlConnection};
use diesel::{
    insert_into, update, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl,
};

pub fn add_new_serie(
    connection: &mut SqlConnection,
    new_serie: NewSerie,
) -> Result<usize, ServerError> {
    insert_into(serie)
        .values(&new_serie)
        .execute(connection)
        .map_err(|error| ServerError::InsertFailure(error.to_string()))
}

pub fn search_series(
    connection: &mut SqlConnection,
    logged_user_id: i16,
) -> Result<Vec<Serie>, ServerError> {
    serie
        .filter(user_id.eq(logged_user_id))
        .select((
            serie::id,
            serie::user_id,
            serie::name,
            serie::season,
            serie::chapter,
            serie::score,
        ))
        .load::<Serie>(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}

pub fn get_serie_by_id(
    connection: &mut SqlConnection,
    logged_user_id: i16,
    serie_id: i16,
) -> Result<Serie, ServerError> {
    serie
        .filter(user_id.eq(logged_user_id).and(id.eq(serie_id)))
        .select((
            serie::id,
            serie::user_id,
            serie::name,
            serie::season,
            serie::chapter,
            serie::score,
        ))
        .first::<Serie>(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}

pub fn update_serie(
    connection: &mut SqlConnection,
    serie_id: i16,
    updated_serie: NewSerie,
) -> Result<usize, ServerError> {
    update(serie.filter(id.eq(serie_id)))
        .set((
            name.eq(updated_serie.name),
            season.eq(updated_serie.season),
            chapter.eq(updated_serie.chapter),
            score.eq(updated_serie.score),
        ))
        .execute(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}
