use axum::debug_handler;
use axum::extract::{State, Path};
use mi_db::user::User as DbUser;
use mi_osu_api::user::User as OsuUser;
use serde::Deserialize;
use tower_cookies::Cookies;

use super::get_session_cookie;
use crate::result::{AppResult, Json};
use crate::state::SharedState;

#[debug_handler]
pub async fn get_user(
    cookies: Cookies,
    State(state): State<SharedState>,
    Path(path_user_id): Path<Option<i64>>,
) -> AppResult<Json<DbUser>> {
    let token = get_session_cookie(&cookies)?;
    let user_id = state.redis().get_user_id(token).await?;

    let user_id = match path_user_id {
        Some(id) => id,
        None => user_id,
    };

    let user = state.postgres().get_user(user_id).await?;

    Ok(Json(user))
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    user_id: i64,
}

#[debug_handler]
pub async fn create_user(
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(request): Json<CreateUserRequest>,
) -> AppResult<Json<OsuUser>> {
    let token = get_session_cookie(&cookies)?;
    let user_id = state.redis().get_user_id(token).await?;
    let osu_token = state.redis().get_access_token(user_id).await?;

    let user = state
        .http()
        .request_osu_user(&osu_token, request.user_id)
        .await?;

    state.postgres().insert_user(user.clone().into()).await?;

    Ok(Json(user))
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    user_name: Option<String>,
    profile_picture: Option<String>,
    // Missing value -> None,
    // Null value -> Some(None)
    // Existing value -> Some(Some("Value"))
    bio: Option<Option<String>>,
}

#[debug_handler]
pub async fn update_user(
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(request): Json<UpdateUserRequest>,
) -> AppResult<()> {
    let token = get_session_cookie(&cookies)?;
    let user_id = state.redis().get_user_id(token).await?;

    if let Some(user_name) = request.user_name {
        state
            .postgres()
            .update_user_name(&user_name, user_id)
            .await?;
    }
    if let Some(profile_picture) = request.profile_picture {
        state
            .postgres()
            .update_user_picture(&profile_picture, user_id)
            .await?;
    }
    if let Some(bio) = request.bio {
        state
            .postgres()
            .update_user_bio(bio.as_deref(), user_id)
            .await?;
    }

    Ok(())
}
