use axum::extract::FromRef;
use mi_core::future_log_ext::FutureLogExt;
use mi_osu_api::auth::{access_token, refresh_token, AuthResponseBody};
use mi_osu_api::{
    request_token_user, request_user, request_user_beatmapsets, BeatmapType, Beatmapset,
    OsuApiError, User,
};
use tracing::instrument;

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

    #[instrument(skip(self, osu_refresh_token), fields(elapsed), err)]
    pub async fn get_osu_refresh_token(
        &self,
        osu_refresh_token: String,
    ) -> Result<AuthResponseBody, OsuApiError> {
        refresh_token(&self.client, osu_refresh_token)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self, code), fields(elapsed), err)]
    pub async fn get_osu_access_token(
        &self,
        code: String,
    ) -> Result<AuthResponseBody, OsuApiError> {
        access_token(&self.client, code).log_elapsed().await
    }

    #[instrument(skip(self, auth_token), fields(elapsed), err)]
    pub async fn request_osu_token_user(&self, auth_token: &str) -> Result<User, OsuApiError> {
        request_token_user(&self.client, auth_token)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self, auth_token), fields(elapsed), err)]
    pub async fn request_osu_user(
        &self,
        auth_token: &str,
        user_id: i64,
    ) -> Result<User, OsuApiError> {
        request_user(&self.client, auth_token, user_id)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self, auth_token), fields(elapsed), err)]
    pub async fn get_all_user_mapsets(
        &self,
        user_id: i64,
        auth_token: &str,
    ) -> Result<Vec<Beatmapset>, OsuApiError> {
        let func = async move {
            let results = tokio::try_join!(
                request_user_beatmapsets(&self.client, auth_token, user_id, BeatmapType::Ranked),
                request_user_beatmapsets(&self.client, auth_token, user_id, BeatmapType::Loved),
                request_user_beatmapsets(&self.client, auth_token, user_id, BeatmapType::Pending),
                request_user_beatmapsets(&self.client, auth_token, user_id, BeatmapType::Graveyard),
            );

            match results {
                Ok((ranked, loved, pending, graveyard)) => {
                    let mut beatmapsets = Vec::new();
                    beatmapsets.extend(ranked);
                    beatmapsets.extend(loved);
                    beatmapsets.extend(pending);
                    beatmapsets.extend(graveyard);
                    Ok(beatmapsets)
                }
                Err(e) => Err(e),
            }
        };

        func.log_elapsed().await
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
