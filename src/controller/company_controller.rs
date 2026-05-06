use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::service::company_service::CompanyService;
use crate::entity::company::{Company};

pub type AppState = Arc<CompanyService>;

pub async fn get_all(State(service): State<AppState>) -> Json<Vec<crate::entity::company::Company>> {
    Json(service.get_all().await)
}

pub async fn get_by_id(
    State(service): State<AppState>,
    Path(id): Path<i64>,
) -> Json<Option<crate::entity::company::Company>> {
    Json(service.get_by_id(id).await)
}

pub async fn create(
    State(service): State<AppState>,
    Json(payload): Json<Company>,
) -> Json<i64> {
    Json(service.create(payload).await)
}

pub async fn update(
    State(service): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<Company>,
) -> Json<&'static str> {
    service.update(id, payload).await;
    Json("updated")
}


pub async fn delete(
    State(service): State<AppState>,
    Path(id): Path<i64>,
) -> Json<&'static str> {
    service.delete(id).await;
    Json("deleted")
}