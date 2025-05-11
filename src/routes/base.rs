use actix_web::{get, web, HttpResponse, Responder};

#[get("/healthz")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body(format!("OK"))
}

#[get("/hello-world")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello from base!"))
}


// Export the route config
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(hello);
}

