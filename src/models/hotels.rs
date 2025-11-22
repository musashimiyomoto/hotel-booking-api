use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Hotel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub city: String,
    pub country: String,
    pub rating: Option<f64>,
    pub total_reviews: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateHotelRequest {
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateHotelRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HotelResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub address: String,
    pub city: String,
    pub country: String,
    pub rating: Option<f64>,
    pub total_reviews: Option<i32>,
}

impl From<Hotel> for HotelResponse {
    fn from(hotel: Hotel) -> Self {
        Self {
            id: hotel.id,
            name: hotel.name,
            description: hotel.description,
            address: hotel.address,
            city: hotel.city,
            country: hotel.country,
            rating: hotel.rating,
            total_reviews: hotel.total_reviews,
        }
    }
}
