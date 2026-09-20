use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sqlx::FromRow;

use crate::warrant_type::model::WarrantType;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Warrant {
    pub id: i64,

    pub provider_id: Option<i32>,

    pub process_type: Option<String>,

    pub extension: Option<String>,

    pub proveedor: Option<String>,

    pub expediente: Option<i32>,

    pub entidad: Option<String>,

    pub numero: Option<i32>,

    pub obra: Option<String>,

    pub warrant_type_id: Option<i32>,

    pub total: Option<f64>,

    pub fecha_renovacion: Option<NaiveDateTime>,

    pub nro_carta: Option<String>,

    pub fecha_emision: Option<NaiveDateTime>,

    pub fecha_vencimiento: Option<NaiveDateTime>,

    pub fecha_registro: Option<NaiveDateTime>,

    pub observacion: Option<String>,

    pub confirmacion_banco: Option<String>,

    pub renovated: Option<bool>,

    pub status: Option<bool>,

    pub canceled: bool,

    pub upload: Option<bool>,

    pub diff: Option<i64>,

    #[sqlx(skip)]
    pub warrant_type: Option<WarrantType>,
    
    #[sqlx(skip)]
    pub ext: Option<WarrantExt>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WarrantExt {
    pub src: String,
}