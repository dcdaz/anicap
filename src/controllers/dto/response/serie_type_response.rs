use serde::Serialize;
use utoipa::ToSchema;

use crate::models::SerieType;

#[derive(ToSchema, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SerieTypeResponse {
    pub id: i16,
    pub name: String,
}


impl SerieTypeResponse {
    pub fn from_serie_genre(serie_type: SerieType) -> Self {
        SerieTypeResponse { 
            id: serie_type.id,
            name: serie_type.name
        }
    }

    pub fn from_serie_genres(series: Vec<SerieType>) -> Vec<Self> {
        series.into_iter().map(SerieTypeResponse::from_serie_genre).collect()
    }
}