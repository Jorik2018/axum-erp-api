use deadpool_redis::Pool;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct SessionService {
    pub pool: Pool,
    ttl_seconds: u64,
}

impl SessionService {
    pub fn new(pool: Pool) -> Self {
        Self {
            pool,
            ttl_seconds: 120,
        }
    }

    async fn conn(&self) -> Result<deadpool_redis::Connection, String> {
        self.pool.get().await.map_err(|e| e.to_string())
    }

    pub async fn put(&self, key: &str, value: String) -> Result<(), String> {
        let mut conn = self.conn().await?;
        let _: () = conn
            .set_ex(key, value, self.ttl_seconds)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    // GET (sliding TTL)
    pub async fn get(&self, key: &str) -> Result<Option<String>, String> {
        let mut conn = self.conn().await?;

        let result: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| e.to_string())?;

        if result.is_some() {
            let _: bool = conn
                .expire(key, self.ttl_seconds as i64)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // 👈 para no correr siempre
    async fn test_put_get() {
        let redis_url = "redis://127.0.0.1/";
        let pool = crate::session::create_redis_pool(redis_url).await;

        let service = SessionService::new(pool);

        service.put("test:key", "hola".to_string()).await.unwrap();

        let value = service.get("test:key").await.unwrap();

        assert_eq!(value, Some("hola".to_string()));
    }
}
/*      let mut conn = self.pool.get().await.map_err(|e| {
            eprintln!("Failed to get Redis connection: {}", e);
            e.to_string()
        })?; */