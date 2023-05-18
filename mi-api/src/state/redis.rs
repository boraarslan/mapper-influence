use axum::extract::FromRef;
use mi_core::future_log_ext::FutureLogExt;
use mi_db::auth::AuthResult;
use mi_db::user_lock::LockError;
use mi_db::RedisPool;
use tracing::instrument;

use super::SharedState;

#[derive(Debug, Clone)]
pub struct RedisDb {
    pool: RedisPool,
}

impl RedisDb {
    pub async fn new() -> Self {
        let url = std::env::var("MI_REDIS_URL").expect("Redis URL is not set!");
        let conn = bb8_redis::RedisConnectionManager::new(url)
            .expect("Error while constructing Redis connection!");
        let pool = bb8::Pool::builder()
            .build(conn)
            .await
            .expect("Error while constructing Redis connection!");

        Self { pool }
    }

    #[instrument(skip(self, session_token), fields(elapsed), ret)]
    pub async fn get_user_id(&self, session_token: u128) -> AuthResult<i64> {
        mi_db::get_user_id(session_token, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed))]
    pub async fn get_access_token(&self, user_id: i64) -> AuthResult<String> {
        mi_db::get_access_token(user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed))]
    pub async fn get_refresh_token(&self, user_id: i64) -> AuthResult<String> {
        mi_db::get_refresh_token(user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self, session_token), fields(elapsed), ret)]
    pub async fn set_session_token(&self, user_id: i64, session_token: u128) -> AuthResult<()> {
        mi_db::set_session_token(user_id, session_token, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self, access_token, refresh_token), fields(elapsed), ret)]
    pub async fn set_osu_tokens(
        &self,
        user_id: i64,
        access_token: &str,
        refresh_token: &str,
    ) -> AuthResult<()> {
        mi_db::set_osu_tokens(user_id, access_token, refresh_token, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn lock_user(&self, user_id: i64) -> Result<(), LockError> {
        mi_db::lock_user(user_id, &self.pool).log_elapsed().await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn is_user_locked(&self, user_id: i64) -> Result<bool, LockError> {
        mi_db::is_user_locked(user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn unlock_user(&self, user_id: i64) -> Result<(), LockError> {
        mi_db::unlock_user(user_id, &self.pool).log_elapsed().await
    }
}

impl FromRef<SharedState> for RedisDb {
    fn from_ref(state: &SharedState) -> Self {
        state.redis.clone()
    }
}
