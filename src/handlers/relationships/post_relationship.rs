use crate::queries;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(serde::Deserialize)]
pub struct CreateRelationshipPayload {
    from_individual_id: String,
    to_individual_id: String,
    relationship_type: String,
}

pub async fn post_relationship(payload: Json<CreateRelationshipPayload>) -> impl IntoResponse {
    match payload.relationship_type.as_str() {
        "PARENT_SON" => {}
        "MARRIAGE" => {}
        "DIVORCE" => {}
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid relationship type"
                })),
            );
        }
    }

    match queries::relationships::create_relationship::create_relationship(
        payload.from_individual_id.clone(),
        payload.to_individual_id.clone(),
        payload.relationship_type.clone(),
    )
    .await
    {
        Ok(relationship) => {
            return (StatusCode::CREATED, Json(json!(relationship)));
        }
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": error.to_string()
                })),
            );
        }
    }
}
