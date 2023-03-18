# https://cheatography.com/linux-china/cheat-sheets/justfile/
set dotenv-load

# Temporary addition to use sparse protocol
# https://blog.rust-lang.org/2023/03/09/Rust-1.68.0.html#cargos-sparse-protocol
# Remove when it becomes the default
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL := "sparse"

PG_DATABASE_URL := "postgres://mi-dev:mi-dev@localhost:5432/mapper-influence-dev"
REDIS_URL := "redis://localhost:6379"

fmt:
	@echo "Formatting Rust files"
	@(rustup toolchain list | (! grep -q nightly && echo "Toolchain 'nightly' is not installed. Please install using 'rustup toolchain install nightly'.") ) || cargo +nightly fmt

fix: fmt
	@echo "Running cargo clippy --fix"
	cargo clippy --fix --all-features --allow-dirty --allow-staged

docker-compose-up DOCKER_SERVICES="all":
	@echo "Launching {{DOCKER_SERVICES}} Docker service(s)"
	COMPOSE_PROFILES={{DOCKER_SERVICES}} docker compose -f docker-compose.yml up -d --remove-orphans --wait

docker-compose-down:
	docker compose -f docker-compose.yml down --remove-orphans

test-all: docker-compose-up
	sqlx migrate run --database-url {{PG_DATABASE_URL}} --source ./mi-db/migrations
	DATABASE_URL={{PG_DATABASE_URL}} MI_TEST_REDIS_URL={{REDIS_URL}} cargo test --all-features

update-db-schema: docker-compose-up
	sqlx migrate run --database-url {{PG_DATABASE_URL}} --source ./mi-db/migrations	
	cd mi-db && cargo sqlx prepare --database-url {{PG_DATABASE_URL}}

install-ui-deps:
	cd mi-ui && npm install

export-ui: install-ui-deps
	cd mi-ui && npm run export

host: export-ui
	cd mi-api && cargo run

host-release: export-ui
	cd mi-api && cargo run --release

# Builds the docker image using the .env file
docker-build:
	docker build -t mi-api . \
	--build-arg DATABASE_URL=$DATABASE_URL \
	--build-arg MI_REDIS_URL=$MI_REDIS_URL \
	--build-arg MI_AUTH_REDIRECT_URi=$MI_AUTH_REDIRECT_URI \
	--build-arg PORT=$PORT \
	--build-arg OSU_CLIENT_ID=$OSU_CLIENT_ID \
	--build-arg OSU_CLIENT_SECRET=$OSU_CLIENT_SECRET \
	--build-arg OSU_REDIRECT_URI=$OSU_REDIRECT_URI \
	--build-arg RUST_LOG=$RUST_LOG
