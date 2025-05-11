use actix_web::{HttpResponse, Responder, get, web};

#[get("/z")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body(format!("OK"))
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello from base!"))
}

// Export the route config
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").service(health).service(hello));
}
