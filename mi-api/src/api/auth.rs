use axum::debug_handler;
use axum::extract::{Query, State};
use axum::response::Redirect;
use mi_db::user::UserError;
use once_cell::sync::Lazy;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use super::get_session_cookie;
use crate::api::COOKIE_NAME;
use crate::result::AppResult;
use crate::state::SharedState;

static REDIRECT_URI: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_AUTH_REDIRECT_URI")
        .expect("Environment variable MI_AUTH_REDIRECT_URI is not set.")
});

static OSU_CLIENT_ID: Lazy<String> = Lazy::new(|| {
    std::env::var("OSU_CLIENT_ID").expect("Environment variable OSU_CLIENT_ID is not set.")
});

static OSU_REDIRECT_URI: Lazy<String> = Lazy::new(|| {
    std::env::var("OSU_REDIRECT_URI").expect("Environment variable OSU_REDIRECT_URI is not set.")
});

#[derive(Debug, Deserialize)]
pub struct OsuAuthResponseParams {
    code: String,
}

pub async fn authorize_from_osu_api(
    Query(params): Query<OsuAuthResponseParams>,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> AppResult<Redirect> {
    info!("Auth request received");
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

    match state.postgres().get_user(user.id).await {
        Ok(_) => {}
        Err(err) => {
            if let UserError::UserNotFound(_) = err {
                state.postgres().insert_user(user.into()).await?;
            } else {
                return Err(err.into());
            }
        }
    }

    Ok(Redirect::to(&REDIRECT_URI))
}

#[debug_handler]
pub async fn main_page(cookies: Cookies, State(state): State<SharedState>) -> AppResult<String> {
    let cookie = get_session_cookie(&cookies);
    match cookie {
        Ok(session_token) => {
            let user_id = state.redis().get_user_id(session_token).await?;
            let user = state.postgres().get_user(user_id).await?;
            Ok(format!("This is the main page and your cookie is: {}\nYour user info: {:#?}", session_token, user))
        }
        _ => Ok("This is the main page and you don't have any cookie 🤨".to_string()),
    }
}

pub async fn login(cookies: Cookies, State(state): State<SharedState>) -> AppResult<Redirect> {
    let cookie = get_session_cookie(&cookies);

    if let Ok(session_token) = cookie {
        let user_id_res = state.redis().get_user_id(session_token).await;
        
        // User already authed and their session tokens are on the redis DB
        if let Ok(_) = user_id_res {
            return Ok(Redirect::to(&REDIRECT_URI))
        }
    }

    let redirect_uri = format!(
        "https://osu.ppy.sh/oauth/authorize?response_type=code&client_id={}&redirect_uri={}",
        *OSU_CLIENT_ID, *OSU_REDIRECT_URI
    );

    info!(redirect_uri, "Redirecting");
    Ok(Redirect::to(&redirect_uri))
}
