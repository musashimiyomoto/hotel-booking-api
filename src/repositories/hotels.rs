use crate::models::hotels::Hotel;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct HotelRepository {
    pool: Pool<Postgres>,
}

impl HotelRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn list_all(&self) -> Result<Vec<Hotel>, sqlx::Error> {
        sqlx::query_as::<_, Hotel>("SELECT * FROM hotels ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Hotel>, sqlx::Error> {
        sqlx::query_as::<_, Hotel>("SELECT * FROM hotels WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create(
        &self,
        name: String,
        description: Option<String>,
        address: String,
        city: String,
        country: String,
    ) -> Result<Hotel, sqlx::Error> {
        sqlx::query_as::<_, Hotel>(
            "INSERT INTO hotels (name, description, address, city, country) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, name, description, address, city, country, rating, total_reviews, created_at, updated_at"
        )
        .bind(name)
        .bind(description)
        .bind(address)
        .bind(city)
        .bind(country)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(
        &self,
        id: i32,
        name: Option<String>,
        description: Option<String>,
        address: Option<String>,
        city: Option<String>,
        country: Option<String>,
    ) -> Result<Option<Hotel>, sqlx::Error> {
        sqlx::query_as::<_, Hotel>(
            "UPDATE hotels SET name = $1, description = $2, address = $3, city = $4, country = $5, updated_at = CURRENT_TIMESTAMP 
             WHERE id = $6 
             RETURNING id, name, description, address, city, country, rating, total_reviews, created_at, updated_at"
        )
        .bind(name)
        .bind(description)
        .bind(address)
        .bind(city)
        .bind(country)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM hotels WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}
