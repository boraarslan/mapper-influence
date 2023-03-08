pub mod http;
mod postgres;
pub mod redis;

use std::sync::Arc;

use parking_lot::{Mutex, MutexGuard};
use rand_chacha::rand_core::{OsRng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use tower_cookies::Cookies;

pub use self::http::HttpClient;
pub use self::postgres::PgDb;
pub use self::redis::RedisDb;
use crate::api::get_session_cookie;
use crate::result::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct SharedState {
    http_client: HttpClient,
    redis: RedisDb,
    postgres: PgDb,
    random: Arc<Mutex<ChaCha8Rng>>,
}

impl SharedState {
    pub async fn new() -> Self {
        let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
        let random = Arc::new(Mutex::new(random));

        Self {
            http_client: HttpClient::new(),
            redis: RedisDb::new().await,
            postgres: PgDb::new().await,
            random,
        }
    }

    /// Authanticates user from session token
    ///
    /// If successful, returns user's Osu! id
    /// If user's session token is expired or it does not exist in DB, redirects user to /login
    pub async fn auth_user(&self, cookies: &Cookies) -> AppResult<i64> {
        let token = get_session_cookie(cookies)?;
        let auth_res = self.redis().get_user_id(token).await;

        if let Err(mi_db::auth::AuthError::ValueNotFound) = auth_res {
            Err(AppError::session_expired())
        } else {
            Ok(auth_res?)
        }
    }

    pub fn http(&self) -> &HttpClient {
        &self.http_client
    }

    pub fn redis(&self) -> &RedisDb {
        &self.redis
    }

    pub fn postgres(&self) -> &PgDb {
        &self.postgres
    }

    pub fn random(&self) -> MutexGuard<ChaCha8Rng> {
        self.random.lock()
    }

    pub fn generate_session_token(&self) -> u128 {
        let mut random = self.random();
        let mut u128_pool = [0u8; 16];
        random.fill_bytes(&mut u128_pool);
        u128::from_le_bytes(u128_pool)
    }
}

impl AsRef<PgDb> for SharedState {
    fn as_ref(&self) -> &PgDb {
        &self.postgres
    }
}

impl AsRef<RedisDb> for SharedState {
    fn as_ref(&self) -> &RedisDb {
        &self.redis
    }
}
