use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{model::{DistrictModel, ProvinceModel, RegionModel}, schema::FilterOptions, AppState};

pub async fn region_list_handler(
    State(data): State<Arc<AppState>>,
    Path((from, limit)): Path<(i32, i32)>,
    Query(_opts): Query<FilterOptions>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let offset = from;
    let base_query =
        r#"SELECT id_dpto, codigo_dpto, nombre_dpto, abreviatura_dpto FROM drt_departamento"#;
    let mut query_builder = sqlx::QueryBuilder::new(base_query);
    if limit > 0 {
        query_builder.push(" LIMIT ? OFFSET ?");
    }
    let mut query = query_builder.build_query_as::<RegionModel>();
    if limit > 0 {
        query = query.bind(limit);
        query = query.bind(offset);
    }
    let notes: Vec<RegionModel> = query.fetch_all(&data.db).await.map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error2",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    if limit == 0 {
        return Ok(Json(serde_json::json!(notes)));
    }
    let total_count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM drt_departamento"#)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let note_responses: Vec<RegionModel> = notes.into_iter().collect();
    let json_response = serde_json::json!({
        "size": total_count,
        "data": note_responses
    });
    Ok(Json(json_response))

    /*if limit == 0 {
        let notes = sqlx::query_as!(
            RegionModel,
            query
        )
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
        let note_responses = notes
        .iter()
        //.map(|note| to_note_response(&note))
        .map(|region| RegionModel {
            code: region.code.clone(),
            name: region.name.clone(),
        })
        .collect::<Vec<RegionModel>>();
    Ok(Json(serde_json::json!(note_responses)))
    } else {
        let notes = sqlx::query_as!(
            RegionModel,
            r#"SELECT codigo_dpto as code, nombre_dpto as name FROM drt_departamento LIMIT ? OFFSET ?"#,
            limit,
            offset as i32
        )
        .fetch_all(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

        // Query to count total regions
        let total_count: i64 = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM drt_departamento"#)
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "error",
                    "message": format!("Database error: { }", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

        let note_responses = notes
            .iter()
            //.map(|note| to_note_response(&note))
            .map(|region| RegionModel {
                code: region.code.clone(),
                name: region.name.clone(),
            })
            .collect::<Vec<RegionModel>>();

        let json_response = serde_json::json!({
            "size": total_count,
            "data": note_responses
        });
        Ok(Json(json_response))
    }*/
}

pub async fn district_list_handler(
    Path((from, limit)): Path<(i32, i32)>,
    Query(_opts): Query<FilterOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let offset = from;
    let base_query =
        r#"SELECT id_pais, id_dpto, id_prov, nombre_prov, abreviatura_prov, codigo_prov, government_id FROM drt_provincia"#;
    let mut query_builder = sqlx::QueryBuilder::new(base_query);
    if limit > 0 {
        query_builder.push(" LIMIT ? OFFSET ?");
    }
    let mut query = query_builder.build_query_as::<ProvinceModel>();
    if limit > 0 {
        query = query.bind(limit);
        query = query.bind(offset);
    }
    let notes: Vec<ProvinceModel> = query.fetch_all(&data.db).await.map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error2",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    if limit == 0 {
        return Ok(Json(serde_json::json!(notes)));
    }
    let total_count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM drt_provincia"#)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    let note_responses: Vec<ProvinceModel> = notes.into_iter().collect();
    let json_response = serde_json::json!({
        "size": total_count,
        "data": note_responses
    });
    Ok(Json(json_response))
}

pub async fn province_list_handler(
    Path((from, limit)): Path<(i32, i32)>,
    Query(_opts): Query<FilterOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let offset = from;
    let base_query =
        r#"SELECT id_pais, id_dpto, id_prov, id_distrito, nombre_dist, abreviatura_dist, id_ubg, codigo_dist FROM drt_distrito"#;
    let mut query_builder = sqlx::QueryBuilder::new(base_query);
    if limit > 0 {
        query_builder.push(" LIMIT ? OFFSET ?");
    }
    let mut query = query_builder.build_query_as::<DistrictModel>();
    if limit > 0 {
        query = query.bind(limit);
        query = query.bind(offset);
    }
    let notes: Vec<DistrictModel> = query.fetch_all(&data.db).await.map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error2",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    if limit == 0 {
        return Ok(Json(serde_json::json!(notes)));
    }
    let total_count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM drt_distrito"#)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    let note_responses: Vec<DistrictModel> = notes.into_iter().collect();
    let json_response = serde_json::json!({
        "size": total_count,
        "data": note_responses
    });
    Ok(Json(json_response))
}