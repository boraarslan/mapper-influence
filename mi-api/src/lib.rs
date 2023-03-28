use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use hyper::StatusCode;
use result::{AppResult, AppError};
use state::AuthUser;
use tower_cookies::Cookies;
use utoipa::openapi::security::{ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

pub mod api;
pub mod result;
pub mod state;

const COOKIE_NAME: &str = "mi-session-token";

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
        mi_db::User,
        mi_db::FullUser,
        mi_db::FeaturedMaps,
        mi_db::Maps,
        mi_db::Influence,
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


pub fn get_session_cookie(cookies: &Cookies) -> AppResult<u128> {
    match cookies.get(COOKIE_NAME) {
        Some(cookie) => Ok(cookie
            .value()
            .parse()
            .map_err(|_| AppError::cookie_error())?),
        None => Err(AppError::cookie_error()),
    }
}
pub struct AuthUserId(i64);

#[async_trait::async_trait]
impl<S: AuthUser + Sync + Send> FromRequestParts<S> for AuthUserId {
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state).await?;
        if let Ok(user_id) = state.auth_user(cookies).await {
            Ok(AuthUserId(user_id))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
        }
    }

}
