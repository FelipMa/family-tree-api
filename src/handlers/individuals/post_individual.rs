use crate::queries;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateIndividualPayload {
    name: String,
    image: Option<String>,
    birthday: Option<String>,
    generation: i32,
    is_alive: bool,
    death: Option<String>,
}

pub async fn post_individual(payload: Json<CreateIndividualPayload>) -> impl IntoResponse {
    match queries::individuals::create_individual::create_individual(
        payload.name.clone(),
        payload.image.clone(),
        payload.birthday.clone(),
        payload.generation.clone(),
        payload.is_alive.clone(),
        payload.death.clone(),
    )
    .await
    {
        Ok(individual) => {
            return (StatusCode::CREATED, Json(json!(individual)));
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
