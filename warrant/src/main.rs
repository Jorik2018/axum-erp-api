mod auth;
mod error;
mod warrant;
mod warrant_type;

use std::{env, net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use sqlx::mysql::MySqlPoolOptions;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use auth::JwtService;
use warrant::routes::warrant_routes;
use warrant_type::routes::warrant_type_routes;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub jwt: Arc<JwtService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = env::var("DATABASE_URL")?;
    let public_key_path = env::var("JWT_PUBLIC_KEY")
        .unwrap_or_else(|_| "keys/publicKey.pem".to_string());
    let issuer = env::var("JWT_ISSUER").ok();

    let db = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    let public_key = std::fs::read(public_key_path)?;
    let jwt = Arc::new(JwtService::new(&public_key, issuer)?);

    let state = AppState { db, jwt };

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest("/api/warrants", warrant_routes())
        .nest("/api/warrant-types", warrant_type_routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()?;

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
