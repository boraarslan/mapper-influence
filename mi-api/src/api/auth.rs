use axum::debug_handler;
use axum::extract::{Query, State};
use axum::response::Redirect;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use super::get_session_cookie;
use crate::api::COOKIE_NAME;
use crate::result::AppResult;
use crate::state::SharedState;

#[derive(Debug, Deserialize)]
pub struct OsuAuthResponseParams {
    code: String,
}

#[debug_handler]
pub async fn authorize_from_osu_api(
    Query(params): Query<OsuAuthResponseParams>,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> AppResult<Redirect> {
    let auth_response = state.http().get_osu_access_token(params.code).await?;
    info!("Successfully got the auth response");
    let user = state
        .http()
        .request_osu_token_user(&auth_response.access_token)
        .await?;
    info!("Successfully got the Osu! user");

    let session_token = state.generate_session_token();

    tokio::try_join!(
        state.redis().set_session_token(user.id, session_token),
        state.redis().set_osu_tokens(
            user.id,
            &auth_response.access_token,
            &auth_response.refresh_token,
        )
    )?;

    cookies.add(Cookie::new(COOKIE_NAME, session_token.to_string()));

    state.postgres().insert_user(user.into()).await?;

    Ok(Redirect::to("http://localhost:3000/"))
}

#[debug_handler]
pub async fn main_page(cookies: Cookies) -> String {
    let cookie = get_session_cookie(&cookies);
    if let Ok(cookie) = cookie {
        format!("This is the main page and your cookie is: {}", cookie)
    } else {
        "This is the main page and you don't have any cookie 🤨".to_string()
    }
}
