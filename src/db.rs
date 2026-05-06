use sqlx::{MySql, Pool};

pub type DbPool = Pool<MySql>;

pub async fn create_pool(database_url: &str) -> DbPool {
    sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Error creating pool")
}