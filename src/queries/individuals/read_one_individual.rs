use crate::database::get_connection_pool;
use crate::database::models::Individual;
use crate::queries::get_possible_null_date;
use sqlx::{types::time::OffsetDateTime, types::Uuid, Row};

pub async fn read_one_individual(id: String) -> Result<Vec<Individual>, sqlx::Error> {
    let query = "SELECT * FROM \"individuals\" WHERE id = $1";

    let pool = get_connection_pool().await?;

    let id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::Protocol("Invalid ID".to_string())),
    };

    let row = sqlx::query(&query).bind(&id).fetch_optional(&pool).await?;

    let mut individuals: Vec<Individual> = Vec::new();

    if let Some(row) = row {
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
        individuals.push(individual);
    }

    return Ok(individuals);
}
