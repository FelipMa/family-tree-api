use crate::database::get_connection_pool;
use sqlx::types::Uuid;

pub async fn delete_individual(id: String) -> Result<bool, sqlx::Error> {
    let query = "DELETE FROM \"individuals\" WHERE id = $1";

    let pool = get_connection_pool().await?;

    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::Protocol("Invalid ID".to_string())),
    };

    let n = sqlx::query(&query).bind(id).execute(&pool).await?;

    if n.rows_affected() == 0 {
        return Ok(false);
    } else {
        return Ok(true);
    }
}
