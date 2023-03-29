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

async fn request_token(
    client: &Client,
    body: AuthRequest,
) -> Result<AuthResponseBody, OsuApiError> {
    let response_result = client
        .post("https://osu.ppy.sh/oauth/token")
        .form(&body)
        .send()
        .await?;
    let response_result: Result<AuthResponseBody, OsuApiError> =
        response_result.try_deserialising().await;
    response_result
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
    request_token(client, access_request).await
}

#[tokio::test]
async fn test() {
    dotenvy::dotenv().unwrap();
    let code = "def50200d88187735e783a74c0670915bac8ed6430161fff4dd73601dcdc0bc601ef36d7ec98153e6a1e98e41a231a83b46c61d37477356eb35761702274aabb21e21c0104e8a2b3b14658ed41b971871caec7971bc49bd3d362d58238c1cbb7ccdd888e03925baa56bb0642365efa3d2499990d6e2ee0782849ba8d846c74f69112e5d8a7b50ba50e4c4f88d4f233bdfdc18d4fea01e067d37b1679b2311a8a7513ea4559b3ff9367ac860613724b6411ed4646e0af2d15daeda32830ea3a750786c0a1d3207efcbe227d84c72b40f93ffa66ef54948fda3a85d9405b59722db6dea17c72344ca759150375e51650999fb047e495c721d34f04d3432c9cbb1dbb6a469b43c171b3d211b1884ec8f7a80d9362c7b69be04768592428737f55da5d2bbd74151c9283cb82d2ab0a2a33ea1048f978bd3cfb90b95ed2450614d6c126cf6f1d25fd1f69c472bfc60759306ef27dfd8551e803c90296782739bf01443a396af7481a1e59c1dd50b58b7fcc659d6035705d89ad27a4f97f256a59c8e4f06bc9a44f04a3";
    let client = reqwest::Client::new();
    let result = access_token(&client, code.to_string()).await;

    dbg!(result.unwrap());
}
