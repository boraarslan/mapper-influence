#![allow(dead_code)]
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

static MI_CLIENT_ID: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_CLIENT_ID").expect("Environment variable MI_CLIENT_ID is not set.")
});
static MI_CLIENT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_CLIENT_SECRET").expect("Environment variable MI_CLIENT_SECRET is not set.")
});
static MI_REDIRECT_URI: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_REDIRECT_URI").expect("Environment variable MI_REDIRECT_URI is not set.")
});

// Only implementing the essential and likely to be used fields first
#[derive(Debug, Deserialize)]
pub struct User {
    avatar_url: String,
    id: i64,
    username: String,
    join_date: String,
    kudosu: Kudosu,
    playmode: String,
    title: Option<String>,
    favourite_beatmapset_count: i64,
    graveyard_beatmapset_count: i64,
    loved_beatmapset_count: i64,
    pending_beatmapset_count: i64,
    guest_beatmapset_count: i64,
    ranked_beatmapset_count: i64,
    nominated_beatmapset_count: i64,
    country: Country,
    cover: Cover,
    // not documented but assuming it's string
    previous_usernames: Vec<String>,
    badges: Vec<Badge>,
}

#[derive(Debug, Deserialize)]
pub struct Badge {
    awarded_at: String,
    description: String,
    image_url: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    code: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Cover {
    custom_url: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct Kudosu {
    total: i64,
    available: i64,
}

pub async fn request_token_user(client: &Client, auth_token: &str) -> Result<User, ReqwestError> {
    let response_result = client
        .get("https://osu.ppy.sh/api/v2/me/")
        .header("Authorization", "Bearer ".to_string() + auth_token)
        .send()
        .await?;

    let response_body = response_result.json::<User>().await?;
    Ok(response_body)
}

// Adding test for example
#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test() {
        let client = reqwest::Client::new();
        // Get Access token from auth.rs
        let token = "token";
        dbg!(request_token_user(&client, token).await.unwrap());
    }
}
