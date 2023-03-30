use std::future::Future;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use hyper::StatusCode;
use pin_project_lite::pin_project;
use result::{AppError, AppResult};
use state::AuthUser;
use tokio::time::Instant;
use tower_cookies::Cookies;

pub mod api;
pub mod api_docs;
pub mod request_id;
pub mod result;
pub mod state;

const COOKIE_NAME: &str = "mi-session-token";

pub fn get_session_cookie(cookies: &Cookies) -> AppResult<u128> {
    match cookies.get(COOKIE_NAME) {
        Some(cookie) => Ok(cookie
            .value()
            .parse()
            .map_err(|_| AppError::cookie_error())?),
        None => Err(AppError::cookie_error()),
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

pin_project! {
    pub struct LogElapsed<F> {
        #[pin]
        future: F,
    }
}

pub trait FutureLogExt: Future {
    fn log_elapsed(self) -> LogElapsed<Self>
    where
        Self: Sized,
    {
        LogElapsed { future: self }
    }
}

impl<F> Future for LogElapsed<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let time = Instant::now();
        let output = ready!(this.future.poll(cx));
        let elapsed = time.elapsed();
        tracing::Span::current().record("elapsed", format!("{:.3?}", elapsed));
        Poll::Ready(output)
    }
}

impl<T: ?Sized> FutureLogExt for T where T: Future {}
