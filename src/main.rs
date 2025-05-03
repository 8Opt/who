
use actix_web::{middleware::Logger, App, HttpServer};

// ----- Just to make the callling shorter
mod core; 
mod routes;

use crate::core::config::Config;
// ----- 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config = Config::load();

    let _server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(routes::base::routes)
    })
    .bind((
        config.host.as_str(), // Use .as_str() to borrow &str from the String
        // `port` is integer, so we need to convert it to string
        config.port.to_string().parse().unwrap()))?
    .run()
    .await;

    Ok(())
}


