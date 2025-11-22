pub mod health;
pub mod hotels;
pub mod users;

pub use health::HealthService;
pub use hotels::HotelService;
pub use users::UserService;

use crate::repositories::{
    health::HealthRepository, hotels::HotelRepository, users::UserRepository,
};
use redis::aio::MultiplexedConnection;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct Services {
    pub health_service: HealthService,
    pub hotel_service: HotelService,
    pub user_service: UserService,
}

impl Services {
    pub fn new(pool: Pool<Postgres>, redis_conn: MultiplexedConnection) -> Self {
        Self {
            health_service: HealthService::new(HealthRepository::new(
                pool.clone(),
                redis_conn.clone(),
            )),
            hotel_service: HotelService::new(HotelRepository::new(pool.clone())),
            user_service: UserService::new(UserRepository::new(pool.clone())),
        }
    }
}
