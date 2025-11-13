use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey};

use crate::{
    middleware::extract_user_from_token,
    models::{
        users::{AuthResponse, Claims, LoginRequest, RegisterRequest, User, UserResponse},
        AppState,
    },
};

const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";
const JWT_EXPIRY_HOURS: i64 = 24;

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = StatusCode::CREATED, description = "User registered successfully", body = AuthResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input or email already exists"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, String)> {
    if !payload.email.contains('@') {
        return Err((StatusCode::BAD_REQUEST, "Invalid email format".to_string()));
    }

    if payload.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password must be at least 6 characters".to_string(),
        ));
    }

    let password_hash = bcrypt::hash(&payload.password, 12).map_err(|e| {
        tracing::error!("Failed to hash password: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to hash password".to_string(),
        )
    })?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, first_name, last_name) 
         VALUES ($1, $2, $3, $4) 
         RETURNING id, email, password_hash, first_name, last_name, created_at, updated_at"
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create user: {}", e);
        let msg = if e.to_string().contains("duplicate key") {
            "Email already exists".to_string()
        } else {
            "Failed to create user".to_string()
        };
        (StatusCode::BAD_REQUEST, msg)
    })?;

    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        iat: now,
        exp: now + (JWT_EXPIRY_HOURS * 3600),
    };

    let token = encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|e| {
        tracing::error!("Failed to generate token: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate token".to_string(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            user: UserResponse::from(user),
            token,
        }),
    ))
}


#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = StatusCode::OK, description = "Login successful", body = AuthResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid credentials"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, first_name, last_name, created_at, updated_at FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch user: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch user".to_string(),
        )
    })?
    .ok_or((StatusCode::BAD_REQUEST, "Invalid email or password".to_string()))?;

    let password_valid = bcrypt::verify(&payload.password, &user.password_hash).map_err(|e| {
        tracing::error!("Failed to verify password: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to verify password".to_string(),
        )
    })?;

    if !password_valid {
        return Err((StatusCode::BAD_REQUEST, "Invalid email or password".to_string()));
    }

    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        iat: now,
        exp: now + (JWT_EXPIRY_HOURS * 3600),
    };

    let token = encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|e| {
        tracing::error!("Failed to generate token: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate token".to_string(),
        )
    })?;

    Ok(Json(AuthResponse {
        user: UserResponse::from(user),
        token,
    }))
}

#[utoipa::path(
    get,
    path = "/auth/profile",
    tag = "auth",
    responses(
        (status = StatusCode::OK, description = "User profile", body = UserResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn profile(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let claims = extract_user_from_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, first_name, last_name, created_at, updated_at FROM users WHERE id = $1"
    )
    .bind(claims.sub)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch user: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch user".to_string(),
        )
    })?
    .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(UserResponse::from(user)))
}

#[utoipa::path(
    put,
    path = "/auth/profile",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = StatusCode::OK, description = "Profile updated", body = UserResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn update_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let claims = extract_user_from_token(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET first_name = $1, last_name = $2, updated_at = CURRENT_TIMESTAMP 
         WHERE id = $3 
         RETURNING id, email, password_hash, first_name, last_name, created_at, updated_at"
    )
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(claims.sub)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update user: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update user".to_string(),
        )
    })?
    .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(UserResponse::from(user)))
}
