use sqlx::{PgPool, FromRow};
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use crate::core::db_connection::CURD;

#[derive(Debug, Clone, FromRow)]
pub struct User {       
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CURD<User> for UserRepository {
    fn create(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
            user.username,
            user.email,
            user.password_hash,
        )
        .execute(&self.pool)?;
        Ok(())
    }

    fn get_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)?;
        Ok(user)
    }

    fn update(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET username = $1, email = $2, password_hash = $3, updated_at = now() WHERE id = $4",
            user.username,
            user.email,
            user.password_hash,
            user.id,
        )
        .execute(&self.pool)?;
        Ok(())
    }

    fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            id
        )
        .execute(&self.pool)?;
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users ORDER BY id"
        )
        .fetch_all(&self.pool)?;
        Ok(users)
    }
}
