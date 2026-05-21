use crate::controllers::SerieGenreRequest;
use crate::schema::serie_genre;

#[derive(Queryable, Selectable)]
#[diesel(table_name = serie_genre)]
pub struct SerieGenre {
    pub id: i16,
    pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = serie_genre)]
pub struct NewSerieGenre {
    pub user_id: i32,
    pub name: String,
}

impl NewSerieGenre {
    pub fn from_request(logged_user_id: i32, request: SerieGenreRequest) -> Self {
        NewSerieGenre { 
            user_id: logged_user_id,
            name: request.name,
        }
    }
}