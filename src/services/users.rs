use crate::models::users::User;
use crate::repositories::users::UserRepository;

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self,
        email: String,
        password_hash: String,
        first_name: String,
        last_name: String,
    ) -> Result<User, sqlx::Error> {
        self.repo
            .create(email, password_hash, first_name, last_name)
            .await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        self.repo.get_by_email(email).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        self.repo.get_by_id(id).await
    }

    pub async fn update(
        &self,
        id: i32,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<Option<User>, sqlx::Error> {
        self.repo.update(id, first_name, last_name).await
    }
}
