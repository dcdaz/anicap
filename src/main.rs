use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use env_logger::{Builder, Target};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod configuration;
mod controllers;
mod migrator;
mod models;
mod repositories;
mod schema;
mod security;
mod services;
mod utils;

use configuration::{get_cors, routes};

fn exit(code: i32) {
    std::process::exit(code);
}

fn print_help() {
    let help = r#"
    Anicap Backend

    Usage: anicap <option>

    Available Options:
        -h, --help                      Show this help message
        init                            Init database, aka run the initial schema
        migrate                         Run migrations if there are any to run
    "#;
    println!("{}", help);
}

fn execute_action_based_on_args() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Too many args, use -h or --help to see available commands!");
        exit(1);
    } else if args.len() == 2 {
        let arg = args[1].clone();
        match arg.as_str() {
            "init" => {
                migrator::initialize_database();
                exit(0);
            },
            "migrate" => {
                migrator::migrate();
                exit(0);
            },
            "--help" | "-h" => {
                print_help();
                exit(0);
            },
            _ => {
                println!("Unrecognized command: '{}' use -h or --help to see available commands!" , arg);
                exit(1);
            },
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Put log type as env variable since env_logger uses it
    std::env::set_var("RUST_LOG", configuration::SERVER_CONFIG.clone().log_type);
    // Init env_logger
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();

    execute_action_based_on_args();

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
