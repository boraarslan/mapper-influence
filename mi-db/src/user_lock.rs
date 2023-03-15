use bb8::RunError;
use redis::RedisError;
use thiserror::Error;

use crate::RedisPool;

pub async fn lock_user(user_id: i64, db: &RedisPool) -> Result<(), LockError> {
    let mut conn = db.get().await?;
    let mut cmd = redis::Cmd::new();
    let key = format!("user:lock:{}", user_id);

    cmd.arg("SET")
        .arg(&key)
        .arg(1)
        .arg("NX")
        .arg("PX")
        .arg(30000);

    cmd.query_async(&mut *conn).await?;

    Ok(())
}

pub async fn is_user_locked(user_id: i64, db: &RedisPool) -> Result<bool, LockError> {
    let mut conn = db.get().await?;
    let mut cmd = redis::Cmd::new();
    let key = format!("user:lock:{}", user_id);

    cmd.arg("GET").arg(&key);

    let value: Option<i64> = cmd.query_async(&mut *conn).await?;

    Ok(value.is_some())
}

pub async fn unlock_user(user_id: i64, db: &RedisPool) -> Result<(), LockError> {
    let mut conn = db.get().await?;
    let mut cmd = redis::Cmd::new();
    let key = format!("user:lock:{}", user_id);

    cmd.arg("DEL").arg(&key);

    cmd.query_async(&mut *conn).await?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum LockError {
    #[error("Redis database returned an error {0}")]
    RedisError(#[from] RedisError),
    #[error("Getting a connection from pool took too long.")]
    ConnectionTimedOut,
}

impl From<RunError<RedisError>> for LockError {
    fn from(err: RunError<RedisError>) -> Self {
        match err {
            RunError::TimedOut => LockError::ConnectionTimedOut,
            RunError::User(err) => LockError::from(err),
        }
    }
}
