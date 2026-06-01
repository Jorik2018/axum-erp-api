use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: i8,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct RegionModel {
    #[sqlx(rename = "id_dpto")]
    pub id: Option<i32>, //column db = codigo_dpto
    #[sqlx(rename = "codigo_dpto")]
    pub code: Option<String>, //column db = codigo_dpto
    #[sqlx(rename = "nombre_dpto")]
    pub name: Option<String>, // column = nombre_dpto
    #[sqlx(rename = "abreviatura_dpto")]
    pub abbreviation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct ProvinceModel {
    #[sqlx(rename = "id_pais")]
    pub countryId: Option<i32>,
    #[sqlx(rename = "id_dpto")]
    pub regionId: Option<i32>,
    #[sqlx(rename = "id_prov")]
    pub id: Option<i32>,
    #[sqlx(rename = "nombre_prov")]
    pub name: Option<String>,
    #[sqlx(rename = "abreviatura_prov")]
    pub abbreviation: Option<String>,
    #[sqlx(rename = "codigo_prov")]
    pub code: Option<String>,
    #[sqlx(rename = "government_id")]
    pub governmentId: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
//#[allow(non_snake_case)]
pub struct DistrictModel {
    #[sqlx(rename = "id_pais")]
    pub countryId: Option<i32>,
    #[sqlx(rename = "id_dpto")]
    pub regionId: Option<i32>,
    #[sqlx(rename = "id_prov")]
    pub provinceId: Option<i32>,
    #[sqlx(rename = "id_distrito")]
    pub id: Option<i32>,
    #[sqlx(rename = "nombre_dist")]
    pub name: Option<String>,
    #[sqlx(rename = "abreviatura_dist")]
    pub abbreviation: Option<String>,
    #[sqlx(rename = "id_ubg")]
    pub ubigeoId: Option<i32>,
    #[sqlx(rename = "codigo_dist")]
    pub code: Option<String>,
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModelResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}