use axum::debug_handler;
use axum::extract::State;
use mi_db::influence::Influence;
use mi_db::InfluenceResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::result::{AppResult, Json};
use crate::state::SharedState;
use crate::AuthUserId;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetInfluenceRequest {
    user_id: i64,
}

#[utoipa::path(
    post,
    path = "/influence/get/",
    request_body = Option<GetInfluenceRequest>,
    responses((status = 200, description = "List of influences", body = [Influence])),
)]
#[debug_handler]
pub async fn get_influences(
    AuthUserId(user_id): AuthUserId,
    State(state): State<SharedState>,
    Json(request): Json<Option<GetInfluenceRequest>>,
) -> AppResult<Json<Vec<InfluenceResponse>>> {
    let user_id = match request {
        Some(req) => req.user_id,
        None => user_id,
    };
    let influences = state.postgres().get_user_influencers(user_id).await?;

    Ok(Json(influences))
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct InsertInfluenceRequest {
    from_id: i64,
    #[schema(minimum = 1, maximum = 9)]
    #[validate(range(min = 1, max = 9))]
    level: i32,
    info: Option<String>,
}

#[utoipa::path(
    post,
    path = "/influence/create/",
    request_body = InsertInfluenceRequest,
    responses((status = 200, description = "Influence successfully created")),
)]
#[debug_handler]
pub async fn create_influence(
    AuthUserId(user_id): AuthUserId,
    State(state): State<SharedState>,
    Json(request): Json<InsertInfluenceRequest>,
) -> AppResult<()> {
    request.validate()?;

    let influence = Influence::new(request.from_id, user_id, request.level, request.info);
    state.postgres().insert_influence(influence).await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteInfluenceRequest {
    from_id: i64,
}

#[utoipa::path(
    delete,
    path = "/influence/delete/",
    request_body = DeleteInfluenceRequest,
    responses((status = 200, description = "Influence successfully deleted")),
)]
#[debug_handler]
pub async fn delete_influence(
    AuthUserId(user_id): AuthUserId,
    State(state): State<SharedState>,
    Json(request): Json<DeleteInfluenceRequest>,
) -> AppResult<()> {
    state
        .postgres()
        .delete_influence(request.from_id, user_id)
        .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateInfluenceLevelRequest {
    from_id: i64,
    #[schema(minimum = 1, maximum = 9)]
    #[validate(range(min = 1, max = 9))]
    level: i32,
}

#[utoipa::path(
    post,
    path = "/influence/update/level",
    request_body = UpdateInfluenceLevelRequest,
    responses((status = 200, description = "Influence level successfully updated")),
)]
#[debug_handler]
pub async fn update_influence_level(
    AuthUserId(user_id): AuthUserId,
    State(state): State<SharedState>,
    Json(request): Json<UpdateInfluenceLevelRequest>,
) -> AppResult<()> {
    request.validate()?;

    state
        .postgres()
        .update_influence_level(request.from_id, user_id, request.level)
        .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateInfluenceInfoRequest {
    from_id: i64,
    #[schema(nullable)]
    info: Option<String>,
}

#[utoipa::path(
    post,
    path = "/influence/update/info",
    request_body = UpdateInfluenceInfoRequest,
    responses((status = 200, description = "Influence info successfully updated")),
)]
#[debug_handler]
pub async fn update_influence_info(
    AuthUserId(user_id): AuthUserId,
    State(state): State<SharedState>,
    Json(request): Json<UpdateInfluenceInfoRequest>,
) -> AppResult<()> {
    state
        .postgres()
        .update_influence_info(request.from_id, user_id, request.info.as_deref())
        .await?;

    Ok(())
}
