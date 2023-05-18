use std::fmt::Display;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use thiserror::Error;
use tracing::error;

use crate::{AppErrorExt, ErrorType, INTERNAL_SERVER_ERROR_MESSAGE};

#[derive(Debug, Error)]
pub struct DeserializeError {
    #[source]
    pub error: serde_json::Error,
    pub data: String,
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

pub trait TryDeserialize<D> {
    fn try_deserialize(&self) -> Result<D, DeserializeError>;
}

impl<'a, D, T> TryDeserialize<D> for T
where
    D: DeserializeOwned,
    T: Deserialize<'a> + ToString,
{
    fn try_deserialize(&self) -> Result<D, DeserializeError> {
        let self_copy = self.to_string();
        serde_json::from_slice(self_copy.as_bytes()).map_err(|error| DeserializeError {
            error,
            data: self_copy,
        })
    }
}

impl AppErrorExt for DeserializeError {
    fn user_message(&self) -> String {
        INTERNAL_SERVER_ERROR_MESSAGE.to_string()
    }

    fn error_type(&self) -> crate::ErrorType {
        ErrorType::DeserializeError
    }

    fn log_error(&self) {
        error!(
            error = %self.error,
            "Unable to deserialize data into expected type",
        );
    }

    fn error_message(&self) -> String {
        self.to_string()
    }

    fn should_save(&self) -> bool {
        true
    }

    fn error_data(&self) -> Option<serde_json::Value> {
        self.data.try_deserialize().ok()
    }

    fn as_db_error_object(&self) -> crate::DbErrorObject {
        crate::DbErrorObject::new(
            self.error_type(),
            self.error_message(),
            self.error_data().unwrap_or_default(),
        )
    }
}
