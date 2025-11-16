use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
}

#[derive(Debug)]
pub enum HealthStatus {
    Ok,
    Unavailable,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Ok => write!(f, "ok"),
            HealthStatus::Unavailable => write!(f, "unavailable"),
        }
    }
}

#[derive(Debug)]
pub enum ServiceName {
    Redis,
    Postgres,
}

impl fmt::Display for ServiceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceName::Redis => write!(f, "redis"),
            ServiceName::Postgres => write!(f, "postgres"),
        }
    }
}
