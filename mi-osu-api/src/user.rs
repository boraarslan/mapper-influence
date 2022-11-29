#![allow(dead_code)]
use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

#[derive(Debug, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub id: i64,
    pub playmode: String,
    pub title: Option<String>,
    pub username: String,
    pub country: Country,
    pub cover: Cover,
    #[serde(rename = "mapping_follower_count")]
    pub followers: i64,
    #[serde(flatten)]
    pub stats: BeatmapsetStats,
}

#[derive(Debug, Deserialize)]
pub struct BeatmapsetStats {
    #[serde(rename = "ranked_beatmapset_count")]
    pub ranked: i64,
    #[serde(rename = "loved_beatmapset_count")]
    pub loved: i64,
    #[serde(rename = "nominated_beatmapset_count")]
    pub nominated: i64,
    #[serde(rename = "pending_beatmapset_count")]
    pub pending: i64,
    #[serde(rename = "graveyard_beatmapset_count")]
    pub graveyard: i64,
    #[serde(rename = "guest_beatmapset_count")]
    pub guest: i64,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Cover {
    pub custom_url: String,
    pub url: String,
}

pub async fn request_token_user(client: &Client, auth_token: &str) -> Result<User, ReqwestError> {
    let response_result = client
        .get("https://osu.ppy.sh/api/v2/me/")
        .bearer_auth(auth_token)
        .send()
        .await?;

    let response_body: User = response_result.json().await?;
    Ok(response_body)
}

pub async fn request_user(
    client: &Client,
    auth_token: &str,
    user_id: i64,
) -> Result<User, ReqwestError> {
    let url = format!("https://osu.ppy.sh/api/v2/users/{}", user_id);
    let response_result = client.get(url).bearer_auth(auth_token).send().await?;
    let response_body: User = response_result.json().await?;
    Ok(response_body)
}
