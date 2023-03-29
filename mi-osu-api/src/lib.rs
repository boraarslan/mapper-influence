//! osu! API library for Mapper Influence backend.
//! It is used for requesting data from [official osu! API].
//!
//! It is not a complete implementation of the API,
//! rather, only the endpoints, which are relevant to the website, are present.
//!
//! [official osu! API]: <https://osu.ppy.sh/docs/index.html>

use async_trait::async_trait;
use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;
use thiserror::Error;

pub mod auth;
pub mod beatmap;
pub mod user;

pub use crate::beatmap::*;
pub use crate::user::*;

pub type ReqwestError = reqwest::Error;

#[derive(Error, Debug)]
pub enum OsuApiError {
    #[error("Request failed with HTTP Status code.")]
    HTTPError { body: String, error: StatusCode },
    #[error("Request failed because of internal errors.")]
    InternalError(#[from] ReqwestError),
    #[error(
        "Invalid BeatmapType argument. Available variants for this method are Graveyard, Loved, \
         Pending and Ranked."
    )]
    InvalidBeatmapType,
}

#[async_trait]
pub trait ResponseWithBody<T> {
    async fn try_deserialising(self) -> Result<T, OsuApiError>;
}

#[async_trait]
impl<T> ResponseWithBody<T> for Response
where
    T: DeserializeOwned,
{
    async fn try_deserialising(self) -> Result<T, OsuApiError> {
        match self.error_for_status_ref() {
            Ok(_) => Ok(self.json::<T>().await?),
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
