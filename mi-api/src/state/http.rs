use axum::extract::FromRef;
use mi_osu_api::auth::{access_token, refresh_token, AuthResponseBody};
use mi_osu_api::{request_token_user, request_user, ReqwestError, User};

use super::SharedState;

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }

    pub async fn get_osu_refresh_token(
        &self,
        osu_refresh_token: String,
    ) -> Result<AuthResponseBody, ReqwestError> {
        refresh_token(&self.client, osu_refresh_token).await
    }

    pub async fn get_osu_access_token(
        &self,
        code: String,
    ) -> Result<AuthResponseBody, ReqwestError> {
        access_token(&self.client, code).await
    }

    pub async fn request_osu_token_user(&self, auth_token: &str) -> Result<User, ReqwestError> {
        request_token_user(&self.client, auth_token).await
    }

    pub async fn request_osu_user(
        &self,
        auth_token: &str,
        user_id: i64,
    ) -> Result<User, ReqwestError> {
        request_user(&self.client, auth_token, user_id).await
    }
}

impl FromRef<SharedState> for HttpClient {
    fn from_ref(state: &SharedState) -> Self {
        state.http_client.clone()
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient::new()
    }
}
