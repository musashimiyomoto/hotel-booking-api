# hotel-booking-api

A Rust application for managing hotel bookings using Axum, PostgreSQL, and Redis.

## Requirements

- Rust
- Cargo
- Docker
- Docker Compose

## Docker Compose Setup

### 1. Environment Configuration

```bash
cp .env.example .env
```

Edit `.env` and set variables (use `postgres` as host instead of `localhost`):
```
APP_PORT=8000
POSTGRES_HOST=postgres
POSTGRES_PORT=5432
REDIS_HOST=redis
REDIS_PORT=6379
```

### 2. Start All Services

```bash
docker-compose up --build
```

### 3. Access the API

Open your browser and navigate to `http://localhost:8000/docs` to access the API documentation.

## Running Tests

### Quick Start (Recommended)

```bash
make test
```

This will start services, run all tests, and stop containers automatically.

### Individual Test Suites

```bash
# Start services
make up

# Run specific tests
make test-health
make test-users
make test-hotels

# Or run all tests
make test-all

# Stop services
make down
```

### Useful Commands

```bash
make help      # Show all available commands
make restart   # Restart all services
make logs      # View service logs
make clean     # Remove containers and volumes
```

### Test Structure

Tests are organized by endpoint:

- **tests_health.rs** - Health check endpoints
  - GET `/health/live` (200)
  - GET `/health/ready` (200, 503)

- **tests_users.rs** - Authentication and profile endpoints
  - POST `/auth/register` (201, 400)
  - POST `/auth/login` (200, 400)
  - GET `/auth/profile` (200, 401)
  - PUT `/auth/profile` (200, 401)

- **tests_hotels.rs** - Hotel management endpoints
  - GET `/hotels` (200)
  - GET `/hotels/{id}` (200, 404)
  - POST `/hotels` (201, 400, 401)
  - PUT `/hotels/{id}` (200, 401, 404)
  - DELETE `/hotels/{id}` (204, 401, 404)

Each test validates the correct HTTP status code and response body format.
