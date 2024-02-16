use crate::database::{
    get_connection_pool,
    models::{Relationship, RelationshipType},
};
use sqlx::{types::Uuid, Row};

pub async fn read_all_relationships() -> Result<Vec<Relationship>, sqlx::Error> {
    let query = "SELECT * FROM \"relationships\"";

    let pool = get_connection_pool().await?;

    let row = sqlx::query(&query).fetch_all(&pool).await?;

    let mut relationships: Vec<Relationship> = Vec::new();

    for row in row {
        let relationship = Relationship {
            from_individual_id: row.get::<Uuid, _>("from_id").to_string(),
            to_individual_id: row.get::<Uuid, _>("to_id").to_string(),
            relationship_type: row.get::<RelationshipType, _>("type").to_string(),
        };
        relationships.push(relationship);
    }

    return Ok(relationships);
}
