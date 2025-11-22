use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};

use crate::{models, utils};

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = models::users::RegisterRequest,
    responses(
        (status = StatusCode::CREATED, description = "User registered successfully", body = models::users::AuthResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input or email already exists"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn register(
    State(state): State<models::AppState>,
    Json(payload): Json<models::users::RegisterRequest>,
) -> Result<(StatusCode, Json<models::users::AuthResponse>), (StatusCode, String)> {
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

    let user: models::users::User = state
        .services
        .user_service
        .create(
            payload.email.clone(),
            password_hash,
            payload.first_name,
            payload.last_name,
        )
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

    let token: String = utils::create_jwt_token(
        user.id,
        user.email.clone(),
        &state.jwt_secret,
        state.jwt_expire_hours,
    )?;

    Ok((
        StatusCode::CREATED,
        Json(models::users::AuthResponse {
            user: models::users::UserResponse::from(user),
            token,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = models::users::LoginRequest,
    responses(
        (status = StatusCode::OK, description = "Login successful", body = models::users::AuthResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid credentials"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn login(
    State(state): State<models::AppState>,
    Json(payload): Json<models::users::LoginRequest>,
) -> Result<Json<models::users::AuthResponse>, (StatusCode, String)> {
    let user: models::users::User = state
        .services
        .user_service
        .get_by_email(&payload.email)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch user".to_string(),
            )
        })?
        .ok_or((
            StatusCode::BAD_REQUEST,
            "Invalid email or password".to_string(),
        ))?;

    let password_valid: bool =
        bcrypt::verify(&payload.password, &user.password_hash).map_err(|e| {
            tracing::error!("Failed to verify password: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to verify password".to_string(),
            )
        })?;

    if !password_valid {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid email or password".to_string(),
        ));
    }

    let token: String = utils::create_jwt_token(
        user.id,
        user.email.clone(),
        &state.jwt_secret,
        state.jwt_expire_hours,
    )?;

    Ok(Json(models::users::AuthResponse {
        user: models::users::UserResponse::from(user),
        token,
    }))
}

#[utoipa::path(
    get,
    path = "/auth/profile",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = StatusCode::OK, description = "User profile", body = models::users::UserResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn profile(
    State(state): State<models::AppState>,
    headers: HeaderMap,
) -> Result<Json<models::users::UserResponse>, (StatusCode, String)> {
    let token: &str = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let claims: models::users::Claims = utils::extract_user_from_token(token, &state.jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    let user: models::users::User = state
        .services
        .user_service
        .get_by_id(claims.sub)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch user".to_string(),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(models::users::UserResponse::from(user)))
}

#[utoipa::path(
    put,
    path = "/auth/profile",
    tag = "auth",
    request_body = models::users::UpdateUserRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = StatusCode::OK, description = "Profile updated", body = models::users::UserResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn update_profile(
    State(state): State<models::AppState>,
    headers: HeaderMap,
    Json(payload): Json<models::users::UpdateUserRequest>,
) -> Result<Json<models::users::UserResponse>, (StatusCode, String)> {
    let token: &str = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

    let claims: models::users::Claims = utils::extract_user_from_token(token, &state.jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    let user: models::users::User = state
        .services
        .user_service
        .update(claims.sub, payload.first_name, payload.last_name)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update user".to_string(),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(models::users::UserResponse::from(user)))
}
