DOCKER_SERVICES ?= all
DATABASE_URL = postgres://mi-dev:mi-dev@localhost:5432/mapper-influence-dev 

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
	sqlx migrate run --database-url ${DATABASE_URL} --source ./mi-db/migrations
	DATABASE_URL=${DATABASE_URL} cargo test --all-features

update-db-schema: docker-compose-up
	sqlx migrate run --database-url ${DATABASE_URL} --source ./mi-db/migrations	
	cd mi-db && cargo sqlx prepare --database-url ${DATABASE_URL}
