pub mod health;
pub mod hotels;
pub mod users;

use crate::services::Services;

#[derive(Clone)]
pub struct AppState {
    pub jwt_secret: String,
    pub jwt_expire_hours: i64,
    pub services: Services,
}
