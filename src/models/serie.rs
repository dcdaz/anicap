use crate::utils::database_utils::SqlConnection;
use diesel::{
    insert_into, update, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::schema::serie;
use super::schema::serie::dsl::*;
use crate::utils::error_mapper::ServerError;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Serie {
    pub id: i16,
    pub user_id: i16,
    pub name: String,
    pub season: i16,
    pub chapter: i16,
    pub score: f32,
}

#[derive(Validate, Insertable, Serialize, Deserialize)]
#[serde(default)]
#[diesel(table_name = serie)]
pub struct NewSerie {
    pub user_id: i16,
    #[validate(length(min = 1, max = 255, code = "too_long", message = "Name is too long"))]
    pub name: String,
    #[validate(range(min = 0, code = "invalid_value", message = "Season must be 0 or greater"))]
    pub season: i16,
    #[validate(range(min = 0, code = "invalid_value", message = "Chapter must be 0 or greater"))]
    pub chapter: i16,
    #[validate(range(min = 0.0, max = 10.0, code = "out_of_range", message = "Score must be between 0.0 and 10.0"))]
    pub score: f32,
}

impl Default for NewSerie {
    fn default() -> Self {
        NewSerie {
            user_id: 0,
            name: String::from(""),
            season: 0,
            chapter: 0,
            score: 0.0,
        }
    }
}

impl Serie {
    pub fn add_new_serie(
        connection: &mut SqlConnection,
        new_serie: NewSerie,
    ) -> Result<usize, ServerError> {
        new_serie.validate().unwrap();
        insert_into(serie)
            .values(&new_serie)
            .execute(connection)
            .map_err(|error| ServerError::InsertFailure(error.to_string()))
    }

    pub fn get_series(
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
        match updated_serie.validate() {
            Ok(_) => (),
            Err(e) => return Err(ServerError::ObjectValidationError(e.to_string())),
        };
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
}
