use super::{
    dto::{NotificationResponse, SaveWarrant, WarrantFilter},
    repository,
};
use crate::{auth::AuthUser, error::ApiError, state::AppState};
use axum::{
    Json, Router,
    body::Body,
    extract::{Path, Query, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::NaiveDate;
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
        .route("/create/{id}", get(create_template))
        .route("/max-expediente", get(max_expediente))
        .route("/notifications", get(notifications))
        .route("/{from}/{to}", get(list_range))
        .route("/{id}", get(find).put(update).delete(remove))
        .route("/download", post(download))
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
        warrant_type: None,
        total: None,
        fecha_renovacion: None,
        nro_carta: None,
        fecha_emision: None,
        fecha_vencimiento: None,
        fecha_registro: None,
        observacion: None,
        confirmacion_banco: None,
        renovated: Some(false),
        status: Some(true),
        canceled: false,
        upload: Some(false),
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

async fn download(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Json(input): Json<Value>,
) -> Result<impl IntoResponse, ApiError> {
    require_tesoreria(&claims)?;

    let format = input
        .get("FORMAT")
        .and_then(Value::as_str)
        .unwrap_or("json");

    let group = input
        .get("group")
        .and_then(|value| {
            value
                .as_str()
                .and_then(|value| value.parse::<i32>().ok())
                .or_else(|| value.as_i64().map(|value| value as i32))
        })
        .unwrap_or(0);

    let option = input
        .get("option")
        .and_then(|value| {
            value
                .as_str()
                .and_then(|value| value.parse::<i32>().ok())
                .or_else(|| value.as_i64().map(|value| value as i32))
        })
        .unwrap_or(0);

    let fecha_ini = input
        .get("FECHA_INI")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(|value| {
            NaiveDate::parse_from_str(value, "%d/%m/%Y")
                .map_err(|_| ApiError::BadRequest(format!("FECHA_INI inválida: {value}")))
        })
        .transpose()?;

    let fecha_fin = input
        .get("FECHA_FIN")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(|value| {
            NaiveDate::parse_from_str(value, "%d/%m/%Y")
                .map_err(|_| ApiError::BadRequest(format!("FECHA_FIN inválida: {value}")))
        })
        .transpose()?;

    let danger = input
        .get("danger")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let order = input
        .get("order")
        .and_then(Value::as_str)
        .map(str::to_string);

    let title_report = input
        .get("TITLE_REPORT")
        .and_then(Value::as_str)
        .unwrap_or("REPORTE DE CARTAS FIANZAS");

    let filter = WarrantFilter {
        code: None,
        obra: None,
        provider: None,
        expediente: None,
        entidad: None,
        warrant_type: None,

        faltan: None,

        danger: Some(danger),

        fecha_ini,
        fecha_fin,

        page: None,
        size: None,

        order: order.clone(),
    };

    let result = repository::list_range(&state.db, &filter, 0, 0).await?;

    let report_name = match group {
        1 => "cartaFianza_x_expediente",

        2 => "cartaFianza_x_proveedor",

        _ => match option {
            1 => "cartaFianza_1",

            _ => "cartaFianza",
        },
    };

    let output = json!({

        "report": report_name,

        "parameters": {

            "FORMAT": format,

            "FECHA_INI": fecha_ini
                .map(|value| value.format("%d/%m/%Y").to_string()),

            "FECHA_FIN": fecha_fin
                .map(|value| value.format("%d/%m/%Y").to_string()),

            "TITLE_REPORT": title_report,

            "danger": danger,

            "order": order,

            "group": group,

            "option": option,

            "IS_ONE_PAGE_PER_SHEET": false,

            "SIGN_SECTION": true,

            "rest": true
        },

        "data": result.data
    });

    let content = serde_json::to_string_pretty(&output)
        .map_err(|error| ApiError::BadRequest(format!("error serializando reporte: {error}")))?;

    let filename = format!("{report_name}.json");

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        )
        .body(Body::from(content))
        .map_err(|error| ApiError::BadRequest(format!("error construyendo respuesta: {error}")))?;

    Ok(response)
}

async fn list_range(
    State(state): State<Arc<AppState>>,
    AuthUser(_claims): AuthUser,
    Path((from, to)): Path<(u64, u64)>,
    Query(filter): Query<WarrantFilter>,
) -> Result<Json<super::dto::PagedWarrants>, ApiError> {
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
    //if claims.has_group("ACCESS_TESORERIA") {
    Ok(())
    //} else {
    //  Err(ApiError::Forbidden)
    //}
}
