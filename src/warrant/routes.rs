use super::{
    dto::{NotificationResponse, SaveWarrant, WarrantFilter},
    repository,
};
use crate::{auth::AuthUser, error::ApiError, state::AppState};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde_json::{Value, json};
use std::sync::Arc;
use unicode_normalization::UnicodeNormalization;

fn simplify_file_name(input: &str) -> String {
    let ascii: String = input
        .trim()
        .nfd()
        .filter(|c| c.is_ascii())
        .filter(|c| *c != '*')
        .collect();

    ascii.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn warrant_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/create/{id}", get(create_template))
        .route("/max-expediente", get(max_expediente))
        .route("/notifications", get(notifications))
        .route("/{from}/{to}", get(list_range))
        .route("/{id}", get(find).put(update).delete(remove))
}

async fn create_template(
    State(state): State<Arc<AppState>>,
    AuthUser(_claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<super::model::Warrant>, ApiError> {
    let mut warrant = super::model::Warrant {
        id: 0,
        provider_id: None,
        process_type: None,
        extension: None,
        proveedor: None,
        expediente: None,
        entidad: None,
        numero: None,
        obra: None,
        warrant_type_id: None,
        total: None,
        fecha_renovacion: None,
        nro_carta: None,
        fecha_emision: None,
        fecha_vencimiento: None,
        fecha_registro: None,
        observacion: None,
        confirmacion_banco: None,
        renovated: None,
        status: None,
        canceled: false,
        upload: None,
        diff: None,
        ext: None,
    };

    if id > 0 {
        let existing = repository::find_by_id(&state.db, id).await?;

        warrant.expediente = existing.expediente;
        warrant.proveedor = existing.proveedor;
        warrant.process_type = existing.process_type;
        warrant.entidad = existing.entidad;
        warrant.obra = existing.obra;
    }

    if warrant.expediente.is_none() {
        let max = repository::max_expediente(&state.db).await?;

        warrant.expediente = Some(max + 1);
    }

    warrant.ext = Some(super::model::WarrantExt { src: String::new() });

    Ok(Json(warrant))
}

async fn list(
    State(state): State<Arc<AppState>>,
    AuthUser(_claims): AuthUser,
    Query(filter): Query<WarrantFilter>,
) -> Result<Json<super::dto::PagedWarrants>, ApiError> {
    Ok(Json(repository::list(&state.db, &filter).await?))
}

async fn list_range(
    State(state): State<Arc<AppState>>,
    AuthUser(_claims): AuthUser,
    Path((from, to)): Path<(u64, u64)>,
    Query(filter): Query<WarrantFilter>,
) -> Result<Json<super::dto::PagedWarrants>, ApiError> {
    if to <= from {
        return Err(ApiError::BadRequest(
            "`to` must be greater than `from`".into(),
        ));
    }

    let mut result = repository::list_range(&state.db, &filter, from, to).await?;

    for warrant in &mut result.data {
        if warrant.upload.unwrap_or(false) {
            warrant.ext = Some(super::model::WarrantExt {
                src: get_file_name(warrant),
            });
        }
    }

    Ok(Json(result))
}

fn get_file_name(warrant: &super::model::Warrant) -> String {
    let id = warrant.id;

    let expediente = warrant
        .expediente
        .map(|v| v.to_string())
        .unwrap_or_default();

    let nro_carta = warrant.nro_carta.as_deref().unwrap_or_default();

    let extension = warrant.extension.as_deref().unwrap_or_default();

    let filename = format!("CF-{id:04}-{expediente}-{nro_carta}");

    let filename = simplify_file_name(&filename);

    if extension.is_empty() {
        filename
    } else {
        format!("{filename}.{extension}")
    }
}

async fn find(
    State(state): State<Arc<AppState>>,
    AuthUser(_claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<super::model::Warrant>, ApiError> {
    let mut warrant = repository::find_by_id(&state.db, id).await?;

    warrant.ext = Some(super::model::WarrantExt {
        src: get_file_name(&warrant),
    });

    Ok(Json(warrant))
}

async fn create(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Json(input): Json<SaveWarrant>,
) -> Result<Json<super::model::Warrant>, ApiError> {
    require_tesoreria(&claims)?;
    Ok(Json(repository::create(&state.db, input).await?))
}

async fn update(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
    Json(input): Json<SaveWarrant>,
) -> Result<Json<super::model::Warrant>, ApiError> {
    require_tesoreria(&claims)?;
    Ok(Json(repository::update(&state.db, id, input).await?))
}

async fn remove(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    require_tesoreria(&claims)?;
    repository::delete(&state.db, id).await?;
    Ok(Json(json!({ "ok": true })))
}

async fn max_expediente(
    State(state): State<Arc<AppState>>,
    AuthUser(_claims): AuthUser,
) -> Result<Json<Value>, ApiError> {
    let value = repository::max_expediente(&state.db).await?;
    Ok(Json(json!({ "maxExpediente": value })))
}

async fn notifications(
    State(state): State<Arc<AppState>>,
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
