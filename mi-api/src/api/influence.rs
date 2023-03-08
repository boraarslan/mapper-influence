use axum::debug_handler;
use axum::extract::State;
use mi_db::influence::Influence;
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;
use validator::Validate;

use crate::result::{AppResult, Json};
use crate::state::SharedState;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInfluenceRequest {
    user_id: Option<i64>,
}

#[debug_handler]
pub async fn get_influences(
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(request): Json<GetInfluenceRequest>,
) -> AppResult<Json<Vec<Influence>>> {
    let user_id = state.auth_user(&cookies).await?;

    let influences = state
        .postgres()
        .get_user_influencers(request.user_id.unwrap_or(user_id))
        .await?;

    Ok(Json(influences))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct InsertInfluenceRequest {
    from_id: i64,
    #[validate(range(min = 1, max = 9))]
    level: i32,
    info: Option<String>,
}

#[debug_handler]
pub async fn create_influence(
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(request): Json<InsertInfluenceRequest>,
) -> AppResult<()> {
    request.validate()?;
    let user_id = state.auth_user(&cookies).await?;

    let influence = Influence::new(request.from_id, user_id, request.level, request.info);
    state.postgres().insert_influence(influence).await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteInfluenceRequest {
    from_id: i64,
}

#[debug_handler]
pub async fn delete_influence(
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(request): Json<DeleteInfluenceRequest>,
) -> AppResult<()> {
    let user_id = state.auth_user(&cookies).await?;

    state
        .postgres()
        .delete_influence(request.from_id, user_id)
        .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateInfluenceLevelRequest {
    from_id: i64,
    #[validate(range(min = 1, max = 9))]
    level: i32,
}

#[debug_handler]
pub async fn update_influence_level(
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(request): Json<UpdateInfluenceLevelRequest>,
) -> AppResult<()> {
    request.validate()?;
    let user_id = state.auth_user(&cookies).await?;

    state
        .postgres()
        .update_influence_level(request.from_id, user_id, request.level)
        .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfluenceInfoRequest {
    from_id: i64,
    info: Option<String>,
}

#[debug_handler]
pub async fn update_influence_info(
    cookies: Cookies,
    State(state): State<SharedState>,
    Json(request): Json<UpdateInfluenceInfoRequest>,
) -> AppResult<()> {
    let user_id = state.auth_user(&cookies).await?;

    state
        .postgres()
        .update_influence_info(request.from_id, user_id, request.info.as_deref())
        .await?;

    Ok(())
}
