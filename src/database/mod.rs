pub mod models;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn get_connection_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().ok(); // load .env file

    let connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await?;

    return Ok(pool);
}
