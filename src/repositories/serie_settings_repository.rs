use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, delete, insert_into};
use crate::models::{SerieGenre, SerieType};
use crate::schema::serie_genre;
use crate::schema::serie_genre::dsl::*;
use crate::schema::serie_type;
use crate::schema::serie_type::dsl::*;
use crate::{models::{NewSerieGenre, NewSerieType}, utils::{ServerError, SqlConnection}};

pub fn add_new_serie_genre(
    connection: &mut SqlConnection,
    new_serie_genre: NewSerieGenre,
) -> Result<usize, ServerError> {
    insert_into(serie_genre)
        .values(new_serie_genre)
        .execute(connection)
        .map_err(|error| ServerError::InsertFailure(error.to_string()))
}

pub fn search_serie_genres(
    connection: &mut SqlConnection,
    logged_user_id: i32,
) -> Result<Vec<SerieGenre>, ServerError> {
    serie_genre
        .filter(serie_genre::user_id.eq(logged_user_id))
        .select(SerieGenre::as_select())
        .load::<SerieGenre>(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}

pub fn delete_serie_genre(
    connection: &mut SqlConnection,
    serie_genre_id: i16
) -> Result<usize, ServerError> {
    delete(serie_genre.filter(serie_genre::id.eq(serie_genre_id)))
        .execute(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}

pub fn add_new_serie_type(
    connection: &mut SqlConnection,
    new_serie_type: NewSerieType,
) -> Result<usize, ServerError> {
    insert_into(serie_type)
        .values(new_serie_type)
        .execute(connection)
        .map_err(|error| ServerError::InsertFailure(error.to_string()))
}

pub fn search_serie_types(
    connection: &mut SqlConnection,
    logged_user_id: i32,
) -> Result<Vec<SerieType>, ServerError> {
    serie_type
        .filter(serie_type::user_id.eq(logged_user_id))
        .select(SerieType::as_select())
        .load::<SerieType>(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}

pub fn delete_serie_type(
    connection: &mut SqlConnection,
    serie_type_id: i16
) -> Result<usize, ServerError> {
    delete(serie_type.filter(serie_type::id.eq(serie_type_id)))
        .execute(connection)
        .map_err(|error| ServerError::ObjectNotFound(error.to_string()))
}
