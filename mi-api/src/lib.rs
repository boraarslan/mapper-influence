use utoipa::openapi::security::{ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

pub mod api;
pub mod result;
pub mod state;

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
    ),
    components(schemas(
        mi_db::user::User,
        mi_db::user::FullUser,
        mi_db::user::FeaturedMaps,
        mi_db::user::Maps,
        mi_db::influence::Influence,
        mi_osu_api::beatmap::Beatmapset,
        mi_osu_api::beatmap::BeatmapsetNames,
        mi_osu_api::beatmap::Beatmap ,
        mi_osu_api::beatmap::Covers ,
        mi_osu_api::beatmap::BeatmapType,
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
