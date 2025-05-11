use crate::core::config::Config;
use log::{error, info};
use sqlx::{Error, PgPool, postgres::PgPoolOptions};

pub async fn init_db(config: &Config) -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgres://{}:{}@{}:{}/{}",
            config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
        ))
        .await;

    match pool {
        Ok(pool) => {
            info!("Connected to database");

            if let Err(e) = create_auth_tables(&pool).await {
                error!("Failed to create auth tables: {}", e);
                return Err(e);
            }

            Ok(pool)
        }
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            Err(e)
        }
    }
}

async fn create_auth_tables(pool: &PgPool) -> Result<(), Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT now(),
            updated_at TIMESTAMPTZ DEFAULT now()
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS refresh_tokens (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            token TEXT NOT NULL UNIQUE,
            expires_at TIMESTAMPTZ NOT NULL,
            created_at TIMESTAMPTZ DEFAULT now(),
            revoked BOOLEAN DEFAULT false
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS login_attempts (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
            ip_address TEXT,
            user_agent TEXT,
            success BOOLEAN,
            created_at TIMESTAMPTZ DEFAULT now()
        );
        "#,
    )
    .execute(pool)
    .await?;

    info!("Auth tables created");
    Ok(())
}

#[allow(dead_code)]
trait CURD<T> {
    fn create(&self) -> Result<(), sqlx::Error>;
    fn get_by_id(&self) -> Result<(), sqlx::Error>;
    fn update(&self) -> Result<(), sqlx::Error>;
    fn delete(&self) -> Result<(), sqlx::Error>;

    // * For system
    fn get_all(&self) -> Result<(), sqlx::Error>;
}
