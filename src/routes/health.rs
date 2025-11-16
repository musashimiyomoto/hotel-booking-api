use crate::{enums, models};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

#[utoipa::path(
    get,
    path = "/health/live",
    responses(
        (status = StatusCode::OK, description = "API is running", body = models::health::HealthLiveResponse)
    ),
    tag = "health"
)]
pub async fn live() -> Result<Json<models::health::HealthLiveResponse>, (StatusCode, String)> {
    tracing::info!("GET /health/live endpoint called");

    Ok(Json(models::health::HealthLiveResponse {
        status: enums::HealthStatus::Ok.to_string(),
    }))
}

#[utoipa::path(
    get,
    path = "/health/ready",
    responses(
        (status = StatusCode::OK, description = "All services are ready", body = models::health::HealthReadyResponse),
        (status = StatusCode::SERVICE_UNAVAILABLE, description = "One or more services are not ready", body = models::health::HealthReadyResponse)
    ),
    tag = "health"
)]
pub async fn ready(
    state: State<models::AppState>,
) -> Result<Json<models::health::HealthReadyResponse>, (StatusCode, String)> {
    tracing::info!("GET /health/ready endpoint called");

    let postgres_status = match sqlx::query("SELECT 1").fetch_one(&state.pool).await {
        Ok(_) => enums::HealthStatus::Ok,
        Err(_) => enums::HealthStatus::Unavailable,
    };

    let redis_status = match redis::cmd("PING")
        .exec_async(&mut state.redis_conn.clone())
        .await
    {
        Ok(_) => enums::HealthStatus::Ok,
        Err(_) => enums::HealthStatus::Unavailable,
    };

    let services = vec![
        models::health::HealthServiceResponse {
            name: enums::ServiceName::Postgres.to_string(),
            status: postgres_status.to_string(),
        },
        models::health::HealthServiceResponse {
            name: enums::ServiceName::Redis.to_string(),
            status: redis_status.to_string(),
        },
    ];

    if !services
        .iter()
        .all(|s| s.status == enums::HealthStatus::Ok.to_string())
    {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "One or more services are not ready".to_string(),
        ));
    }

    Ok(Json(models::health::HealthReadyResponse {
        status: enums::HealthStatus::Ok.to_string(),
        services,
    }))
}
