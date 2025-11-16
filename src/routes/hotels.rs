use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::models::{AppState, hotels::*};

#[utoipa::path(
    get,
    path = "/hotels",
    tag = "hotels",
    responses(
        (status = http::StatusCode::OK, description = "List of hotels", body = Vec<HotelResponse>),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn list_hotels(
    State(state): State<AppState>,
) -> Result<Json<Vec<HotelResponse>>, (StatusCode, String)> {
    let hotels = sqlx::query_as::<_, Hotel>("SELECT * FROM hotels ORDER BY id ASC")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch hotels: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch hotels".to_string(),
            )
        })?;

    Ok(Json(hotels.into_iter().map(HotelResponse::from).collect()))
}

#[utoipa::path(
    get,
    path = "/hotels/{id}",
    tag = "hotels",
    params(
        ("id" = i32, Path, description = "Hotel ID")
    ),
    responses(
        (status = http::StatusCode::OK, description = "Hotel details", body = HotelResponse),
        (status = http::StatusCode::NOT_FOUND, description = "Hotel not found"),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn get_hotel(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<HotelResponse>, (StatusCode, String)> {
    let hotel = sqlx::query_as::<_, Hotel>("SELECT * FROM hotels WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch hotel: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch hotel".to_string(),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "Hotel not found".to_string()))?;

    Ok(Json(HotelResponse::from(hotel)))
}

#[utoipa::path(
    post,
    path = "/hotels",
    tag = "hotels",
    request_body = CreateHotelRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = http::StatusCode::CREATED, description = "Hotel created", body = HotelResponse),
        (status = http::StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn create_hotel(
    State(state): State<AppState>,
    Json(payload): Json<CreateHotelRequest>,
) -> Result<(StatusCode, Json<HotelResponse>), (StatusCode, String)> {
    let hotel = sqlx::query_as::<_, Hotel>(
        "INSERT INTO hotels (name, description, address, city, country) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING id, name, description, address, city, country, rating, total_reviews, created_at, updated_at"
    )
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.address)
    .bind(payload.city)
    .bind(payload.country)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create hotel: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create hotel".to_string(),
        )
    })?;

    Ok((StatusCode::CREATED, Json(HotelResponse::from(hotel))))
}

#[utoipa::path(
    put,
    path = "/hotels/{id}",
    tag = "hotels",
    params(
        ("id" = i32, Path, description = "Hotel ID")
    ),
    request_body = CreateHotelRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = http::StatusCode::OK, description = "Hotel updated", body = HotelResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = http::StatusCode::NOT_FOUND, description = "Hotel not found"),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn update_hotel(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateHotelRequest>,
) -> Result<Json<HotelResponse>, (StatusCode, String)> {
    let hotel = sqlx::query_as::<_, Hotel>(
        "UPDATE hotels SET name = $1, description = $2, address = $3, city = $4, country = $5, updated_at = CURRENT_TIMESTAMP 
         WHERE id = $6 
         RETURNING id, name, description, address, city, country, rating, total_reviews, created_at, updated_at"
    )
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.address)
    .bind(payload.city)
    .bind(payload.country)
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update hotel: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update hotel".to_string(),
        )
    })?
    .ok_or((StatusCode::NOT_FOUND, "Hotel not found".to_string()))?;

    Ok(Json(HotelResponse::from(hotel)))
}

#[utoipa::path(
    delete,
    path = "/hotels/{id}",
    tag = "hotels",
    params(
        ("id" = i32, Path, description = "Hotel ID")
    ),
    security(("bearer_auth" = [])),
    responses(
        (status = http::StatusCode::NO_CONTENT, description = "Hotel deleted"),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = http::StatusCode::NOT_FOUND, description = "Hotel not found"),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn delete_hotel(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM hotels WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete hotel: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete hotel".to_string(),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Hotel not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
