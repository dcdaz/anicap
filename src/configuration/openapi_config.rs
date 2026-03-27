use utoipa::{
    openapi::security::{HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controllers::register,
        crate::controllers::login,
        crate::controllers::logout,
        crate::controllers::insert_serie,
        crate::controllers::search_series,
        crate::controllers::get_serie_by_id,
        crate::controllers::update_serie,
        crate::controllers::delete_serie,
        crate::controllers::insert_serie_genre,
        crate::controllers::search_serie_genres,
        crate::controllers::delete_serie_genre,
        crate::controllers::insert_serie_type,
        crate::controllers::search_serie_types,
        crate::controllers::delete_serie_type,
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components.as_mut().unwrap().add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
