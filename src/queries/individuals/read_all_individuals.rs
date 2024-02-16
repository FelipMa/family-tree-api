use crate::database::get_connection_pool;
use crate::database::models::Individual;
use crate::queries::get_possible_null_date;
use sqlx::{types::time::OffsetDateTime, types::Uuid, Row};

pub async fn read_all_individuals() -> Result<Vec<Individual>, sqlx::Error> {
    let query = "SELECT * FROM \"individuals\"";

    let pool = get_connection_pool().await?;

    let row = sqlx::query(&query).fetch_all(&pool).await?;

    let mut users: Vec<Individual> = Vec::new();

    for row in row {
        let individual = Individual {
            id: row.get::<Uuid, _>("id").to_string(),
            name: row.get("name"),
            image: row.try_get("image").unwrap_or("null".to_string()),
            birthday: get_possible_null_date(&row, "birthday"),
            generation: row.get("generation"),
            is_alive: row.get("is_alive"),
            death: get_possible_null_date(&row, "death"),
            created_at: row.get::<OffsetDateTime, _>("created_at").to_string(),
            updated_at: row.get::<OffsetDateTime, _>("updated_at").to_string(),
        };
        users.push(individual);
    }

    return Ok(users);
}
