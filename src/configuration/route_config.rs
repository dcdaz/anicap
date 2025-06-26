use actix_cors::Cors;
use actix_web::web;

pub fn get_cors() -> Cors {
    Cors::permissive().max_age(3600)
}

// Endpoints registration config
pub fn routes(config: &mut web::ServiceConfig) {
    use crate::controller::{
        app_user_controller::{login, register},
        serie_controller::{get_serie_by_id, get_series, insert_serie, update_serie},
    };

    config
        .service(register)
        .service(login)
        .service(insert_serie)
        .service(get_series)
        .service(get_serie_by_id)
        .service(update_serie);
}
