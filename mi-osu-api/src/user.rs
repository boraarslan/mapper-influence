#![allow(dead_code)]
use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

#[derive(Debug, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub id: i64,
    pub username: String,
    pub join_date: String,
    pub kudosu: Kudosu,
    pub playmode: String,
    pub title: Option<String>,
    pub favourite_beatmapset_count: i64,
    pub graveyard_beatmapset_count: i64,
    pub loved_beatmapset_count: i64,
    pub pending_beatmapset_count: i64,
    pub guest_beatmapset_count: i64,
    pub ranked_beatmapset_count: i64,
    pub nominated_beatmapset_count: i64,
    pub country: Country,
    pub cover: Cover,
    pub previous_usernames: Vec<String>,
    pub badges: Vec<Badge>,
}

#[derive(Debug, Deserialize)]
pub struct Badge {
    pub awarded_at: String,
    pub description: String,
    pub image_url: String,
    pub url: String,
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

#[derive(Debug, Deserialize)]
pub struct Kudosu {
    pub total: i64,
    pub available: i64,
}

pub async fn request_user_info(client: &Client, auth_token: &str) -> Result<User, ReqwestError> {
    let response_result = client
        .get("https://osu.ppy.sh/api/v2/me/")
        .header("Authorization", "Bearer ".to_string() + auth_token)
        .send()
        .await?;

    let response_body = response_result.json::<User>().await?;
    Ok(response_body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test() {
        let client = reqwest::Client::new();
        // Get Access token from auth.rs
        let token = "token";
        dbg!(request_user_info(&client, token).await.unwrap());
    }
}
