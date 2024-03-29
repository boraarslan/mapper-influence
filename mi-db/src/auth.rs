use bb8::RunError;
use mi_core::error::{AppErrorExt, ErrorType};
use mi_core::INTERNAL_DB_ERROR_MESSAGE;
use redis::RedisError;
use secrecy::{ExposeSecret, Secret};
use thiserror::Error;
use tracing::{error, warn};

use crate::RedisPool;

pub type AuthResult<T> = Result<T, AuthError>;

const SESSION_TOKEN_TIMEOUT: usize = 85800; // 23 hours 50 minutes
const ACCESS_TOKEN_TIMEOUT: usize = 43200; // 12 hours

pub async fn get_user_id(session_token: u128, db: &RedisPool) -> AuthResult<i64> {
    let mut conn = db.get().await?;
    let mut cmd = redis::Cmd::new();
    let key = format!("user:session:{}", session_token);
    cmd.arg("GET").arg(key);
    let token: Option<i64> = cmd.query_async(&mut *conn).await?;

    match token {
        Some(token) => Ok(token),
        None => Err(AuthError::ValueNotFound {
            value: Secret::new(session_token.to_string()),
            expected: "user_id",
        }),
    }
}

pub async fn get_access_token(user_id: i64, db: &RedisPool) -> AuthResult<String> {
    let mut conn = db.get().await?;
    let mut cmd = redis::Cmd::new();
    let key = format!("user:access:{}", user_id);

    cmd.arg("GET").arg(key);
    let token: Option<String> = cmd.query_async(&mut *conn).await?;

    match token {
        Some(token) => Ok(token),
        None => Err(AuthError::ValueNotFound {
            value: Secret::new(user_id.to_string()),
            expected: "access_token",
        }),
    }
}

pub async fn get_refresh_token(user_id: i64, db: &RedisPool) -> AuthResult<String> {
    let mut conn = db.get().await?;
    let mut cmd = redis::Cmd::new();
    let key = format!("user:refresh:{}", user_id);

    cmd.arg("GET").arg(key);
    let token: Option<String> = cmd.query_async(&mut *conn).await?;

    match token {
        Some(token) => Ok(token),
        None => Err(AuthError::ValueNotFound {
            value: Secret::new(user_id.to_string()),
            expected: "refresh_token",
        }),
    }
}

pub async fn set_session_token(
    user_id: i64,
    session_token: u128,
    db: &RedisPool,
) -> AuthResult<()> {
    let mut conn = db.get().await?;
    let mut cmd = redis::Cmd::new();
    let key = format!("user:session:{}", session_token);

    cmd.arg("SET").arg(&key).arg(user_id);
    cmd.arg("EX").arg(SESSION_TOKEN_TIMEOUT);
    cmd.query_async(&mut *conn).await?;

    Ok(())
}

pub async fn set_osu_tokens(
    user_id: i64,
    access_token: &str,
    refresh_token: &str,
    db: &RedisPool,
) -> AuthResult<()> {
    let mut conn = db.get().await?;
    let mut pipe = redis::pipe();
    let access_key = format!("user:access:{}", user_id);
    let refresh_key = format!("user:refresh:{}", user_id);

    pipe.cmd("SET")
        .arg(&access_key)
        .arg(access_token)
        .arg("EX")
        .arg(ACCESS_TOKEN_TIMEOUT)
        .ignore();

    pipe.cmd("SET")
        .arg(&refresh_key)
        .arg(refresh_token)
        .ignore();

    pipe.query_async(&mut *conn).await?;

    Ok(())
}

// TODO: Add record fields to errors.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Getting a connection from pool took too long.")]
    ConnectionTimedOut,
    #[error("Redis database returned an error {0}")]
    RedisError(#[from] RedisError),
    #[error("Database returned an empty response for the given key {value:?}")]
    ValueNotFound {
        value: Secret<String>,
        expected: &'static str,
    },
}

impl AppErrorExt for AuthError {
    fn user_message(&self) -> String {
        match self {
            AuthError::ValueNotFound { .. } => "Couldn't authorize the user".to_string(),
            AuthError::ConnectionTimedOut => INTERNAL_DB_ERROR_MESSAGE.to_string(),
            AuthError::RedisError(_) => INTERNAL_DB_ERROR_MESSAGE.to_string(),
        }
    }

    fn log_error(&self) {
        match self {
            AuthError::ValueNotFound { value, expected } => {
                if *expected == "user_id" {
                    warn!(
                        expected,
                        "Database returned an empty response for the given key: {:?}", value
                    )
                } else {
                    warn!(
                        expected,
                        "Database returned an empty response for the given key: {:?}",
                        value.expose_secret()
                    )
                }
            }
            AuthError::ConnectionTimedOut => {
                warn!("Getting a connection from Redis pool took too long.")
            }
            AuthError::RedisError(err) => error!("Redis database returned an error: {}", err),
        }
    }

    fn error_type(&self) -> ErrorType {
        match self {
            AuthError::ValueNotFound { .. } => ErrorType::AuthorizatonError,
            AuthError::ConnectionTimedOut => ErrorType::DatabaseError,
            AuthError::RedisError(_) => ErrorType::DatabaseError,
        }
    }
}

impl From<RunError<RedisError>> for AuthError {
    fn from(run_err: RunError<RedisError>) -> Self {
        match run_err {
            RunError::User(redis_err) => Self::from(redis_err),
            RunError::TimedOut => Self::ConnectionTimedOut,
        }
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod test {
    use super::*;

    async fn create_db_pool() -> RedisPool {
        dotenvy::dotenv().ok();
        let local_redis_url = std::env::var("MI_TEST_REDIS_URL").unwrap();
        let manager = bb8_redis::RedisConnectionManager::new(local_redis_url).unwrap();

        bb8::Pool::builder()
            .max_size(1)
            .build(manager)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_session_token() {
        let user_id = 1;
        let session_token = 12345;
        let db_pool = create_db_pool().await;

        set_session_token(user_id, session_token, &db_pool)
            .await
            .unwrap();
        let db_user_id = get_user_id(session_token, &db_pool).await.unwrap();
        assert_eq!(user_id, db_user_id);
    }

    #[tokio::test]
    async fn test_osu_tokens() {
        let user_id = 31;
        let access_token = "3112345";
        let refresh_token = "3212345";
        let db_pool = create_db_pool().await;

        set_osu_tokens(user_id, access_token, refresh_token, &db_pool)
            .await
            .unwrap();
        let db_access_token = get_access_token(user_id, &db_pool).await.unwrap();
        let db_refresh_token = get_refresh_token(user_id, &db_pool).await.unwrap();

        assert_eq!(access_token, db_access_token);
        assert_eq!(refresh_token, db_refresh_token);
    }
}
