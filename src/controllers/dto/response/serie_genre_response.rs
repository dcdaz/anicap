use serde::Serialize;
use utoipa::ToSchema;

use crate::models::SerieGenre;

#[derive(ToSchema, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SerieGenreResponse {
    pub id: i16,
    pub name: String,
}


impl SerieGenreResponse {
    pub fn from_serie_genre(serie_genre: SerieGenre) -> Self {
        SerieGenreResponse { 
            id: serie_genre.id,
            name: serie_genre.name
        }
    }

    pub fn from_serie_genres(series: Vec<SerieGenre>) -> Vec<Self> {
        series.into_iter().map(SerieGenreResponse::from_serie_genre).collect()
    }
}