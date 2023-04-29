use axum::extract::State;
use mi_db::leaderboard::LeaderboardUser;

use crate::result::{AppResult, Json};
use crate::state::SharedState;

pub async fn get_user_leaderboard(
    State(state): State<SharedState>,
) -> AppResult<Json<Vec<LeaderboardUser>>> {
    let users = state.postgres().get_user_leaderboard().await?;

    Ok(Json(users))
}
