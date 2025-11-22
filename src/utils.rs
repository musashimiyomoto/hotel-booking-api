use crate::models;
use axum::http::StatusCode;
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode, encode};

pub fn create_jwt_token(
    user_id: i32,
    email: String,
    jwt_secret: &str,
    jwt_expire_hours: i64,
) -> Result<String, (StatusCode, String)> {
    let now: i64 = Utc::now().timestamp();
    let claims: models::users::Claims = models::users::Claims {
        sub: user_id,
        email,
        iat: now,
        exp: now + (jwt_expire_hours * 3600),
    };

    encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| {
        tracing::error!("Failed to generate token: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate token".to_string(),
        )
    })
}

pub fn extract_user_from_token(token: &str, secret: &str) -> Result<models::users::Claims, String> {
    decode::<models::users::Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Invalid token: {}", e))
}
