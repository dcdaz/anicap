use serde::{Deserialize, Serialize};
use crate::utils::database_utils::SqlConnection;
use diesel::{
    insert_into,
    update,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods
};

use super::schema::user_serie;
use super::schema::user_serie::dsl::*;
use crate::utils::error_mapper::ServerError;

#[derive(Queryable, Serialize, Deserialize)]
pub struct UserSerie {
    pub id: i16,
    pub user_id: i16,
    pub name: String,
    pub season: i16,
    pub chapter: i16,
    pub score: f32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(default)]
#[diesel(table_name = user_serie)]
pub struct NewUserSerie {
    pub user_id: i16,
    pub name: String,
    pub season: i16,
    pub chapter: i16,
    pub score: f32,
}

impl Default for NewUserSerie {
    fn default() -> Self {
        NewUserSerie {
            user_id: 0,
            name: String::from(""),
            season: 0,
            chapter: 0,
            score: 0.0,
        }
    }
}

impl UserSerie {

    pub fn add_new_serie(
        connection: &mut SqlConnection,
        new_user_serie: NewUserSerie
    ) -> Result<usize, ServerError> {
        insert_into(user_serie)
            .values(&new_user_serie)
            .execute(connection)
            .map_err(|error| { ServerError::InsertFailure(error.to_string()) })
    }

    pub fn get_series(
        connection: &mut SqlConnection,
        logged_user_id: i16
    ) -> Result<Vec<UserSerie>, ServerError> {
        user_serie
            .filter(user_id.eq(logged_user_id))
            .select((
                user_serie::id,
                user_serie::user_id,
                user_serie::name,
                user_serie::season,
                user_serie::chapter,
                user_serie::score,
            ))
            .load::<UserSerie>(connection)
            .map_err(|error| { ServerError::ErrorRetrievingData(error.to_string()) })
    }

    pub fn update_serie(
        connection: &mut SqlConnection,
        serie_id: i16,
        updated_user_serie: NewUserSerie
    ) -> Result<usize, ServerError> {
        update(user_serie.filter(id.eq(serie_id)))
            .set((
                name.eq(updated_user_serie.name),
                season.eq(updated_user_serie.season),
                chapter.eq(updated_user_serie.chapter),
                score.eq(updated_user_serie.score)
            ))
            .execute(connection)
            .map_err(|error| { ServerError::ObjectNotFound(error.to_string()) })
    }
}
