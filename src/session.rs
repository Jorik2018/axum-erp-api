use deadpool_redis::{Config, Pool, Runtime};

pub async fn create_redis_pool(redis_url: &str) -> Pool {
    let cfg = Config::from_url(redis_url);
    cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Cannot create Redis pool")
}