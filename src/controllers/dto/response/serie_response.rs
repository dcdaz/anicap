use crate::models::Serie;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct SerieResponse {
    pub id: i16,
    pub user_id: i16,
    pub name: String,
    pub season: i16,
    pub chapter: i16,
    pub score: f32,
}

impl SerieResponse {
    pub fn from_serie(serie: Serie) -> Self {
        SerieResponse {
            id: serie.id,
            user_id: serie.user_id,
            name: serie.name,
            season: serie.season,
            chapter: serie.chapter,
            score: serie.score,
        }
    }

    pub fn from_series(series: Vec<Serie>) -> Vec<Self> {
        series.into_iter().map(SerieResponse::from_serie).collect()
    }
}
