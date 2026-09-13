use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Warrant {
    pub id: i64,
    pub expediente: Option<i32>,
    pub numero: Option<i32>,
    pub nro_carta: Option<String>,
    pub obra: Option<String>,
    pub proveedor: Option<String>,
    pub entidad: Option<String>,
    pub warrant_type_id: Option<i32>,
    pub process_type: Option<String>,
    pub fecha_registro: Option<NaiveDateTime>,
    pub fecha_vencimiento: Option<NaiveDateTime>,
    pub fecha_renovacion: Option<NaiveDateTime>,
    pub canceled: bool,
    pub renovated: bool,
    pub diff: Option<i32>,
}