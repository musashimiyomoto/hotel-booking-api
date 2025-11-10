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
