use axum::debug_handler;
use axum::extract::{Path, State};
use mi_db::{FullUser, User};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::result::{AppResult, Json};
use crate::state::SharedState;
use crate::AuthUserId;

#[utoipa::path(
    get,
    path = "/user/get",
    responses((status = 200, description = "User info found", body = User)),
)]
#[debug_handler]
pub async fn get_user(
    AuthUserId(user_id): AuthUserId,
    State(state): State<SharedState>,
) -> AppResult<Json<User>> {
    let db_user_res = state.postgres().get_user(user_id).await;

    match db_user_res {
        Ok(db_user) => Ok(Json(db_user)),
        Err(err) => {
            if let mi_db::UserError::UserNotFound(_) = err {
                let db_user = init_missing_user(&state, user_id, user_id).await?;
                Ok(Json(db_user))
            } else {
                Err(err.into())
            }
        }
    }
}

#[utoipa::path(
    get,
    path = "/user/get/{user_id}",
    responses((status = 200, description = "User info found", body = User)),
    params(("user_id", description = "Osu! ID of the user. If not specified, defaults to session owner's ID")),
)]
#[debug_handler]
pub async fn get_user_by_id(
    AuthUserId(auth_user_id): AuthUserId,
    State(state): State<SharedState>,
    Path(query_user_id): Path<i64>,
) -> AppResult<Json<User>> {
    let db_user_res = state.postgres().get_user(query_user_id).await;

    match db_user_res {
        Ok(db_user) => Ok(Json(db_user)),
        Err(err) => {
            if let mi_db::UserError::UserNotFound(_) = err {
                let db_user = init_missing_user(&state, auth_user_id, query_user_id).await?;
                Ok(Json(db_user))
            } else {
                Err(err.into())
            }
        }
    }
}

#[utoipa::path(
    get,
    path = "/user/get/full",
    responses((status = 200, description = "User info found", body = FullUser)),
)]
#[debug_handler]
pub async fn get_full_user(
    AuthUserId(user_id): AuthUserId,
    State(state): State<SharedState>,
) -> AppResult<Json<FullUser>> {
    let db_user_res = state.postgres().get_full_user(user_id).await;

    match db_user_res {
        Ok(db_user) => {
            if db_user.is_outdated() {
                update_user_profile(&state, user_id, user_id).await?;
            }
            Ok(Json(db_user))
        }
        Err(err) => {
            if let mi_db::UserError::UserNotFound(_) = err {
                init_missing_user(&state, user_id, user_id).await?;
                let full_user = state.postgres().get_full_user(user_id).await?;
                Ok(Json(full_user))
            } else {
                Err(err.into())
            }
        }
    }
}

#[utoipa::path(
    get,
    path = "/user/get/{user_id}/full",
    responses((status = 200, description = "User info found", body = FullUser)),
    params(("user_id", description = "Osu! ID of the user. If not specified, defaults to session owner's ID")),
)]
#[debug_handler]
pub async fn get_full_user_by_id(
    AuthUserId(auth_user_id): AuthUserId,
    State(state): State<SharedState>,
    Path(query_user_id): Path<i64>,
) -> AppResult<Json<FullUser>> {
    let db_user_res = state.postgres().get_full_user(query_user_id).await;

    match db_user_res {
        Ok(db_user) => {
            if db_user.is_outdated() {
                update_user_profile(&state, auth_user_id, query_user_id).await?;
            }
            Ok(Json(db_user))
        }
        Err(err) => {
            if let mi_db::UserError::UserNotFound(_) = err {
                init_missing_user(&state, auth_user_id, query_user_id).await?;
                let full_user = state.postgres().get_full_user(query_user_id).await?;
                Ok(Json(full_user))
            } else {
                Err(err.into())
            }
        }
    }
}

async fn update_user_profile(
    state: &SharedState,
    requester_user_id: i64,
    user_id_to_update: i64,
) -> AppResult<()> {
    if state.redis().is_user_locked(user_id_to_update).await? {
        return Ok(());
    }

    state.redis().lock_user(user_id_to_update).await?;

    let osu_token = state.redis().get_access_token(requester_user_id).await?;

    let osu_user = state
        .http()
        .request_osu_user(&osu_token, user_id_to_update)
        .await?;

    state.postgres().update_user_osu_data(osu_user).await?;

    state.redis().unlock_user(user_id_to_update).await?;

    Ok(())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    user_id: i64,
}

#[utoipa::path(
    post,
    path = "/user/create",
    request_body = CreateUserRequest,
    responses((status = 200, description = "User successfully created", body = User))
)]
#[debug_handler]
pub async fn create_user(
    AuthUserId(user_id): AuthUserId,
    State(state): State<SharedState>,
    Json(request): Json<CreateUserRequest>,
) -> AppResult<Json<User>> {
    let user = init_missing_user(&state, user_id, request.user_id).await?;

    Ok(Json(user))
}

async fn init_missing_user(
    state: &SharedState,
    user_id: i64,
    missing_user_id: i64,
) -> AppResult<User> {
    let osu_token = state.redis().get_access_token(user_id).await?;

    let osu_user = state
        .http()
        .request_osu_user(&osu_token, missing_user_id)
        .await?;

    let user = state
        .postgres()
        .insert_user(osu_user.clone().into())
        .await?;

    state.postgres().update_user_osu_data(osu_user).await?;

    Ok(user)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    user_name: Option<String>,
    profile_picture: Option<String>,
    // Missing value -> None,
    // Null value -> Some(None)
    // Existing value -> Some(Some("Value"))
    bio: Option<Option<String>>,
}

#[utoipa::path(
    post,
    path = "/user/update",
    request_body = UpdateUserRequest,
    responses((status = 200, description = "User successfully updated"))
)]
#[debug_handler]
pub async fn update_user(
    AuthUserId(user_id): AuthUserId,
    State(state): State<SharedState>,
    Json(request): Json<UpdateUserRequest>,
) -> AppResult<()> {
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
