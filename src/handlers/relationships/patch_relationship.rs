use crate::queries;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

#[derive(serde::Deserialize)]
pub struct PatchRelationshipPayload {
    relationship_type: String,
}

pub async fn patch_relationship(
    Path((from_id, to_id)): Path<(String, String)>,
    payload: Json<PatchRelationshipPayload>,
) -> impl IntoResponse {
    match queries::relationships::update_relationship::update_relationship(
        from_id,
        to_id,
        payload.relationship_type.clone(),
    )
    .await
    {
        Ok(relationship) => {
            return (StatusCode::OK, Json(json!(relationship)));
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
