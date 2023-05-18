use std::future::Future;

use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use hyper::StatusCode;
use mi_core::AppErrorExt;
use result::AppResult;
use state::AuthUser;
use thiserror::Error;
use tokio::time::Instant;
use tower_cookies::Cookies;
use tracing::error;

pub mod api;
pub mod api_docs;
pub mod request_id;
pub mod result;
pub mod state;
pub mod traces;

const COOKIE_NAME: &str = "mi-session-token";

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Unable to get session cookie")]
    CookieError,
    #[error("User's session is expired")]
    SessionExpired,
    #[error("Osu auth error: {0}")]
    OsuAuthError(String),
}

impl AppErrorExt for SessionError {
    fn user_message(&self) -> String {
        match self {
            SessionError::CookieError => self.to_string(),
            SessionError::SessionExpired => self.to_string(),
            SessionError::OsuAuthError(_) => "Unable to authorize with osu!".to_string(),
        }
    }

    fn error_type(&self) -> mi_core::ErrorType {
        match self {
            SessionError::CookieError => mi_core::ErrorType::AuthorizatonError,
            SessionError::SessionExpired => mi_core::ErrorType::AuthorizatonError,
            SessionError::OsuAuthError(_) => mi_core::ErrorType::AuthorizatonError,
        }
    }

    fn log_error(&self) {
        match self {
            SessionError::CookieError => self.log_error(),
            SessionError::SessionExpired => self.log_error(),
            SessionError::OsuAuthError(_) => error!("{}", self),
        }
    }
}

pub fn get_session_cookie(cookies: &Cookies) -> AppResult<u128> {
    match cookies.get(COOKIE_NAME) {
        Some(cookie) => Ok(cookie
            .value()
            .parse()
            .map_err(|_| SessionError::CookieError)?),
        None => Err(SessionError::CookieError.into()),
    }
}

pub struct AuthUserId(i64);

#[async_trait::async_trait]
impl<S: AuthUser + Sync + Send> FromRequestParts<S> for AuthUserId {
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state).await?;
        if let Ok(user_id) = state.auth_user(cookies).await {
            Ok(AuthUserId(user_id))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
        }
    }
}

pub async fn call_and_log_elapsed<T, E>(func: impl Future<Output = Result<T, E>>) -> Result<T, E> {
    let time = Instant::now();
    let res = func.await;
    let elapsed = time.elapsed();
    tracing::Span::current().record("elapsed", format!("{:.3?}", elapsed));
    res
}
