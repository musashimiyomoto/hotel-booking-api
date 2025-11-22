mod enums;
mod middleware;
mod models;
mod repositories;
mod resources;
mod routes;
mod services;
mod settings;
mod utils;

use tower_http::cors;

async fn init_state(settings: &settings::Settings) -> models::AppState {
    tracing::info!("Initializing state");

    let pool: sqlx::Pool<sqlx::Postgres> = match resources::init_postgres(settings).await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to initialize database: {}", e);
            std::process::exit(1);
        }
    };

    let redis_conn: redis::aio::MultiplexedConnection = match resources::init_redis(settings).await
    {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to initialize Redis: {}", e);
            std::process::exit(1);
        }
    };

    tracing::info!("Initialized state");

    models::AppState {
        jwt_secret: settings.jwt_secret.clone(),
        jwt_expire_hours: settings.jwt_expire_hours,
        services: services::Services::new(pool, redis_conn),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let settings: settings::Settings = settings::Settings::new();
    let state: models::AppState = init_state(&settings).await;
    let addr: String = format!("{}:{}", settings.app_host, settings.app_port);

    tracing::info!("Starting hotel booking API on {}/docs", addr);

    let app: axum::Router = routes::create_routers(state).layer(
        cors::CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any),
    );

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
