use actix_web::{middleware::Logger, web::Data, App, HttpServer};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod configuration;
mod controller;
mod models;
mod security;
mod utils;

use configuration::route_config::{get_cors, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Put log type as env variable since env_logger uses it
    std::env::set_var(
        "RUST_LOG",
        configuration::server_config::SERVER_CONFIG.clone().log_type,
    );
    // Init env_logger
    env_logger::init();

    let server_url = format!(
        "{}:{}",
        configuration::server_config::SERVER_CONFIG.ip_address,
        configuration::server_config::SERVER_CONFIG.server_port
    );
    println!("\nServer running on :{}\n", server_url);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(get_cors()) // Cors
            .app_data(Data::new(utils::database_utils::connect_database())) //Database
            .configure(routes) // Routes
    })
    .bind(server_url)?
    .run()
    .await
}
