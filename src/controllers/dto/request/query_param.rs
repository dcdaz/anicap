use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
#[serde(rename_all="camelCase")]
#[serde(default)]
#[into_params(parameter_in = Query)]
pub struct QueryParam {
    #[param(min_length = 1, max_length = 255, pattern = "[a-z]*")]
    #[serde(deserialize_with = "detrim::option_string_non_empty")]
    pub name: Option<String>,
    #[param(minimum = 0)]
    pub season: Option<i16>,
    #[param(minimum = 0)]
    pub chapter: Option<i16>,
    #[param(minimum = 0.0, maximum = 10.0)]
    pub score: Option<f32>,
    pub favorite: Option<bool>,
    pub wish_to_see: Option<bool>,
    #[param(minimum = 0, maximum = 2)]
    pub watch_status: Option<i16>,
}

impl Default for QueryParam {
    fn default() -> Self {
        QueryParam {
            name: None,
            season: None,
            chapter: None,
            score: None,
            favorite: None,
            wish_to_see: None,
            watch_status: None,
        }
    }
}
