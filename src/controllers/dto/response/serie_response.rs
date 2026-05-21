use crate::models::Serie;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SerieResponse {
    pub id: i32,
    pub name: String,
    pub season: i16,
    pub chapter: i16,
    pub score: f32,
    pub favorite: bool,
    pub wish_to_see: bool,
    pub watch_status: i16
}

impl SerieResponse {
    pub fn from_serie(serie: Serie) -> Self {
        SerieResponse {
            id: serie.id,
            name: serie.name,
            season: serie.season,
            chapter: serie.chapter,
            score: serie.score,
            favorite: serie.favorite,
            wish_to_see: serie.wish_to_see,
            watch_status: serie.watch_status,
        }
    }

    pub fn from_series(series: Vec<Serie>) -> Vec<Self> {
        series.into_iter().map(SerieResponse::from_serie).collect()
    }
}
