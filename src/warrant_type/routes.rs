use std::sync::Arc;

use axum::{
    extract::State,
    routing::get,
    Json,
    Router,
};

use crate::{
    auth::AuthUser,
    error::ApiError,
    state::AppState,
};

use super::model::WarrantType;

pub fn warrant_type_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{from}/{to}", get(list_range))
}


async fn list_range(
    State(state): State<Arc<AppState>>,
    AuthUser(_claims): AuthUser,
) -> Result<Json<Vec<WarrantType>>, ApiError> {
    let values = sqlx::query_as::<_, WarrantType>(
        "SELECT id, name FROM warrant_type ORDER BY name"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(values))
}