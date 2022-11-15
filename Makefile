DOCKER_SERVICES ?= all

fmt:
	@echo "Formatting Rust files"
	@(rustup toolchain list | ( ! grep -q nightly && echo "Toolchain 'nightly' is not installed. Please install using 'rustup toolchain install nightly'.") ) || cargo +nightly fmt

fix: fmt
	@echo "Running cargo clippy --fix"
	cargo clippy --fix --all-features --allow-dirty --allow-staged

docker-compose-up:
	@echo "Launching ${DOCKER_SERVICES} Docker service(s)"
	COMPOSE_PROFILES=$(DOCKER_SERVICES) docker compose -f docker-compose.yml up -d --remove-orphans --wait

docker-compose-down:
	docker compose -f docker-compose.yml down --remove-orphans

test-all: docker-compose-up
	sqlx migrate run --database-url postgres://mi-dev:mi-dev@localhost:5432/mapper-influence-dev --source ./mi-db/migrations
	cargo test --all-features
