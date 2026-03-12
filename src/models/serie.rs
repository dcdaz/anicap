use crate::controllers::SerieRequest;
use crate::schema::serie;

#[derive(Queryable, Selectable)]
#[diesel(table_name = serie)]
pub struct Serie {
    pub id: i16,
    pub name: String,
    pub season: i16,
    pub chapter: i16,
    pub score: f32,
    pub favorite: bool,
    pub wish_to_see: bool,
    pub watch_status: i16,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = serie)]
pub struct NewSerie {
    pub user_id: i16,
    pub name: String,
    pub season: i16,
    pub chapter: i16,
    pub score: f32,
    pub favorite: bool,
    pub wish_to_see: bool,
    pub watch_status: i16,
}

impl NewSerie {
    pub fn from_request(logged_user_id: i16, request: SerieRequest) -> Self {
        NewSerie {
            user_id: logged_user_id,
            name: request.name,
            season: request.season,
            chapter: request.chapter,
            score: request.score,
            favorite: request.favorite,
            wish_to_see: request.wish_to_see,
            watch_status: request.watch_status,
        }
    }
}
