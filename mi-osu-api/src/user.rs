#![allow(dead_code)]
use std::num::ParseIntError;

use reqwest::Client;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

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
    pub groups: Vec<UserGroup>,
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

#[derive(Debug, Deserialize)]
pub struct UserGroup {
    pub is_probationary: bool,
    pub name: String,
    pub short_name: String,
    pub playmodes: Vec<String>,
    #[serde(deserialize_with = "hex")]
    pub colour: Colour,
}

#[derive(Debug)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub fn from_hex(hex: &str) -> Result<Colour, ParseIntError> {
        let r = u8::from_str_radix(&hex[1..3], 16)?;
        let g = u8::from_str_radix(&hex[3..5], 16)?;
        let b = u8::from_str_radix(&hex[5..7], 16)?;
        let colour = Colour { r: r, g: g, b: b };
        Ok(colour)
    }

    pub fn get_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

fn hex<'de, D>(deserializer: D) -> Result<Colour, D::Error>
where
    D: Deserializer<'de>,
{
    let hex: &str = Deserialize::deserialize(deserializer)?;
    Colour::from_hex(hex).map_err(D::Error::custom)
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
