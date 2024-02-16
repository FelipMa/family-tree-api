use crate::database::get_connection_pool;
use crate::database::models::Relationship;
use crate::database::models::RelationshipType;
use sqlx::{types::Uuid, Row};

pub async fn create_relationship(
    from_individual_id: String,
    to_individual_id: String,
    relationship_type: String,
) -> Result<Relationship, sqlx::Error> {
    let query =
        "INSERT INTO \"relationships\" (from_id, to_id, type) VALUES ($1, $2, $3) RETURNING *";

    let pool = get_connection_pool().await?;

    let sqlx_from_id = match Uuid::parse_str(&from_individual_id) {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::Protocol("Invalid UUID".to_string())),
    };

    let sqlx_to_id = match Uuid::parse_str(&to_individual_id) {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::Protocol("Invalid UUID".to_string())),
    };

    let sqlx_relationship_type = match RelationshipType::from_string(&relationship_type) {
        Ok(t) => t,
        Err(e) => return Err(sqlx::Error::Protocol(e)),
    };

    let row = sqlx::query(&query)
        .bind(sqlx_from_id)
        .bind(sqlx_to_id)
        .bind(sqlx_relationship_type)
        .fetch_one(&pool)
        .await?;

    let relationship = Relationship {
        from_individual_id: row.get::<Uuid, _>("from_id").to_string(),
        to_individual_id: row.get::<Uuid, _>("to_id").to_string(),
        relationship_type: row.get::<RelationshipType, _>("type").to_string(),
    };

    return Ok(relationship);
}
