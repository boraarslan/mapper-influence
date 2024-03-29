use axum::debug_handler;
use axum::extract::{Query, State};
use axum::response::Redirect;
use mi_db::user::UserError;
use mi_osu_api::OsuApiError;
use once_cell::sync::Lazy;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use tracing::info;

use crate::result::{AppError, AppResult};
use crate::state::SharedState;
use crate::{get_session_cookie, SessionError, COOKIE_NAME};

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
    code: Option<String>,
    error: Option<String>,
}

pub async fn authorize_from_osu_api(
    Query(params): Query<OsuAuthResponseParams>,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Result<Redirect, AppError> {
    if let Some(err) = params.error {
        // TODO: Better error handling
        return Err(SessionError::OsuAuthError(err).into());
    }

    let Some(code) = params.code else {
        return Err(SessionError::OsuAuthError("No code provided".to_string()).into());
    };

    let auth_response = state.http().get_osu_access_token(code).await;
    let auth_response = match auth_response {
        Ok(response) => response,
        Err(err) => match err {
            OsuApiError::PublicScopeError => {
                let bad_scope_redirect_uri = format!("{}/login/failed", *REDIRECT_URI);
                return Ok(Redirect::to(&bad_scope_redirect_uri));
            }
            err => return Err(err.into()),
        },
    };

    let user = state
        .http()
        .request_osu_token_user(&auth_response.access_token)
        .await?;

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

    let redirect_uri = (*REDIRECT_URI).to_string();

    Ok(Redirect::to(&redirect_uri))
}

#[debug_handler]
pub async fn cookie_page(cookies: Cookies, State(state): State<SharedState>) -> AppResult<String> {
    let cookie = get_session_cookie(&cookies);
    match cookie {
        Ok(session_token) => {
            let user_id = state.redis().get_user_id(session_token).await?;
            let user = state.postgres().get_user(user_id).await?;
            Ok(format!(
                "This is the cookie page and your cookie is: {}\nYour user info: {:#?}",
                session_token, user
            ))
        }
        _ => Ok("This is the cookie page and you don't have any cookie 🤨".to_string()),
    }
}

pub async fn login(cookies: Cookies, State(state): State<SharedState>) -> AppResult<Redirect> {
    let cookie = get_session_cookie(&cookies);

    if let Ok(session_token) = cookie {
        let user_id_res = state.redis().get_user_id(session_token).await;

        // User already authed and their session tokens are on the redis DB
        if user_id_res.is_ok() {
            return Ok(Redirect::to(&REDIRECT_URI));
        }
    }

    let redirect_uri = format!(
        "https://osu.ppy.sh/oauth/authorize?response_type=code&client_id={}&redirect_uri={}&scope=public+identify",
        *OSU_CLIENT_ID, *OSU_REDIRECT_URI
    );

    info!(redirect_uri, "Redirecting");
    Ok(Redirect::to(&redirect_uri))
}
