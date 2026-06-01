//src/bin/test_redis.rs
use std::env;

use axum_erp_api::{service::session_service::SessionService, session::create_redis_pool};

// 👇 ajusta esto según tu crate
//unresolved import `axum_erp_api`
//if you wanted to use a crate named `axum_erp_api`, use `cargo add axum_erp_api` to add it to your `Cargo.toml

#[tokio::main]
async fn main() {
    // 👉 mejor usar variable de entorno
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    println!("🔌 Connecting to Redis...");

    let redis_pool = create_redis_pool(&redis_url).await;
    let session_service = SessionService::new(redis_pool);

    // PUT
    session_service
        .put("test:key", "hola desde bin".to_string())
        .await
        .expect("❌ Error saving");

    println!("✅ Saved");

    // GET
    let value = session_service
        .get("test:key")
        .await
        .expect("❌ Error getting");

    println!("📦 Value: {:?}", value);
}