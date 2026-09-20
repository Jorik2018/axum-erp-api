use chrono::NaiveDateTime;
use sqlx::{MySql, MySqlPool, QueryBuilder, prelude::FromRow};

use super::{
    dto::{PagedWarrants, SaveWarrant, WarrantFilter},
    model::Warrant,
};
use crate::{error::ApiError, warrant_type::model::WarrantType};

pub async fn list(pool: &MySqlPool, f: &WarrantFilter) -> Result<PagedWarrants, ApiError> {
    let mut where_sql = QueryBuilder::<MySql>::new(" WHERE canceled = 0 ");

    if let Some(expediente) = &f.expediente {
        where_sql
            .push(" AND LPAD(expediente, 3, '0') LIKE ")
            .push_bind(format!("%{}%", expediente.replace(' ', "%")));
    }
    if let Some(entidad) = &f.entidad {
        where_sql
            .push(" AND UPPER(entidad) LIKE ")
            .push_bind(format!("%{}%", entidad.to_uppercase().replace(' ', "%")));
    }
    if let Some(obra) = &f.obra {
        where_sql
            .push(" AND UPPER(obra) LIKE ")
            .push_bind(format!("%{}%", obra.to_uppercase().replace(' ', "%")));
    }
    if let Some(code) = &f.code {
        where_sql
            .push(" AND UPPER(nro_carta) LIKE ")
            .push_bind(format!("%{}%", code.to_uppercase().replace(' ', "%")));
    }
    if let Some(provider) = &f.provider {
        where_sql
            .push(" AND UPPER(proveedor) LIKE ")
            .push_bind(format!("%{}%", provider.to_uppercase().replace(' ', "%")));
    }
    if let Some(ids) = &f.warrant_type {
        if !ids.is_empty() {
            where_sql.push(" AND warrant_type_id IN (");
            let mut separated = where_sql.separated(", ");
            for id in ids {
                separated.push_bind(id);
            }
            separated.push_unseparated(")");
        }
    }
    if let Some(d) = f.fecha_ini {
        where_sql
            .push(" AND DATE(fecha_vencimiento) >= ")
            .push_bind(d);
    }
    if let Some(d) = f.fecha_fin {
        where_sql
            .push(" AND DATE(fecha_vencimiento) <= ")
            .push_bind(d);
    }

    let danger = f.danger.unwrap_or(false);
    match (danger, f.faltan.unwrap_or(0)) {
        (true, _) | (_, 1) => {
            where_sql.push(" AND DATEDIFF(fecha_vencimiento, CURDATE()) BETWEEN 1 AND 5 ");
        }
        (_, x) if x < 0 => {
            where_sql.push(" AND DATEDIFF(fecha_vencimiento, CURDATE()) <= 0 ");
        }
        _ => {}
    }

    let where_fragment = where_sql.sql().to_string();

    let mut qb = QueryBuilder::<MySql>::new(
        "SELECT id, expediente, numero, nro_carta, obra, proveedor, entidad, warrant_type_id, process_type, fecha_registro, fecha_vencimiento, fecha_renovacion, canceled, renovated, DATEDIFF(fecha_vencimiento, CURDATE()) AS diff FROM warrant",
    );

    // Rebuild filters so bind values are attached to this query.
    apply_filters(&mut qb, f);

    if f.order.as_deref() == Some("e") {
        qb.push(" ORDER BY expediente DESC, fecha_vencimiento DESC ");
    } else {
        qb.push(" ORDER BY fecha_vencimiento DESC ");
    }

    let size = f.size.unwrap_or(50).min(500);
    let page = f.page.unwrap_or(0);
    qb.push(" LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind(page * size);

    let data = qb.build_query_as::<Warrant>().fetch_all(pool).await?;

    let mut count_qb = QueryBuilder::<MySql>::new("SELECT COUNT(*) FROM warrant");
    apply_filters(&mut count_qb, f);
    let total: i64 = count_qb.build_query_scalar().fetch_one(pool).await?;

    let _ = where_fragment;
    Ok(PagedWarrants { data, size: total })
}

pub async fn list_range(
    pool: &MySqlPool,
    f: &WarrantFilter,
    from: u64,
    to: u64,
) -> Result<PagedWarrants, ApiError> {
    let mut qb = QueryBuilder::<MySql>::new(
        r#"
        SELECT
            w.id,
            w.provider_id,
            w.process_type,
            w.extension,
            w.proveedor,
            w.expediente,
            w.entidad,
            w.numero,
            w.obra,
            w.warrant_type_id,
            w.total,
            w.fecha_renovacion,
            w.nro_carta,
            w.fecha_emision,
            w.fecha_vencimiento,
            w.fecha_registro,
            w.observacion,
            w.confirmacion_banco,
            w.renovated,
            w.status,
            w.canceled,
            w.upload,

            DATEDIFF(
                w.fecha_vencimiento,
                CURDATE()
            ) AS diff,

            wt.id AS wt_id,
            wt.name AS wt_name

        FROM warrant w

        LEFT JOIN warrant_type wt
            ON wt.id = w.warrant_type_id
        "#,
    );

    apply_filters(&mut qb, f);

    match f.order.as_deref() {
        Some("e") => {
            qb.push(
                " ORDER BY w.expediente DESC, w.fecha_vencimiento DESC "
            );
        }

        _ => {
            qb.push(
                " ORDER BY w.fecha_vencimiento DESC "
            );
        }
    }

qb.push(" LIMIT ")
    .push_bind(to)
    .push(" OFFSET ")
    .push_bind(from);

    let rows = qb
        .build_query_as::<WarrantRow>()
        .fetch_all(pool)
        .await?;

    let data = rows
        .into_iter()
        .map(Warrant::from)
        .collect();

    let mut count_qb =
        QueryBuilder::<MySql>::new(
            "SELECT COUNT(*) FROM warrant w"
        );

    apply_filters(&mut count_qb, f);

    let total: i64 = count_qb
        .build_query_scalar()
        .fetch_one(pool)
        .await?;

    Ok(PagedWarrants {
        data,
        size: total,
    })
}

fn apply_filters<'a>(qb: &mut QueryBuilder<'a, MySql>, f: &'a WarrantFilter) {
    qb.push(" WHERE canceled = 0 ");

    if let Some(expediente) = &f.expediente {
        qb.push(" AND LPAD(expediente, 3, '0') LIKE ")
            .push_bind(format!("%{}%", expediente.replace(' ', "%")));
    }
    if let Some(entidad) = &f.entidad {
        qb.push(" AND UPPER(entidad) LIKE ")
            .push_bind(format!("%{}%", entidad.to_uppercase().replace(' ', "%")));
    }
    if let Some(obra) = &f.obra {
        qb.push(" AND UPPER(obra) LIKE ")
            .push_bind(format!("%{}%", obra.to_uppercase().replace(' ', "%")));
    }
    if let Some(code) = &f.code {
        qb.push(" AND UPPER(nro_carta) LIKE ")
            .push_bind(format!("%{}%", code.to_uppercase().replace(' ', "%")));
    }
    if let Some(provider) = &f.provider {
        qb.push(" AND UPPER(proveedor) LIKE ")
            .push_bind(format!("%{}%", provider.to_uppercase().replace(' ', "%")));
    }
    if let Some(ids) = &f.warrant_type {
        if !ids.is_empty() {
            qb.push(" AND warrant_type_id IN (");
            let mut separated = qb.separated(", ");
            for id in ids {
                separated.push_bind(id);
            }
            separated.push_unseparated(")");
        }
    }
    if let Some(d) = f.fecha_ini {
        qb.push(" AND DATE(fecha_vencimiento) >= ").push_bind(d);
    }
    if let Some(d) = f.fecha_fin {
        qb.push(" AND DATE(fecha_vencimiento) <= ").push_bind(d);
    }

    let danger = f.danger.unwrap_or(false);
    match (danger, f.faltan.unwrap_or(0)) {
        (true, _) | (_, 1) => {
            qb.push(" AND DATEDIFF(fecha_vencimiento, CURDATE()) BETWEEN 1 AND 5 ");
        }
        (_, x) if x < 0 => {
            qb.push(" AND DATEDIFF(fecha_vencimiento, CURDATE()) <= 0 ");
        }
        _ => {}
    }
}

pub async fn find_by_id(pool: &MySqlPool, id: i64) -> Result<Warrant, ApiError> {
    let item = sqlx::query_as::<_, Warrant>(
        r#"
        SELECT
            id,
            provider_id,
            process_type,
            extension,
            proveedor,
            expediente,
            entidad,
            numero,
            obra,
            warrant_type_id,
            total,
            fecha_renovacion,
            nro_carta,
            fecha_emision,
            fecha_vencimiento,
            fecha_registro,
            observacion,
            confirmacion_banco,
            renovated,
            status,
            canceled,
            upload,
            DATEDIFF(fecha_vencimiento, CURDATE()) AS diff
        FROM warrant
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(item)
}

pub async fn create(pool: &MySqlPool, input: SaveWarrant) -> Result<Warrant, ApiError> {
    let mut tx = pool.begin().await?;

    // Equivalente a XUtil.intValue(entity.getExpediente())
    let expediente = input.expediente.unwrap_or(0);

    let numero: i32 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(MAX(numero), 0) + 1
        FROM warrant
        WHERE expediente = ?
        FOR UPDATE
        "#,
    )
    .bind(expediente)
    .fetch_one(&mut *tx)
    .await?;

    let result = sqlx::query(
        r#"
        INSERT INTO warrant (
            expediente,
            numero,
            nro_carta,
            obra,
            proveedor,
            entidad,
            warrant_type_id,
            process_type,
            fecha_registro,
            fecha_vencimiento,
            fecha_renovacion,
            canceled,
            renovated
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, NOW(), ?, ?, 0, 0)
        "#,
    )
    .bind(input.expediente)
    .bind(numero)
    .bind(&input.nro_carta)
    .bind(&input.obra)
    .bind(&input.proveedor)
    .bind(&input.entidad)
    .bind(input.warrant_type_id)
    .bind(&input.process_type)
    .bind(input.fecha_vencimiento)
    .bind(input.fecha_renovacion)
    .execute(&mut *tx)
    .await?;

    let id = result.last_insert_id() as i64;

    if let (Some(warrant_type_id), Some(fecha_vencimiento)) =
        (input.warrant_type_id, input.fecha_vencimiento)
    {
        sqlx::query(
            r#"
            UPDATE warrant
            SET renovated = 1
            WHERE expediente = ?
              AND warrant_type_id = ?
              AND fecha_vencimiento < ?
              AND DATEDIFF(fecha_vencimiento, CURDATE()) > 0
            "#,
        )
        .bind(input.expediente)
        .bind(warrant_type_id)
        .bind(fecha_vencimiento)
        .execute(&mut *tx)
        .await?;
    }

    propagate_process_type(&mut tx, &input).await?;

    tx.commit().await?;

    find_by_id(pool, id).await
}

pub async fn update(pool: &MySqlPool, id: i64, input: SaveWarrant) -> Result<Warrant, ApiError> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query(
        r#"
        UPDATE warrant
        SET expediente = ?,
            nro_carta = ?,
            obra = ?,
            proveedor = ?,
            entidad = ?,
            warrant_type_id = ?,
            process_type = ?,
            fecha_vencimiento = ?,
            fecha_renovacion = ?
        WHERE id = ?
        "#,
    )
    .bind(input.expediente)
    .bind(&input.nro_carta)
    .bind(&input.obra)
    .bind(&input.proveedor)
    .bind(&input.entidad)
    .bind(input.warrant_type_id)
    .bind(&input.process_type)
    .bind(input.fecha_vencimiento)
    .bind(input.fecha_renovacion)
    .bind(id)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    propagate_process_type(&mut tx, &input).await?;

    tx.commit().await?;

    find_by_id(pool, id).await
}

async fn propagate_process_type(
    tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
    input: &SaveWarrant,
) -> Result<(), ApiError> {
    if let Some(process_type) = input.process_type.as_deref() {
        if !process_type.trim().is_empty() {
            sqlx::query(
                r#"
                UPDATE warrant
                SET process_type = ?
                WHERE expediente = ?
                  AND obra <=> ?
                "#,
            )
            .bind(process_type)
            .bind(input.expediente)
            .bind(&input.obra)
            .execute(&mut **tx)
            .await?;
        }
    }

    Ok(())
}

pub async fn delete(pool: &MySqlPool, id: i64) -> Result<(), ApiError> {
    let result = sqlx::query("UPDATE warrant SET canceled = 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(())
}

pub async fn max_expediente(pool: &MySqlPool) -> Result<i32, ApiError> {
    let value: Option<i32> = sqlx::query_scalar("SELECT MAX(expediente) FROM warrant")
        .fetch_one(pool)
        .await?;
    Ok(value.unwrap_or(0))
}

pub async fn notification_count(pool: &MySqlPool) -> Result<i64, ApiError> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM warrant WHERE canceled = 0 AND DATEDIFF(fecha_vencimiento, CURDATE()) BETWEEN 1 AND 5"
    )
    .fetch_one(pool)
    .await?;
    Ok(count)
}

#[derive(Debug, FromRow)]
struct WarrantRow {
    id: i64,
    provider_id: Option<i32>,
    process_type: Option<String>,
    extension: Option<String>,
    proveedor: Option<String>,
    expediente: Option<i32>,
    entidad: Option<String>,
    numero: Option<i32>,
    obra: Option<String>,
    warrant_type_id: Option<i32>,
    total: Option<f64>,
    fecha_renovacion: Option<NaiveDateTime>,
    nro_carta: Option<String>,
    fecha_emision: Option<NaiveDateTime>,
    fecha_vencimiento: Option<NaiveDateTime>,
    fecha_registro: Option<NaiveDateTime>,
    observacion: Option<String>,
    confirmacion_banco: Option<String>,
    renovated: Option<bool>,
    status: Option<bool>,
    canceled: bool,
    upload: Option<bool>,
    diff: Option<i64>,

    wt_id: Option<i32>,
    wt_name: Option<String>,
}

impl From<WarrantRow> for Warrant {
    fn from(row: WarrantRow) -> Self {
        Self {
            id: row.id,
            provider_id: row.provider_id,
            process_type: row.process_type,
            extension: row.extension,
            proveedor: row.proveedor,
            expediente: row.expediente,
            entidad: row.entidad,
            numero: row.numero,
            obra: row.obra,
            warrant_type_id: row.warrant_type_id,
            total: row.total,
            fecha_renovacion: row.fecha_renovacion,
            nro_carta: row.nro_carta,
            fecha_emision: row.fecha_emision,
            fecha_vencimiento: row.fecha_vencimiento,
            fecha_registro: row.fecha_registro,
            observacion: row.observacion,
            confirmacion_banco: row.confirmacion_banco,
            renovated: row.renovated,
            status: row.status,
            canceled: row.canceled,
            upload: row.upload,
            diff: row.diff,

            warrant_type: row.wt_id.map(|id| WarrantType {
                id,
                name: row.wt_name.unwrap_or_default(),
            }),

            ext: None,
        }
    }
}
