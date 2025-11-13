pub mod health;
pub mod hotels;
pub mod users;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::Pool<sqlx::Postgres>,
    pub redis_conn: redis::aio::MultiplexedConnection,
}
