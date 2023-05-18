use axum::extract::rejection::JsonRejection;
use axum::response::IntoResponse;
use axum_macros::FromRequest;
use mi_core::AppErrorExt;
use mi_db::{AuthError, InfluenceError, LockError, UserError};
use mi_osu_api::OsuApiError;
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

use crate::api::leaderboard::LeaderboardError;
use crate::state::DB_POOL;
use crate::SessionError;

/// Custom JSON extractor that returns `AppError` if deserialization failed.
///
/// The reason of this structs existence is simply because we want to customize
/// the rejection handling for JSON deserialization errors.
#[derive(Debug, FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

/// Result returned by the application.
pub type AppResult<T> = Result<T, AppError>;

impl<T> From<AppError> for AppResult<T> {
    fn from(value: AppError) -> Self {
        Err(value)
    }
}

// In my defense:
//
// At first, I tried to return every different error by using a trait object
// (because you can't just return two different types using `impl AppErrorExt`)
// but that was troublesome and I couldn't get it to work. Might be because I'm
// trying to work on this project while I'm drunk, but I don't know. I'm awake while writing this.
// Anyways, its simpler to just add new errors to this enum and implement `AppErrorExt` for them.
// I'm sorry. (I'm not sorry, I will get drunk again)
#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    SessionError(#[from] SessionError),
    #[error(transparent)]
    LeaderboardError(#[from] LeaderboardError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    OsuApiError(#[from] OsuApiError),
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    InfluenceError(#[from] InfluenceError),
    #[error(transparent)]
    Validation(#[from] ValidationErrors),
    #[error(transparent)]
    LockError(#[from] LockError),
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
}

// Yeah, check the upper comment
impl From<AppError> for Box<dyn AppErrorExt> {
    fn from(value: AppError) -> Self {
        match value {
            AppError::SessionError(e) => Box::new(e),
            AppError::LeaderboardError(e) => Box::new(e),
            AppError::AuthError(e) => Box::new(e),
            AppError::OsuApiError(e) => Box::new(e),
            AppError::UserError(e) => Box::new(e),
            AppError::InfluenceError(e) => Box::new(e),
            AppError::Validation(e) => Box::new(e),
            AppError::LockError(e) => Box::new(e),
            AppError::JsonRejection(e) => Box::new(e),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let box_err: Box<dyn AppErrorExt> = self.into();

        if box_err.should_save() {
            let err_db_obj = box_err.as_db_error_object();
            tokio::spawn(async move {
                let result = err_db_obj
                    .insert_to_db(DB_POOL.get().expect(
                        "Tried to get DB pool before initialization. This should never happen.",
                    ))
                    .await;
                match result {
                    Ok(_) => (),
                    Err(e) => error!("Failed to save error to database: {}", e),
                }
            });
        }
        box_err.log_error();
        box_err.as_response()
    }
}
