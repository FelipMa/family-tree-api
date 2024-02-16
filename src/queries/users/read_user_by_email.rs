use crate::database::get_connection_pool;
use crate::database::models::UserWithPassword;
use sqlx::{types::time::OffsetDateTime, types::Uuid, Row};

pub async fn read_user_by_email(email: String) -> Result<UserWithPassword, sqlx::Error> {
    let query = "SELECT * FROM \"users\" WHERE email = $1 LIMIT 1";

    let pool = get_connection_pool().await?;

    let row = sqlx::query(&query).bind(email).fetch_one(&pool).await?;

    let user = UserWithPassword {
        id: row.get::<Uuid, _>("id").to_string(),
        email: row.get("email"),
        password: row.get("password"),
        is_admin: row.get("is_admin"),
        created_at: row.get::<OffsetDateTime, _>("created_at").to_string(),
        updated_at: row.get::<OffsetDateTime, _>("updated_at").to_string(),
    };

    return Ok(user);
}
