pub mod auth;
pub mod influence;
pub mod user;

pub use crate::user::*;
pub use crate::influence::*;

/// Unique key violation error code for PostgreSQL
const PG_UNIQUE_KEY_VIOLATION: &str = "23505";
/// Foreign key violation error code for PostgreSQL
const PG_FOREIGN_KEY_VIOLATION: &str = "23503";
