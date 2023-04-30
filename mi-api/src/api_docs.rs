use utoipa::openapi::security::{ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::api;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::user::get_user,
        api::user::get_full_user,
        api::user::create_user,
        api::user::update_user,
        api::influence::get_influences,
        api::influence::create_influence,
        api::influence::delete_influence,
        api::influence::update_influence_level,
        api::influence::update_influence_info,
        api::leaderboard::get_user_leaderboard,
    ),
    components(schemas(
        mi_db::User,
        mi_db::FullUser,
        mi_db::FeaturedMaps,
        mi_db::Maps,
        mi_db::Influence,
        mi_db::LeaderboardUser,
        mi_osu_api::Beatmapset,
        mi_osu_api::BeatmapsetNames,
        mi_osu_api::Beatmap ,
        mi_osu_api::Covers ,
        mi_osu_api::BeatmapType,
        api::user::CreateUserRequest,
        api::user::UpdateUserRequest,
        api::influence::GetInfluenceRequest,
        api::influence::InsertInfluenceRequest,
        api::influence::DeleteInfluenceRequest,
        api::influence::UpdateInfluenceLevelRequest,
        api::influence::UpdateInfluenceInfoRequest,
    )),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Cookie(
                    ApiKeyValue::new("mi-session-token"),
                )),
            )
        }
    }
}
