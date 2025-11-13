use std::env;

pub struct Settings {
    pub app_host: String,
    pub app_port: String,

    pub redis_host: String,
    pub redis_port: String,
    pub redis_db: String,

    pub postgres_user: String,
    pub postgres_password: String,
    pub postgres_host: String,
    pub postgres_port: String,
    pub postgres_db: String,
    pub postgres_max_pool: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            app_host: env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            app_port: env::var("APP_PORT").unwrap_or_else(|_| "8000".to_string()),

            redis_host: env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string()),
            redis_port: env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string()),
            redis_db: env::var("REDIS_DB").unwrap_or_else(|_| "0".to_string()),

            postgres_user: env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
            postgres_password: env::var("POSTGRES_PASSWORD")
                .unwrap_or_else(|_| "postgres".to_string()),
            postgres_host: env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
            postgres_port: env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string()),
            postgres_db: env::var("POSTGRES_DB").unwrap_or_else(|_| "postgres".to_string()),
            postgres_max_pool: env::var("POSTGRES_MAX_POOL").unwrap_or_else(|_| "5".to_string()),
        }
    }

    pub fn get_redis_url(&self) -> String {
        format!(
            "redis://{}:{}/{}",
            self.redis_host, self.redis_port, self.redis_db
        )
    }

    pub fn get_postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db,
        )
    }
}
