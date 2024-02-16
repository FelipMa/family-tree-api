use crate::queries;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

pub async fn login(Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    let user = match queries::users::read_user_by_email::read_user_by_email(payload.email).await {
        Ok(user) => user,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": err.to_string()
                })),
            );
        }
    };

    let argon2 = Argon2::default();

    let password_hash = match PasswordHash::new(&user.password) {
        Ok(password_hash) => password_hash,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": err.to_string()
                })),
            );
        }
    };

    let password = payload.password.as_bytes();

    if argon2.verify_password(password, &password_hash).is_ok() {
        return (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "logged in",
            })),
        );
    } else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Invalid credentials"
            })),
        );
    }
}
