use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_auth::{AuthBearerCustom, Rejection};
use tower_cookies::Cookies;

use crate::result::{AppError, AppResult};

pub mod auth;
pub mod html;
pub mod influence;
pub mod user;

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

pub fn get_bearer_auth(bearer_auth: BearerAuth) -> AppResult<Option<u128>> {
    match bearer_auth.0 {
        Some(token) => Ok(Some(token.parse().map_err(|_| AppError::cookie_error())?)),
        None => Ok(None),
    }
}

// pub fn get_auth_token(cookies: &Cookies, bearer_auth: BearerAuth) -> AppResult<u128> {
//     let bearer_token_opt = get_bearer_auth(bearer_auth)?;
//     let auth_token = match bearer_token_opt {
//         Some(bearer_token) => bearer_token,
//         None => {
//             let session_token = get_session_cookie(cookies)?;
//             match session_token {
//                 Some(token) => token,
//                 None => return Err(AppError::cookie_error()),
//             }
//         }
//     };

//     Ok(auth_token)
// }

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
