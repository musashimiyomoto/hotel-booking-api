use std::{env, path};

use crate::settings;

pub async fn init_redis(
    settings: &settings::Settings,
) -> Result<redis::aio::MultiplexedConnection, redis::RedisError> {
    let client: redis::Client = redis::Client::open(settings.get_redis_url())?;

    let conn: redis::aio::MultiplexedConnection = client.get_multiplexed_async_connection().await?;

    Ok(conn)
}

pub async fn init_postgres(
    settings: &settings::Settings,
) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
        .max_connections(settings.postgres_max_pool.parse().unwrap())
        .connect(&settings.get_postgres_url())
        .await?;

    let migrator: sqlx::migrate::Migrator = sqlx::migrate::Migrator::new(path::Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/migrations"
    )))
    .await?;

    migrator.run(&pool).await?;

    Ok(pool)
}
