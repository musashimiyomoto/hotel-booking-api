pub mod health;
pub mod hotels;
pub mod users;

use crate::{middleware, models};
use axum::{Router, routing};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Hotel Booking API", version = "0.1.0"),
    paths(
        health::live,
        health::ready,
        users::register,
        users::login,
        users::profile,
        users::update_profile,
        hotels::list_hotels,
        hotels::get_hotel,
        hotels::create_hotel,
        hotels::update_hotel,
        hotels::delete_hotel,
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "hotels", description = "Hotel management"),
    )
)]
pub struct ApiDoc;

pub fn create_routers(state: models::AppState) -> Router {
    let openapi = ApiDoc::openapi();

    let auth_routes = Router::new()
        .route(
            "/profile",
            routing::get(users::profile).put(users::update_profile),
        )
        .layer(axum::middleware::from_fn(middleware::auth_middleware));

    let protected_hotel_routes = Router::new()
        .route("/", routing::post(hotels::create_hotel))
        .route("/{id}", routing::put(hotels::update_hotel).delete(hotels::delete_hotel))
        .layer(axum::middleware::from_fn(middleware::auth_middleware));

    Router::new()
        .route("/health/live", routing::get(health::live))
        .route("/health/ready", routing::get(health::ready))
        .route("/auth/register", routing::post(users::register))
        .route("/auth/login", routing::post(users::login))
        .nest("/auth", auth_routes)
        .route("/hotels", routing::get(hotels::list_hotels))
        .route("/hotels/{id}", routing::get(hotels::get_hotel))
        .nest("/hotels", protected_hotel_routes)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
        .with_state(state)
}
