// external crate
use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;
use log::{error, info};

// internal crate
mod core;
mod error;
mod routes;
mod utils;

use crate::core::config::Config;
use crate::core::db_connection::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config: Config = Config::load();

    info!("Starting connecting to database");
    let _ = init_db(&config).await;

    info!("Starting server on {}:{}", config.host, config.port);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %r %s %b %T"))
            .configure(routes::health::routes)
            .configure(routes::auth::routes)
    })
    .workers(2);

    match server.bind((
        config.host.as_str(), // Use .as_str() to borrow &str from the String
        // `port` is integer, so we need to convert it to string
        config.port.to_string().parse().unwrap(),
    )) {
        Ok(server) => {
            info!("Server started on http://{}:{}", config.host, config.port);
            server.run().await
        }
        Err(e) => {
            error!("Failed to start server: {}", e);
            Err(e)
        }
    }
}
