//! osu! API library for Mapper Influence backend.
//! It is used for requesting data from [official osu! API].
//!
//! It is not a complete implementation of the API,
//! rather, only the endpoints, which are relevant to the website, are present.
//!
//! [official osu! API]: <https://osu.ppy.sh/docs/index.html>

use async_trait::async_trait;
use mi_core::{AppErrorExt, ErrorType, TryDeserialize, INTERNAL_SERVER_ERROR_MESSAGE};
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tracing::{error, info, warn};

pub mod auth;
pub mod beatmap;
pub mod user;

pub use crate::beatmap::*;
pub use crate::user::*;

pub type ReqwestError = reqwest::Error;

#[derive(Error, Debug)]
pub enum OsuApiError {
    #[error("Request failed with HTTP Status code {}", error)]
    HTTPError { body: String, error: StatusCode },
    // For errors that are not caused by error status codes
    #[error("An internal error has occurred.")]
    InternalError(#[from] ReqwestError),
    #[error("Failed to deserialize response.")]
    DeserializeError(#[from] mi_core::DeserializeError),
    #[error(
        "Invalid BeatmapType argument. Available variants for this method are Graveyard, Loved, \
         Pending and Ranked."
    )]
    InvalidBeatmapType,
    #[error("An internal error has occurred.")]
    JwtParseError(#[from] jwt::error::Error),
    #[error("Missing public scope in access token.")]
    PublicScopeError,
}

#[async_trait]
pub trait ResponseWithBody<T> {
    async fn try_deser_api_response(self) -> Result<T, OsuApiError>;
}

#[async_trait]
impl<T> ResponseWithBody<T> for Response
where
    T: DeserializeOwned,
{
    async fn try_deser_api_response(self) -> Result<T, OsuApiError> {
        match self.error_for_status_ref() {
            Ok(_) => Ok(self.text().await?.try_deserialize()?),
            Err(err) => {
                let status = err.status().unwrap();
                let body = self.text().await?;
                Err(OsuApiError::HTTPError {
                    body,
                    error: status,
                })
            }
        }
    }
}

impl AppErrorExt for OsuApiError {
    fn user_message(&self) -> String {
        match self {
            OsuApiError::HTTPError { .. } => INTERNAL_SERVER_ERROR_MESSAGE.to_string(),
            OsuApiError::InternalError(_) => INTERNAL_SERVER_ERROR_MESSAGE.to_string(),
            OsuApiError::DeserializeError(err) => err.user_message(),
            OsuApiError::InvalidBeatmapType => INTERNAL_SERVER_ERROR_MESSAGE.to_string(),
            OsuApiError::JwtParseError(_) => INTERNAL_SERVER_ERROR_MESSAGE.to_string(),
            OsuApiError::PublicScopeError => self.to_string(),
        }
    }

    fn error_type(&self) -> ErrorType {
        match self {
            OsuApiError::HTTPError { .. } => ErrorType::OsuApiError,
            OsuApiError::InternalError(_) => ErrorType::HttpClientError,
            OsuApiError::DeserializeError(err) => err.error_type(),
            OsuApiError::InvalidBeatmapType => ErrorType::BadRequestData,
            OsuApiError::JwtParseError(_) => ErrorType::OsuApiError,
            OsuApiError::PublicScopeError => ErrorType::OsuApiScopeError,
        }
    }

    fn log_error(&self) {
        match self {
            OsuApiError::HTTPError { body, error } => {
                warn!(body, "OsuApiError: {}", error);
            }
            OsuApiError::InternalError(err) => error!("Reqwest client failed: {}", err),
            OsuApiError::DeserializeError(err) => err.log_error(),
            OsuApiError::InvalidBeatmapType => warn!("{}", self),
            OsuApiError::JwtParseError(err) => error!("JWT parsing failed: {}", err),
            OsuApiError::PublicScopeError => info!("{}", self),
        }
    }
}
