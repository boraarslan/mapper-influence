use axum::extract::State;
use mi_core::AppErrorExt;
use mi_db::leaderboard::LeaderboardUser;
use thiserror::Error;
use tracing::error;

use crate::result::{AppResult, Json};
use crate::state::SharedState;

#[derive(Debug, Error)]
pub enum LeaderboardError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::error::Error),
}

impl AppErrorExt for LeaderboardError {
    fn user_message(&self) -> String {
        match self {
            LeaderboardError::DatabaseError(_) => "Unable to get leaderboard".to_string(),
        }
    }

    fn error_type(&self) -> mi_core::ErrorType {
        match self {
            LeaderboardError::DatabaseError(_) => mi_core::ErrorType::DatabaseError,
        }
    }

    fn log_error(&self) {
        match self {
            LeaderboardError::DatabaseError(_) => error!("{}", self),
        }
    }
}

#[utoipa::path(
    get,
    path = "/leaderboard/user/",
    responses((status = 200, description = "List of top influences", body = [LeaderboardUser])),
)]
pub async fn get_user_leaderboard(
    State(state): State<SharedState>,
) -> AppResult<Json<Vec<LeaderboardUser>>> {
    let users = state.postgres().get_user_leaderboard().await?;

    Ok(Json(users))
}
