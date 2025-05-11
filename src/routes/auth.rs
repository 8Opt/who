use actix_web::{HttpResponse, delete, get, post, web};
use sqlx::PgPool;

use crate::models::user::{User, UserRegister};
use crate::services::auth;
use crate::utils::hashing::Argon2id;

#[post("/register")]
async fn register(pool: web::Data<PgPool>, payload: web::Json<UserRegister>) -> HttpResponse {
    let user = User {
        id: 0, // bạn có thể bỏ qua nếu dùng SERIAL
        username: payload.username.clone(),
        email: payload.email.clone(),
        password_hash: Argon2id::hash_password(&payload.password.to_string()).unwrap(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    match auth::register(&pool, user).await {
        Ok(_) => HttpResponse::Ok().json("Register successful"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[get("/me")]
async fn me(pool: web::Data<PgPool>, user_id: web::Query<i32>) -> HttpResponse {
    match auth::me(&pool, *user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

#[delete("/{id}")]
async fn delete_user(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> HttpResponse {
    match auth::delete(&pool, *user_id).await {
        Ok(_) => HttpResponse::Ok().json("User deleted successfully"),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

// TODO: update functions

pub fn routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(register);
    // cfg.service(me);
    // cfg.service(delete_user);
    cfg.service(
        web::scope("/auth")
            .service(register)
            .service(me)
            .service(delete_user),
    );
}
