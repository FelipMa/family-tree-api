use crate::queries;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(serde::Deserialize)]
pub struct RegisterPayload {
    email: String,
    password: String,
}

pub async fn register(Json(payload): Json<RegisterPayload>) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    match queries::users::create_user::create_user(payload.email, password_hash.clone()).await {
        Ok(user) => {
            return (StatusCode::CREATED, Json(json!(user)));
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
