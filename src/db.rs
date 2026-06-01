use sqlx::{MySql, Pool};

pub type DbPool = Pool<MySql>;

pub async fn create_pool(database_url: &str) -> DbPool {
    match sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}