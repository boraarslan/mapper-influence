# mapper-influence

[Api docs](https://mapperinfluence.me/api-docs/)

## Contributing

### How to run locally
- [Rust](https://www.rust-lang.org/learn/get-started) (MSRV >=v1.65)
- [Node.js](https://nodejs.org/)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [Justfile](https://github.com/casey/just) `cargo install just`
- Sqlx `cargo install sqlx-cli`

 1. Install the dependencies above
 2. Clone the repo
 3. Start Docker then run `just docker-compose-up`
 4. Run the migrations with `just update-db-schema`
 5. Start the server with `just host`

### How to run tests

 1. Install SQLx CLI (<https://crates.io/crates/sqlx-cli>)
 2. Start Docker then run `just docker-compose-up`
 3. Run the tests with `just test-all`
