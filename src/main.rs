mod db;
mod entity;
mod repository;
mod service;
mod controller;

use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use controller::company_controller::*;
use repository::company_repository::CompanyRepository;
use service::company_service::CompanyService;

#[tokio::main]
async fn main() {

    let database_url = "mysql://root:ADMIN@localhost:3306/erp";

    let pool = db::create_pool(database_url).await;

    let repo = CompanyRepository::new(pool);
    
    let service = Arc::new(CompanyService::new(repo));

    let app = Router::new()
        .route("/companies", get(get_all).post(create))
        .route("/companies/{id}", get(get_by_id).put(update).delete(delete))
        .with_state(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on :3000");

    axum::serve(listener, app).await.unwrap();
}