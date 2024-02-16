use crate::queries;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

pub async fn delete_individual(Path(user_id): Path<String>) -> impl IntoResponse {
    match queries::individuals::delete_individual::delete_individual(user_id).await {
        Ok(value) => {
            if value {
                return (StatusCode::NO_CONTENT, Json(json!({})));
            } else {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "error": "Individual not found"
                    })),
                );
            }
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
