use actix_cors::Cors;
use actix_web::web;
use utoipa::OpenApi;

pub fn get_cors() -> Cors {
    Cors::permissive().max_age(3600)
}

#[derive(OpenApi)]
#[openapi(paths(crate::controllers::get_all_series))]
struct ApiDoc;

// Endpoints registration config
pub fn routes(config: &mut web::ServiceConfig) {
    use crate::controllers::{
        get_all_series, get_serie_by_id, insert_serie, login, register, update_serie,
    };
    use utoipa_swagger_ui::SwaggerUi;

    config
        .service(register)
        .service(login)
        .service(insert_serie)
        .service(get_all_series)
        .service(get_serie_by_id)
        .service(update_serie)
        .service(
            SwaggerUi::new("/swagger/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi().clone()),
        );
}
