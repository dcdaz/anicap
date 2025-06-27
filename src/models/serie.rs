use crate::controllers::SerieRequest;
use crate::schema::serie;

#[derive(Queryable)]
pub struct Serie {
    pub id: i16,
    pub user_id: i16,
    pub name: String,
    pub season: i16,
    pub chapter: i16,
    pub score: f32,
}

#[derive(Insertable)]
#[diesel(table_name = serie)]
pub struct NewSerie {
    pub user_id: i16,
    pub name: String,
    pub season: i16,
    pub chapter: i16,
    pub score: f32,
}

impl NewSerie {
    pub fn from_request(logged_user_id: i16, request: SerieRequest) -> Self {
        NewSerie {
            user_id: logged_user_id,
            name: request.name,
            season: request.season,
            chapter: request.chapter,
            score: request.score,
        }
    }
}
