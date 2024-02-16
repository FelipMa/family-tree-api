use crate::queries;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

pub async fn get_one_individual(Path(user_id): Path<String>) -> impl IntoResponse {
    match queries::individuals::read_one_individual::read_one_individual(user_id).await {
        Ok(individual) => {
            return (StatusCode::OK, Json(json!(individual)));
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
