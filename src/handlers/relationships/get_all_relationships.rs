use crate::queries;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn get_all_relationships() -> impl IntoResponse {
    match queries::relationships::read_all_relationships::read_all_relationships().await {
        Ok(relationships) => {
            return (StatusCode::OK, Json(json!(relationships)));
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
