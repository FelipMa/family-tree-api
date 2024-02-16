use crate::queries;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn get_all_individuals() -> impl IntoResponse {
    match queries::individuals::read_all_individuals::read_all_individuals().await {
        Ok(individuals) => {
            return (StatusCode::OK, Json(json!(individuals)));
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": err.to_string()
                })),
            );
        }
    };
}
