use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthLiveResponse {
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthServiceResponse {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthReadyResponse {
    pub status: String,
    pub services: Vec<HealthServiceResponse>,
}
