use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::model::Warrant;

#[derive(Debug, Deserialize)]
pub struct WarrantFilter {
    pub code: Option<String>,
    pub obra: Option<String>,
    pub provider: Option<String>,
    pub expediente: Option<String>,
    pub entidad: Option<String>,
    pub warrant_type: Option<Vec<i32>>,
    pub faltan: Option<i32>,
    pub danger: Option<bool>,
    pub fecha_ini: Option<NaiveDate>,
    pub fecha_fin: Option<NaiveDate>,
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub order: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SaveWarrant {
    pub expediente: Option<i32>,
    pub nro_carta: Option<String>,
    pub obra: Option<String>,
    pub proveedor: Option<String>,
    pub entidad: Option<String>,
    pub warrant_type_id: Option<i32>,
    pub process_type: Option<String>,
    pub fecha_vencimiento: Option<NaiveDate>,
    pub fecha_renovacion: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct PagedWarrants {
    pub data: Vec<Warrant>,
    pub size: i64,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub count: i64,
    pub message: String,
    pub url: String,
}
