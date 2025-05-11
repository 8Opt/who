use crate::repository::auth::{UserRepository, User};
use crate::error::AuthError;
use sqlx::PgPool;

pub fn get_repo(pool: &PgPool) -> UserRepository {
    UserRepository::new(pool.clone())
}


pub async fn register(pool: &PgPool, user: User) -> Result<(), AuthError> {
    let repo = get_repo(pool);
    repo.create(user).await.map_err(AuthError::from)
}

pub async fn login(pool: &PgPool, id: i32) -> Result<User, AuthError> {
    let repo = get_repo(pool);
    let user = repo.get_by_id(id).await?;
    Ok(user)
}

pub async fn update(pool: &PgPool, user: User) -> Result<(), AuthError> {
    let repo = get_repo(pool);
    repo.update(user).await.map_err(AuthError::from)
}

pub async fn delete(pool: &PgPool, user_id: i32) -> Result<(), AuthError> {
    let repo = get_repo(pool);
    repo.delete(user_id).await.map_err(AuthError::from)
}

pub async fn me(pool: &PgPool, user_id: i32) -> Result<User, AuthError> {
    let repo = get_repo(pool);
    let user = repo.get_by_id(user_id).await?;
    Ok(user)
}
