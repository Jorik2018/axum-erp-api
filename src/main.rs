mod auth;
mod db;
mod entity;
mod repository;
mod service;
mod controller;
mod vault;
mod handler;
mod model;
mod schema;
mod state;
mod session;
mod middleware;
mod warrant;
mod warrant_type;
mod error;

use std::{
    env,
    net::SocketAddr,
    sync::Arc,
};
use state::AppState;
use axum::{
    middleware::from_fn_with_state,
    routing::get,
    Router,
};

use axum::http::{
    header::{
        ACCEPT,
        AUTHORIZATION,
        CONTENT_TYPE,
    },
    Method,
};

use sqlx::mysql::MySqlPoolOptions;

use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};

use dotenv::dotenv;

use auth::JwtService;

use controller::company_controller::*;

use handler::note_handler::*;
use handler::region_handler::*;

use repository::company_repository::CompanyRepository;

use service::company_service::CompanyService;

use warrant::routes::warrant_routes;
use warrant_type::routes::warrant_type_routes;

use crate::{
    middleware::auth::auth_middleware,
    service::session_service::SessionService,
    session::create_redis_pool,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .init();

    //
    // Vault
    //
    let vault_addr = env::var("VAULT_ADDR")
        .unwrap_or_else(|_| {
            "http://127.0.0.1:8200".to_string()
        });

    let vault_token = env::var("VAULT_TOKEN")
        .expect("VAULT_TOKEN must be set");

    //
    // Database from Vault
    //
    let database_url = vault::get_secret(
        &vault_addr,
        &vault_token,
        "global",
        "DATABASE_URL",
    )
    .await?;

    let db = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    //
    // Redis from Vault
    //
    let redis_url = vault::get_secret(
        &vault_addr,
        &vault_token,
        "global",
        "REDIS_URL",
    )
    .await?;

    let redis_pool = create_redis_pool(
        &redis_url
    )
    .await;

    let session_service =
        SessionService::new(redis_pool);

    //
    // JWT
    //
    // Por ahora desde archivo/env.
    // Luego lo pasas a Vault también.
    //
    let public_key_path = env::var("JWT_PUBLIC_KEY")
        .unwrap_or_else(|_| {
            "keys/publicKey.pem".to_string()
        });

    let issuer = env::var("JWT_ISSUER").ok();

    let public_key = std::fs::read(
        public_key_path
    )?;

    let jwt = Arc::new(
        JwtService::new(
            &public_key,
            issuer,
        )?
    );

    //
    // Company service
    //
    let repo = CompanyRepository::new(
        db.clone()
    );

    let company_service = Arc::new(
        CompanyService::new(repo)
    );

    //
    // App state
    //
    let app_state = Arc::new(
        AppState {
            db: db.clone(),
            jwt,
            company_service,
            session_service,
        }
    );

    //
    // CORS
    //
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_credentials(true)
        .allow_headers([
            AUTHORIZATION,
            ACCEPT,
            CONTENT_TYPE,
        ]);

    //
    // Protected routes
    //
    let protected_routes = Router::new()
        .route(
            "/companies",
            get(get_all)
                .post(create),
        )
        .layer(
            from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )
        );

    //
    // Main router
    //
    let app = Router::new()
        .merge(protected_routes)

        .route(
            "/companies/{id}",
            get(get_by_id)
                .put(update)
                .delete(delete),
        )

        .route(
            "/health",
            get(|| async { "ok" }),
        )

        .route(
            "/healthcheck",
            get(health_check_handler),
        )

        .nest(
            "/notes",
            Router::new()
                .route(
                    "/",
                    get(note_list_handler)
                        .post(create_note_handler),
                )
                .route(
                    "/{id}",
                    get(get_note_handler)
                        .patch(edit_note_handler)
                        .delete(delete_note_handler),
                ),
        )

        .nest(
            "/region",
            Router::new()
                .route(
                    "/{from}/{limit}",
                    get(region_list_handler),
                ),
        )

        .nest(
            "/province",
            Router::new()
                .route(
                    "/{from}/{limit}",
                    get(province_list_handler),
                ),
        )

        .nest(
            "/district",
            Router::new()
                .route(
                    "/{from}/{limit}",
                    get(district_list_handler),
                ),
        )

        //
        // Sin /api porque Nginx ya lo pone
        //
        .nest(
            "/warrants",
            warrant_routes(),
        )

        .nest(
            "/warrant-types",
            warrant_type_routes(),
        )

        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
        )
        .with_state(app_state);

    //
    // Server
    //
    let host = env::var("HOST")
        .unwrap_or_else(|_| {
            "0.0.0.0".to_string()
        });

    let port = env::var("PORT")
        .unwrap_or_else(|_| {
            "3000".to_string()
        })
        .parse::<u16>()?;

    let addr: SocketAddr =
        format!("{host}:{port}")
            .parse()?;

    let listener =
        tokio::net::TcpListener::bind(
            addr
        )
        .await?;

    tracing::info!(
        "Server listening on {}",
        addr
    );

    axum::serve(
        listener,
        app,
    )
    .await?;

    Ok(())
}