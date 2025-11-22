.PHONY: help up down logs test test-health test-users test-hotels test-all clean restart build fmt fmt-check lint check

help:
	@echo "Available commands:"
	@echo ""
	@echo "Docker:"
	@echo "  make up              - Start Docker Compose services"
	@echo "  make down            - Stop Docker Compose services"
	@echo "  make logs            - Show Docker Compose logs"
	@echo "  make restart         - Restart Docker Compose services"
	@echo ""
	@echo "Testing:"
	@echo "  make test            - Start services, run all tests, stop services"
	@echo "  make test-all        - Run all tests"
	@echo "  make test-health     - Run health endpoint tests"
	@echo "  make test-users      - Run users endpoint tests"
	@echo "  make test-hotels     - Run hotels endpoint tests"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt             - Format code with rustfmt"
	@echo "  make fmt-check       - Check code formatting without changes"
	@echo "  make lint            - Run clippy linter"
	@echo "  make check           - Run fmt-check and lint"
	@echo ""
	@echo "Build:"
	@echo "  make build           - Build Rust project"
	@echo "  make clean           - Remove Docker containers and volumes"

up:
	@if [ ! -f .env ]; then \
		echo "$(BLUE)Copying .env.example to .env...$(NC)"; \
		cp .env.example .env; \
		echo "$(GREEN).env.example to .env copied!$(NC)"; \
	fi

	@echo "🚀 Starting Docker Compose..."
	docker compose up -d
	@echo "⏳ Waiting for API to be ready..."

	@for i in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15; do \
		if curl -s http://localhost:8000/health/live > /dev/null 2>&1; then \
			echo "✅ API is ready!"; \
			exit 0; \
		fi; \
		echo "Waiting... (attempt $$i/15)"; \
		sleep 2; \
	done; \
	echo "❌ API failed to start"; \
	exit 1

down:
	@echo "🛑 Stopping Docker Compose..."
	docker compose down

logs:
	docker compose logs -f

restart: down up

build:
	cargo build

test-health:
	@echo "🧪 Running health tests..."
	cargo test --test tests_health -- --nocapture

test-users:
	@echo "🧪 Running users tests..."
	cargo test --test tests_users -- --nocapture

test-hotels:
	@echo "🧪 Running hotels tests..."
	cargo test --test tests_hotels -- --nocapture

test-all: test-health test-users test-hotels

test: up test-all down
	@echo "✨ All tests completed!"

fmt:
	@echo "🎨 Formatting code..."
	cargo fmt
	@echo "✅ Code formatted!"

fmt-check:
	@echo "🔍 Checking code formatting..."
	cargo fmt -- --check

lint:
	@echo "🔎 Running clippy..."
	cargo clippy -- -D warnings

check: fmt-check lint
	@echo "✅ All checks passed!"

clean:
	@echo "🧹 Cleaning up..."
	docker compose down -v
	cargo clean
	@echo "✅ Clean complete!"
