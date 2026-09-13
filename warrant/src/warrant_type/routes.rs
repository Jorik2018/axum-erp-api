use axum::{extract::State, routing::get, Json, Router};

use crate::{auth::AuthUser, error::ApiError, AppState};
use super::model::WarrantType;

pub fn warrant_type_routes() -> Router<AppState> {
    Router::new().route("/", get(list))
}

async fn list(
    State(state): State<AppState>,
    AuthUser(_claims): AuthUser,
) -> Result<Json<Vec<WarrantType>>, ApiError> {
    let values = sqlx::query_as::<_, WarrantType>(
        "SELECT id, name FROM warrant_type ORDER BY name"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(values))
}
