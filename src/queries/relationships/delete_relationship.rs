use crate::database::get_connection_pool;
use sqlx::types::Uuid;

pub async fn delete_relationship(
    from_individual_id: String,
    to_individual_id: String,
) -> Result<bool, sqlx::Error> {
    let query = "DELETE FROM \"relationships\" WHERE from_id = $1 AND to_id = $2 RETURNING *";

    let sqlx_from_id = match Uuid::parse_str(&from_individual_id) {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::Protocol("Invalid UUID".to_string())),
    };

    let sqlx_to_id = match Uuid::parse_str(&to_individual_id) {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::Protocol("Invalid UUID".to_string())),
    };

    let pool = get_connection_pool().await?;

    let n = sqlx::query(&query)
        .bind(sqlx_from_id)
        .bind(sqlx_to_id)
        .execute(&pool)
        .await?;

    if n.rows_affected() == 0 {
        return Ok(false);
    } else {
        return Ok(true);
    }
}
