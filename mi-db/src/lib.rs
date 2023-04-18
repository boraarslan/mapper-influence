pub mod auth;
pub mod influence;
pub mod user;
pub mod user_lock;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub use crate::influence::*;
pub use crate::user::*;

pub type RedisPool = Pool<RedisConnectionManager>;

/// Unique key violation error code for PostgreSQL
const PG_UNIQUE_KEY_VIOLATION: &str = "23505";
/// Foreign key violation error code for PostgreSQL
const PG_FOREIGN_KEY_VIOLATION: &str = "23503";
