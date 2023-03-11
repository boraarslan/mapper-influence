use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");
    // Ignore error if .env is not found
    _ = dotenv();
    // Only run migrations in CI
    let ci_env = std::env::var("MAPPER_INFLUENCE_CI_ENV");
    if ci_env.is_ok() && ci_env.unwrap().starts_with("BUILD") {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let db = PgPoolOptions::new()
            .max_connections(20)
            .connect(&db_url)
            .await
            .unwrap();

        sqlx::migrate!().run(&db).await.unwrap();
    }
}
