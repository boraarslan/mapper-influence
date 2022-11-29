#![allow(dead_code)]

use std::fmt;

use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

#[derive(Debug, Deserialize)]
pub struct Beatmapset {
    pub id: i64,
    pub status: String,
    pub creator: String,
    pub beatmaps: Vec<Beatmap>,
    #[serde(flatten)]
    pub names: BeatmapsetNames,
}

#[derive(Debug, Deserialize)]
pub struct BeatmapsetNames {
    pub artist: String,
    pub artist_unicode: String,
    pub title: String,
    pub title_unicode: String,
}

#[derive(Debug, Deserialize)]
pub struct Beatmap {
    pub difficulty_rating: f64,
    pub id: i64,
    pub url: String,
    #[serde(rename = "version")]
    pub diff_name: String,
}

pub enum BeatmapType {
    Graveyard,
    Loved,
    Pending,
    Ranked,
}

impl fmt::Display for BeatmapType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BeatmapType::Graveyard => write!(f, "graveyard"),
            BeatmapType::Loved => write!(f, "loved"),
            BeatmapType::Pending => write!(f, "pending"),
            BeatmapType::Ranked => write!(f, "ranked"),
        }
    }
}

pub async fn request_user_beatmapsets(
    client: &Client,
    auth_token: &str,
    user: i64,
    beatmap_type: BeatmapType,
) -> Result<Vec<Beatmapset>, ReqwestError> {
    let url = format!(
        "https://osu.ppy.sh/api/v2/users/{}/beatmapsets/{}",
        user, beatmap_type
    );
    let response_result = client.get(url).bearer_auth(auth_token).send().await?;
    let response_body: Vec<Beatmapset> = response_result.json().await?;
    Ok(response_body)
}

pub async fn request_beatmap(
    client: &Client,
    auth_token: &str,
    beatmap_id: i64,
) -> Result<Beatmap, ReqwestError> {
    let url = format!("https://osu.ppy.sh/api/v2/beatmaps/{}", beatmap_id);
    let response_result = client.get(url).bearer_auth(auth_token).send().await?;
    let response_body: Beatmap = response_result.json().await?;
    Ok(response_body)
}
