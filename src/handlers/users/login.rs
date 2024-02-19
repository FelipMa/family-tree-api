use crate::database::models::UserWithoutPassword;
use crate::middlewares::auth::Claims;
use crate::queries;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::OffsetDateTime;

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
        let user_without_password = UserWithoutPassword {
            id: user.id,
            email: user.email,
            is_admin: user.is_admin,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };

        let claims = Claims {
            exp: (OffsetDateTime::now_utc() + time::Duration::days(1)).unix_timestamp() as usize,
            user: user_without_password.clone(),
        };

        dotenv().ok(); // load .env file

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(token) => token,
            Err(err) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": err.to_string()
                    })),
                );
            }
        };

        return (
            StatusCode::OK,
            Json(json!({
                "token": token,
                "user": user_without_password
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
