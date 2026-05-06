use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub address: String,
}