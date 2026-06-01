mod db;
mod entity;
mod repository;
mod service;
mod controller;
mod handler;
mod model;
mod schema;
mod state;
mod session;
mod middleware;
use axum::middleware::from_fn_with_state;
use std::sync::Arc;
use axum::{
    routing::{get},
    Router,
};
use axum::http::{
    header::{CONTENT_TYPE, AUTHORIZATION, ACCEPT}, 
    Method
};
use tower_http::cors::{CorsLayer};
use dotenv::dotenv;
use state::AppState;
use controller::company_controller::*;
use handler::note_handler::*;
use handler::region_handler::*;
use repository::company_repository::CompanyRepository;
use service::company_service::CompanyService;
use crate::middleware::auth::auth_middleware;
use crate::{service::session_service::SessionService, session::create_redis_pool};

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url =std::env::var("DATABASE_URL").expect("DATABASE_URL must set");

    let pool = db::create_pool(&database_url).await;

    let repo = CompanyRepository::new(pool.clone());
    
    let service = Arc::new(CompanyService::new(repo));

    let redis_url = "redis://default:VGzseUUNTiwcpycer1cQnHyR8YICn3Xv@redis-16106.crce219.us-east-1-4.ec2.cloud.redislabs.com:16106";
    
    let redis_pool = create_redis_pool(redis_url).await;

    let session_service = SessionService::new(redis_pool);

    let app_state = Arc::new(AppState {
        company_service: service,
        session_service,
        db: pool
    });

    let cors = CorsLayer::new()
        //.allow_origin(Any)
        //.allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let protected_routes = Router::new()
        .route("/companies", get(get_all).post(create))
        .layer(from_fn_with_state(app_state.clone(), auth_middleware));

    let app = Router::new()
        .merge(protected_routes)
        .route("/companies/{id}", get(get_by_id).put(update).delete(delete))
        .route("/healthcheck", get(health_check_handler))
        .nest("/notes",
            Router::new()
                .route("/", get(note_list_handler).post(create_note_handler))
                .route(
                    "/{id}",
                    get(get_note_handler)
                        .patch(edit_note_handler)
                        .delete(delete_note_handler),
                ),
        )
        
        .nest("/region",
            Router::new().route("/{from}/{limit}", get(region_list_handler)),
        )
        .nest("/province",
            Router::new().route("/{from}/{limit}", get(province_list_handler)),
        )
        .nest("/district",
            Router::new().route("/{from}/{limit}", get(district_list_handler)),
        )
        .with_state(app_state).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on :3000");

    axum::serve(listener, app).await.unwrap();

}