use crate::queries;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateIndividualPayload {
    name: Option<String>,
    image: Option<String>,
    birthday: Option<String>,
    generation: Option<i32>,
    is_alive: Option<bool>,
    death: Option<String>,
}

pub async fn patch_individual(
    Path(user_id): Path<String>,
    payload: Json<UpdateIndividualPayload>,
) -> impl IntoResponse {
    if payload.name.is_none()
        && payload.image.is_none()
        && payload.birthday.is_none()
        && payload.generation.is_none()
        && payload.is_alive.is_none()
        && payload.death.is_none()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "At least one field must be provided"
            })),
        );
    }

    match queries::individuals::update_individual::update_individual(
        user_id.clone(),
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
            return (StatusCode::OK, Json(json!(individual)));
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
