use crate::models::hotels::Hotel;
use crate::repositories::hotels::HotelRepository;

#[derive(Clone)]
pub struct HotelService {
    repo: HotelRepository,
}

impl HotelService {
    pub fn new(repo: HotelRepository) -> Self {
        Self { repo }
    }

    pub async fn list_all(&self) -> Result<Vec<Hotel>, sqlx::Error> {
        self.repo.list_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Hotel>, sqlx::Error> {
        self.repo.find_by_id(id).await
    }

    pub async fn create(
        &self,
        name: String,
        description: Option<String>,
        address: String,
        city: String,
        country: String,
    ) -> Result<Hotel, sqlx::Error> {
        self.repo
            .create(name, description, address, city, country)
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
        self.repo
            .update(id, name, description, address, city, country)
            .await
    }

    pub async fn delete(&self, id: i32) -> Result<u64, sqlx::Error> {
        self.repo.delete(id).await
    }
}
