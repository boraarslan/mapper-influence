#![allow(dead_code)]
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::ReqwestError;

const MI_CLIENT_ID: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_CLIENT_ID").expect("Environment variable MI_CLIENT_ID is not set.")
});
const MI_CLIENT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_CLIENT_SECRET").expect("Environment variable MI_CLIENT_SECRET is not set.")
});
const MI_REDIRECT_URI: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_REDIRECT_URI").expect("Environment variable MI_REDIRECT_URI is not set.")
});

#[derive(Serialize, Debug)]
struct AuthRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub redirect_uri: String,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
}

impl AuthRequest {
    pub fn access(code: String) -> AuthRequest {
        AuthRequest {
            client_id: MI_CLIENT_ID.clone(),
            client_secret: MI_CLIENT_SECRET.clone(),
            redirect_uri: MI_REDIRECT_URI.clone(),
            grant_type: "authorization_code".to_string(),
            code: Some(code),
            refresh_token: None,
        }
    }

    pub fn refresh(refresh_token: String) -> AuthRequest {
        AuthRequest {
            client_id: MI_CLIENT_ID.clone(),
            client_secret: MI_CLIENT_SECRET.clone(),
            redirect_uri: MI_REDIRECT_URI.clone(),
            grant_type: "refresh_token".to_string(),
            code: None,
            refresh_token: Some(refresh_token),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AuthResponseBody {
    pub token_type: String,
    pub expires_in: u32,
    pub access_token: String,
    pub refresh_token: String,
}

async fn request_token(
    client: &Client,
    body: AuthRequest,
) -> Result<AuthResponseBody, ReqwestError> {
    let response_result = client
        .post("https://osu.ppy.sh/oauth/token")
        .json(&body)
        .send()
        .await?;
    let response_body = response_result.json::<AuthResponseBody>().await?;
    Ok(response_body)
}

pub async fn refresh_token(
    client: &Client,
    refresh_token: String,
) -> Result<AuthResponseBody, ReqwestError> {
    let refresh_request = AuthRequest::refresh(refresh_token);
    request_token(client, refresh_request).await
}

pub async fn access_token(client: &Client, code: String) -> Result<AuthResponseBody, ReqwestError> {
    let access_request = AuthRequest::access(code);
    request_token(client, access_request).await
}
