//controller/company_controller.rs
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::entity::company::{Company};
use crate::AppState;

pub async fn get_all(State(state): State<Arc<AppState>>) -> Json<Vec<crate::entity::company::Company>> {
    let AppState { company_service, .. } = &*state;
    Json(company_service.get_all().await)
}

pub async fn get_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Json<Option<crate::entity::company::Company>> {
    let AppState { company_service, .. } = &*state;
    Json(company_service.get_by_id(id).await)
}


pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Company>,
) -> Json<i64> {
    Json(state.company_service.create(payload).await)
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<Company>,
) -> Json<&'static str> {
    state.company_service.update(id, payload).await;
    Json("updated")
}

pub async fn delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Json<&'static str> {
    state.company_service.delete(id).await;
    Json("deleted")
}