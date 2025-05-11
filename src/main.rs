
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use log::{info, error};
// ----- Just to make the callling shorter
mod core; 
mod routes;
mod error;

use crate::core::config::Config;
// use crate::core::db_connection::init_db; 
// ----- 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config: Config = Config::load();

    // let pool = init_db(&config);
    // match pool { 
    //     Ok(pool) => print!("Connection establised"), 
    //     Err(err) => return Err(err)
    // }
    let server = HttpServer::new(move || {
        App::new()
        .wrap(Logger::new("%a %r %s %b %T"))
        .configure(routes::base::routes)
    })
    .workers(2); 

    match server.bind((
        config.host.as_str(), // Use .as_str() to borrow &str from the String
        // `port` is integer, so we need to convert it to string
        config.port.to_string().parse().unwrap())) {
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


