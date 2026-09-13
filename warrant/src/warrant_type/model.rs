use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct WarrantType {
    pub id: i32,
    pub name: String,
}
