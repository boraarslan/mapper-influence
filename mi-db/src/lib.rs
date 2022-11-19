pub mod auth;
pub mod influence;
pub mod user;

/// Unique key violation error code for PostgreSQL
const PG_UNIQUE_KEY_VIOLATION: &str = "23505";
/// Foreign key violation error code for PostgreSQL
const PG_FOREIGN_KEY_VIOLATION: &str = "23503";
