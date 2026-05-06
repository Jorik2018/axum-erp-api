use sqlx::MySqlPool;
use crate::entity::company::{Company};

pub struct CompanyRepository {
    pub pool: MySqlPool,
}

impl CompanyRepository {

    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Company>, sqlx::Error> {
        sqlx::query_as::<_, Company>(
            r#"SELECT id, name, address FROM company"#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Company>, sqlx::Error> {
        sqlx::query_as::<_, Company>(
        //sqlx::query_as!(
        //    Company,
            r#"SELECT id, name, address FROM company WHERE id = ?"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create(&self, data: Company) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"INSERT INTO company (name, address) VALUES (?, ?)"#
        )
        .bind(data.name)
        .bind(data.address)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_id() as i64)
    }

    pub async fn update(&self, id: i64, data: Company) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE company
            SET
                name = COALESCE(?, name),
                address = COALESCE(?, address)
            WHERE id = ?
            "#
        )
        .bind(data.name)
        .bind(data.address)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"DELETE FROM company WHERE id = ?"#
        )
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}