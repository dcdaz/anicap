use super::openapi_config::ApiDoc;
use actix_cors::Cors;
use actix_web::web;
use utoipa::OpenApi;

pub fn get_cors() -> Cors {
    Cors::permissive().max_age(3600)
}
// Endpoints registration config
pub fn routes(config: &mut web::ServiceConfig) {
    use crate::controllers::{
        search_series,
        get_serie_by_id,
        insert_serie,
        login,
        logout,
        register,
        update_serie,
        delete_serie,
        insert_serie_genre,
        search_serie_genres,
        delete_serie_genre,
        insert_serie_type,
        search_serie_types,
        delete_serie_type,
    };
    use utoipa_swagger_ui::SwaggerUi;

    config
        .service(register)
        .service(login)
        .service(logout)
        .service(insert_serie)
        .service(search_series)
        .service(get_serie_by_id)
        .service(update_serie)
        .service(delete_serie)
        .service(insert_serie_genre)
        .service(search_serie_genres)
        .service(delete_serie_genre)
        .service(insert_serie_type)
        .service(search_serie_types)
        .service(delete_serie_type)
        .service(
            SwaggerUi::new("/swagger/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi().clone()),
        );
}
