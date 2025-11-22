use crate::enums::HealthStatus;
use redis::aio::MultiplexedConnection;
use sqlx::Pool;

#[derive(Clone)]
pub struct HealthRepository {
    pool: Pool<sqlx::Postgres>,
    redis_conn: MultiplexedConnection,
}

impl HealthRepository {
    pub fn new(pool: Pool<sqlx::Postgres>, redis_conn: MultiplexedConnection) -> Self {
        Self { pool, redis_conn }
    }

    pub async fn check_postgres(&self) -> HealthStatus {
        match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(_) => HealthStatus::Ok,
            Err(_) => HealthStatus::Unavailable,
        }
    }

    pub async fn check_redis(&self) -> HealthStatus {
        match redis::cmd("PING")
            .exec_async(&mut self.redis_conn.clone())
            .await
        {
            Ok(_) => HealthStatus::Ok,
            Err(_) => HealthStatus::Unavailable,
        }
    }
}
