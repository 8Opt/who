use sqlx::postgres::PgPoolOptions;

use crate::core::config::Config;

pub async fn connect(config: &Config) -> Result<PgPoolOptions, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!("postgres://{}:{}@{}:{}/{}", config.db_user, config.db_password, config.db_host, config.db_port, config.db_name))
        .await
        .expect("Failed to create pool");

    Ok(pool)
}


trait CURD<T> { 
    fn create(&self) -> Result<(), sqlx::Error>;
    fn get(&self) -> Result<(), sqlx::Error>;
    fn update(&self) -> Result<(), sqlx::Error>;
    fn delete(&self) -> Result<(), sqlx::Error>;

    // * For system
    fn get_all(&self) -> Result<(), sqlx::Error>;

}