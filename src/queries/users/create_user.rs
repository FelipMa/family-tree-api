use crate::database::get_connection_pool;
use crate::database::models::UserWithoutPassword;
use sqlx::{types::time::OffsetDateTime, types::Uuid, Row};

pub async fn create_user(
    email: String,
    password_hash: String,
) -> Result<UserWithoutPassword, sqlx::Error> {
    let query = "INSERT INTO \"users\" (email, password) VALUES ($1, $2) RETURNING *";

    let pool = get_connection_pool().await?;

    let row = sqlx::query(&query)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&pool)
        .await?;

    let user = UserWithoutPassword {
        id: row.get::<Uuid, _>("id").to_string(),
        email: row.get("email"),
        is_admin: row.get("is_admin"),
        created_at: row.get::<OffsetDateTime, _>("created_at").to_string(),
        updated_at: row.get::<OffsetDateTime, _>("updated_at").to_string(),
    };

    return Ok(user);
}
