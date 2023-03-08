use axum::debug_handler;
use axum::extract::{Path, State};
use mi_db::user::User as DbUser;
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::result::{AppResult, Json};
use crate::state::SharedState;

#[debug_handler]
pub async fn get_user(
    cookies: Cookies,
    State(state): State<SharedState>,
    Path(path_user_id_opt): Path<Option<i64>>,
) -> AppResult<Json<DbUser>> {
    let auth_user_id = state.auth_user(&cookies).await?;

    let query_user_id = match path_user_id_opt {
        Some(path_user_id) => path_user_id,
        None => auth_user_id,
    };

    let db_user_res = state.postgres().get_user(query_user_id).await;

    match db_user_res {
        Ok(db_user) => Ok(Json(db_user)),
        Err(err) => {
            if let mi_db::user::UserError::UserNotFound(_) = err {
                let db_user = init_missing_user(state, auth_user_id, query_user_id).await?;
                Ok(Json(db_user))
            } else {
                Err(err.into())
            }
        }
    }
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
) -> AppResult<Json<DbUser>> {
    let user_id = state.auth_user(&cookies).await?;

    let user = init_missing_user(state, user_id, request.user_id).await?;

    Ok(Json(user))
}

async fn init_missing_user(
    state: SharedState,
    user_id: i64,
    missing_user_id: i64,
) -> AppResult<DbUser> {
    let osu_token = state.redis().get_access_token(user_id).await?;

    let osu_user = state
        .http()
        .request_osu_user(&osu_token, missing_user_id)
        .await?;

    let user = state
        .postgres()
        .insert_user(osu_user.clone().into())
        .await?;

    Ok(user)
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
    let user_id = state.auth_user(&cookies).await?;

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
