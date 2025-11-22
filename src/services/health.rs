use crate::enums::{HealthStatus, ServiceName};
use crate::models::health::HealthServiceResponse;
use crate::repositories::health::HealthRepository;

#[derive(Clone)]
pub struct HealthService {
    repo: HealthRepository,
}

impl HealthService {
    pub fn new(repo: HealthRepository) -> Self {
        Self { repo }
    }

    pub async fn check_services(&self) -> Vec<HealthServiceResponse> {
        let postgres_status = self.repo.check_postgres().await;
        let redis_status = self.repo.check_redis().await;

        vec![
            HealthServiceResponse {
                name: ServiceName::Postgres.to_string(),
                status: postgres_status.to_string(),
            },
            HealthServiceResponse {
                name: ServiceName::Redis.to_string(),
                status: redis_status.to_string(),
            },
        ]
    }

    pub async fn is_ready(&self) -> bool {
        let services = self.check_services().await;
        services
            .iter()
            .all(|s| s.status == HealthStatus::Ok.to_string())
    }
}
