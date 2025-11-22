use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::models;

#[utoipa::path(
    get,
    path = "/hotels",
    tag = "hotels",
    responses(
        (status = http::StatusCode::OK, description = "List of hotels", body = Vec<models::hotels::HotelResponse>),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn list_hotels(
    State(state): State<models::AppState>,
) -> Result<Json<Vec<models::hotels::HotelResponse>>, (StatusCode, String)> {
    let hotels: Vec<models::hotels::Hotel> =
        state.services.hotel_service.list_all().await.map_err(|e| {
            tracing::error!("Failed to fetch hotels: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch hotels".to_string(),
            )
        })?;

    Ok(Json(
        hotels
            .into_iter()
            .map(models::hotels::HotelResponse::from)
            .collect(),
    ))
}

#[utoipa::path(
    get,
    path = "/hotels/{id}",
    tag = "hotels",
    params(
        ("id" = i32, Path, description = "Hotel ID")
    ),
    responses(
        (status = http::StatusCode::OK, description = "Hotel details", body = models::hotels::HotelResponse),
        (status = http::StatusCode::NOT_FOUND, description = "Hotel not found"),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn get_hotel(
    State(state): State<models::AppState>,
    Path(id): Path<i32>,
) -> Result<Json<models::hotels::HotelResponse>, (StatusCode, String)> {
    let hotel: models::hotels::Hotel = state
        .services
        .hotel_service
        .get_by_id(id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch hotel: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch hotel".to_string(),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "Hotel not found".to_string()))?;

    Ok(Json(models::hotels::HotelResponse::from(hotel)))
}

#[utoipa::path(
    post,
    path = "/hotels",
    tag = "hotels",
    request_body = models::hotels::CreateHotelRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = http::StatusCode::CREATED, description = "Hotel created", body = models::hotels::HotelResponse),
        (status = http::StatusCode::UNPROCESSABLE_ENTITY, description = "Invalid input"),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn create_hotel(
    State(state): State<models::AppState>,
    Json(payload): Json<models::hotels::CreateHotelRequest>,
) -> Result<(StatusCode, Json<models::hotels::HotelResponse>), (StatusCode, String)> {
    let hotel: models::hotels::Hotel = state
        .services
        .hotel_service
        .create(
            payload.name,
            payload.description,
            payload.address,
            payload.city,
            payload.country,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to create hotel: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create hotel".to_string(),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(models::hotels::HotelResponse::from(hotel)),
    ))
}

#[utoipa::path(
    put,
    path = "/hotels/{id}",
    tag = "hotels",
    params(
        ("id" = i32, Path, description = "Hotel ID")
    ),
    request_body = models::hotels::UpdateHotelRequest,
    security(("bearer_auth" = [])),
    responses(
        (status = http::StatusCode::OK, description = "Hotel updated", body = models::hotels::HotelResponse),
        (status = http::StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = http::StatusCode::NOT_FOUND, description = "Hotel not found"),
        (status = http::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    )
)]
pub async fn update_hotel(
    State(state): State<models::AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<models::hotels::UpdateHotelRequest>,
) -> Result<Json<models::hotels::HotelResponse>, (StatusCode, String)> {
    let hotel: models::hotels::Hotel = state
        .services
        .hotel_service
        .update(
            id,
            payload.name,
            payload.description,
            payload.address,
            payload.city,
            payload.country,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to update hotel: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update hotel".to_string(),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "Hotel not found".to_string()))?;

    Ok(Json(models::hotels::HotelResponse::from(hotel)))
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
    State(state): State<models::AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let rows_affected: u64 = state.services.hotel_service.delete(id).await.map_err(|e| {
        tracing::error!("Failed to delete hotel: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to delete hotel".to_string(),
        )
    })?;

    if rows_affected == 0 {
        return Err((StatusCode::NOT_FOUND, "Hotel not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
