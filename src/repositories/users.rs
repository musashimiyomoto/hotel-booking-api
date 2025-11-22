use crate::models::users::User;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        email: String,
        password_hash: String,
        first_name: String,
        last_name: String,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash, first_name, last_name) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id, email, password_hash, first_name, last_name, created_at, updated_at",
        )
        .bind(email)
        .bind(password_hash)
        .bind(first_name)
        .bind(last_name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, first_name, last_name, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, first_name, last_name, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update(
        &self,
        id: i32,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE users SET first_name = $1, last_name = $2, updated_at = CURRENT_TIMESTAMP 
             WHERE id = $3 
             RETURNING id, email, password_hash, first_name, last_name, created_at, updated_at",
        )
        .bind(first_name)
        .bind(last_name)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }
}
