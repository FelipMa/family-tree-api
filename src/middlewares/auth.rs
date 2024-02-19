use crate::database::models::UserWithoutPassword;
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use dotenv::dotenv;
use jsonwebtoken::{decode, errors::Error, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub user: UserWithoutPassword,
}

fn authenticate_token(token: String) -> Result<Claims, Error> {
    dotenv().ok(); // load .env file

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token_data) => token_data,
        Err(err) => {
            return Err(err);
        }
    };

    return Ok(token_data.claims);
}

pub async fn auth_middleware_user_admin(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let header_token = match headers.get("Authorization") {
        Some(header_token) => header_token,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!(
                    {
                        "error": "No token provided"
                    }
                )),
            ));
        }
    };

    let token = match header_token.to_str() {
        Ok(token) => token,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid Token"})),
            ));
        }
    };

    let jwt_token = token.replace("Bearer ", "");

    let claims = match authenticate_token(jwt_token) {
        Ok(claims) => claims,
        Err(err) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": err.to_string()})),
            ));
        }
    };

    if claims.user.is_admin == false {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "User is not admin"})),
        ));
    }

    let response = next.run(request).await;
    Ok(response)
}
