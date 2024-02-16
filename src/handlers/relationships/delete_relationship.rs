use crate::queries;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

pub async fn delete_relationship(
    Path((from_id, to_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match queries::relationships::delete_relationship::delete_relationship(from_id, to_id).await {
        Ok(true) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "success": true
                })),
            );
        }
        Ok(false) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": "Relationship not found"
                })),
            );
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
