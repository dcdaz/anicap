use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod configuration;
mod handler;
mod model;
mod security;
mod schema;
mod utils;

use configuration::{get_cors, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Put log type as env variable since env_logger uses it
    std::env::set_var(
        "RUST_LOG",
        configuration::SERVER_CONFIG.clone().log_type,
    );
    // Init env_logger
    env_logger::init();

    let server_url = format!(
        "{}:{}",
        configuration::SERVER_CONFIG.ip_address,
        configuration::SERVER_CONFIG.server_port
    );
    println!("\nServer running on :{}\n", server_url);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(get_cors()) // Cors
            .app_data(Data::new(utils::connect_database())) //Database
            .configure(routes) // Routes
    })
    .bind(server_url)?
    .run()
    .await
}
