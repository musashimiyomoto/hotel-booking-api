use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::models;

pub fn extract_user_from_token(token: &str, secret: &str) -> Result<models::users::Claims, String> {
    decode::<models::users::Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("Invalid token: {}", e))
}
