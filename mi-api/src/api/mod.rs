use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_auth::{AuthBearerCustom, Rejection};

use crate::result::AppResult;
use crate::SessionError;

pub mod auth;
pub mod html;
pub mod influence;
pub mod leaderboard;
pub mod redoc;
pub mod user;

pub fn get_bearer_auth(bearer_auth: BearerAuth) -> AppResult<Option<u128>> {
    match bearer_auth.0 {
        Some(token) => Ok(Some(token.parse().map_err(|_| SessionError::CookieError)?)),
        None => Ok(None),
    }
}

pub struct BearerAuth(Option<String>);

impl AuthBearerCustom for BearerAuth {
    const ERROR_CODE: hyper::StatusCode = hyper::StatusCode::BAD_REQUEST;
    const ERROR_OVERWRITE: Option<&'static str> = None;

    fn from_header(contents: &str) -> Self {
        Self(Some(contents.to_string()))
    }
}

#[async_trait]
impl<B> FromRequestParts<B> for BearerAuth
where
    B: Send + Sync,
{
    type Rejection = Rejection;

    async fn from_request_parts(parts: &mut Parts, _: &B) -> Result<Self, Self::Rejection> {
        if let Ok(auth) = Self::decode_request_parts(parts) {
            Ok(auth)
        } else {
            Ok(Self(None))
        }
    }
}
