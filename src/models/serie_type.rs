use crate::controllers::SerieTypeRequest;
use crate::schema::serie_type;

#[derive(Queryable, Selectable)]
#[diesel(table_name = serie_type)]
pub struct SerieType {
    pub id: i16,
    pub name: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = serie_type)]
pub struct NewSerieType {
    pub user_id: i16,
    pub name: String,
}

impl NewSerieType {
    pub fn from_request(logged_user_id: i16, request: SerieTypeRequest) -> Self {
        NewSerieType { 
            user_id: logged_user_id,
            name: request.name,
        }
    }
}