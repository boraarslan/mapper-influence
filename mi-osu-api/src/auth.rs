//! osu! Authentication API implementation.
//!
//! It is used to get an authentication token and refresh that said token. For more information,
//! visit [official osu! API Documentation]
//!
//! To get an authentication token, the application must be [registered in osu website]
//!
//! Afterwards, the authorization code can be acquired using [authorization code grant]
//!
//! The authorization code can be used to get an authentication token to be used in other API
//! endpoints.
//!
//! [official osu! API Documentation]: <https://osu.ppy.sh/docs/index.html#authentication>
//! [registered in osu website]: <https://osu.ppy.sh/home/account/edit#new-oauth-application>
//! [authorization code grant]: <https://osu.ppy.sh/docs/index.html#authorization-code-grant>

#![allow(dead_code)]
use jwt::{Header, Token};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{OsuApiError, ResponseWithBody};

static OSU_CLIENT_ID: Lazy<String> = Lazy::new(|| {
    std::env::var("OSU_CLIENT_ID").expect("Environment variable OSU_CLIENT_ID is not set.")
});
static OSU_CLIENT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("OSU_CLIENT_SECRET").expect("Environment variable OSU_CLIENT_SECRET is not set.")
});
static OSU_REDIRECT_URI: Lazy<String> = Lazy::new(|| {
    std::env::var("OSU_REDIRECT_URI").expect("Environment variable OSU_REDIRECT_URI is not set.")
});

#[derive(Deserialize, Serialize, Debug)]
struct Scopes {
    scopes: Vec<String>,
}

#[derive(Serialize, Debug)]
struct AuthRequest {
    pub client_id: &'static str,
    pub client_secret: &'static str,
    pub grant_type: &'static str,
    pub redirect_uri: &'static str,
    /// Without the "public" scope, authorization tokens can't be used to request public
    /// information. Check the official osu! API [scopes](https://osu.ppy.sh/docs/index.html#scopes) section
    pub scope: &'static str,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
}

impl AuthRequest {
    fn access(code: String) -> AuthRequest {
        AuthRequest {
            client_id: &OSU_CLIENT_ID,
            client_secret: &OSU_CLIENT_SECRET,
            redirect_uri: &OSU_REDIRECT_URI,
            grant_type: "authorization_code",
            scope: "public, identify",
            code: Some(code),
            refresh_token: None,
        }
    }

    fn refresh(refresh_token: String) -> AuthRequest {
        AuthRequest {
            client_id: &OSU_CLIENT_ID,
            client_secret: &OSU_CLIENT_SECRET,
            redirect_uri: &OSU_REDIRECT_URI,
            grant_type: "refresh_token",
            scope: "public, identify",
            code: None,
            refresh_token: Some(refresh_token),
        }
    }
}

/// Auth response body. Returned after authentication requests such as [`access_token`] and
/// [`refresh_token`].
#[derive(Deserialize, Debug)]
pub struct AuthResponseBody {
    /// Bearer token
    pub token_type: String,
    /// Token validity duration in seconds
    pub expires_in: u32,
    /// An access token to authorize requests on endpoints
    pub access_token: String,
    /// Refresh token. Used to get a new access token without using authorization code grant
    pub refresh_token: String,
}

fn check_scope(access_token: &str) -> Result<(), OsuApiError> {
    let parsed_token: Token<Header, Scopes, _> = Token::parse_unverified(access_token)?;
    if !parsed_token.claims().scopes.contains(&"public".to_string()) {
        return Err(OsuApiError::PublicScopeError);
    }
    Ok(())
}

async fn request_token(
    client: &Client,
    body: AuthRequest,
) -> Result<AuthResponseBody, OsuApiError> {
    let response_result = client
        .post("https://osu.ppy.sh/oauth/token")
        .form(&body)
        .send()
        .await?;

    response_result.try_deser_api_response().await
}

/// Authorization code refresh method. Returns an [`AuthResponseBody`] with fresh codes to be used.
///
/// After using the refresh token, a new refresh token is generated so the old one can not be used
/// again.
pub async fn refresh_token(
    client: &Client,
    refresh_token: String,
) -> Result<AuthResponseBody, OsuApiError> {
    let refresh_request = AuthRequest::refresh(refresh_token);
    request_token(client, refresh_request).await
}

/// Authorization code request method. Returns an [`AuthResponseBody`] with necessary information to
/// update the code later and authorize other endpoints.
///
/// For more information, check the [authorization code grant] section on osu! API documentation.
///
/// [authorization code grant]: <https://osu.ppy.sh/docs/index.html#authorization-code-grant>
pub async fn access_token(client: &Client, code: String) -> Result<AuthResponseBody, OsuApiError> {
    let access_request = AuthRequest::access(code);
    let requested_token = request_token(client, access_request).await?;
    check_scope(&requested_token.access_token)?;
    Ok(requested_token)
}
