use axum::{extract::{Path, Query, State}, routing::{delete, get, post, put}, Json, Router};
use serde_json::{json, Value};

use crate::{auth::AuthUser, error::ApiError, AppState};
use super::{dto::{NotificationResponse, SaveWarrant, WarrantFilter}, repository};

pub fn warrant_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/max-expediente", get(max_expediente))
        .route("/notifications", get(notifications))
        .route("/{id}", get(find).put(update).delete(remove))
}

async fn list(
    State(state): State<AppState>,
    AuthUser(_claims): AuthUser,
    Query(filter): Query<WarrantFilter>,
) -> Result<Json<super::dto::PagedWarrants>, ApiError> {
    Ok(Json(repository::list(&state.db, &filter).await?))
}

async fn find(
    State(state): State<AppState>,
    AuthUser(_claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<super::model::Warrant>, ApiError> {
    Ok(Json(repository::find_by_id(&state.db, id).await?))
}

async fn create(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(input): Json<SaveWarrant>,
) -> Result<Json<super::model::Warrant>, ApiError> {
    require_tesoreria(&claims)?;
    Ok(Json(repository::create(&state.db, input).await?))
}

async fn update(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
    Json(input): Json<SaveWarrant>,
) -> Result<Json<super::model::Warrant>, ApiError> {
    require_tesoreria(&claims)?;
    Ok(Json(repository::update(&state.db, id, input).await?))
}

async fn remove(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    require_tesoreria(&claims)?;
    repository::delete(&state.db, id).await?;
    Ok(Json(json!({ "ok": true })))
}

async fn max_expediente(
    State(state): State<AppState>,
    AuthUser(_claims): AuthUser,
) -> Result<Json<Value>, ApiError> {
    let value = repository::max_expediente(&state.db).await?;
    Ok(Json(json!({ "maxExpediente": value })))
}

async fn notifications(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<Json<NotificationResponse>, ApiError> {
    require_tesoreria(&claims)?;
    let count = repository::notification_count(&state.db).await?;
    Ok(Json(NotificationResponse {
        count,
        message: format!("hay {} cartas fianzas por vencer", count),
        url: "/admin/tesoreria/cartaFianza".to_string(),
    }))
}

fn require_tesoreria(claims: &crate::auth::Claims) -> Result<(), ApiError> {
    if claims.has_group("ACCESS_TESORERIA") {
        Ok(())
    } else {
        Err(ApiError::Forbidden)
    }
}
