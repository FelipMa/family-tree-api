use crate::database::{
    get_connection_pool,
    models::{Relationship, RelationshipType},
};
use sqlx::{types::Uuid, Row};

pub async fn update_relationship(
    from_id: String,
    to_id: String,
    relationship_type: String,
) -> Result<Relationship, sqlx::Error> {
    let query =
        "UPDATE \"relationships\" SET type = $1 WHERE from_id = $2 AND to_id = $3 RETURNING *";

    let pool = get_connection_pool().await?;

    let sqlx_from_id = match Uuid::parse_str(&from_id) {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::Protocol("Invalid UUID".to_string())),
    };

    let sqlx_to_id = match Uuid::parse_str(&to_id) {
        Ok(id) => id,
        Err(_) => return Err(sqlx::Error::Protocol("Invalid UUID".to_string())),
    };

    let sqlx_relationship_type = match RelationshipType::from_string(&relationship_type) {
        Ok(t) => t,
        Err(e) => return Err(sqlx::Error::Protocol(e)),
    };

    let row = sqlx::query(&query)
        .bind(sqlx_relationship_type)
        .bind(sqlx_from_id)
        .bind(sqlx_to_id)
        .fetch_one(&pool)
        .await?;

    let relationship = Relationship {
        from_individual_id: row.get::<Uuid, _>("from_id").to_string(),
        to_individual_id: row.get::<Uuid, _>("to_id").to_string(),
        relationship_type: row.get::<RelationshipType, _>("type").to_string(),
    };

    Ok(relationship)
}
