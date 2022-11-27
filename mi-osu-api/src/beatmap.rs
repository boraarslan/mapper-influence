#![allow(dead_code)]

use std::fmt;

use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

#[derive(Debug, Deserialize)]
pub struct Beatmapset {
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub favourite_count: i64,
    pub id: i64,
    pub nsfw: bool,
    pub offset: i64,
    pub play_count: i64,
    pub preview_url: String,
    pub source: String,
    pub spotlight: bool,
    pub status: String,
    pub title: String,
    pub title_unicode: String,
    pub track_id: Option<i64>,
    pub user_id: i64,
    pub video: bool,
    pub bpm: i64,
    pub can_be_hyped: bool,
    pub discussion_enabled: bool,
    pub discussion_locked: bool,
    pub is_scoreable: bool,
    pub last_updated: String,
    pub legacy_thread_url: String,
    pub nominations_summary: NominationsSummary,
    pub ranked: i64,
    pub ranked_date: Option<String>,
    pub storyboard: bool,
    pub submitted_date: String,
    pub tags: String,
    pub beatmaps: Vec<Beatmap>,
}

#[derive(Debug, Deserialize)]
pub struct Beatmap {
    pub beatmapset_id: i64,
    pub difficulty_rating: f64,
    pub id: i64,
    pub mode: String,
    pub status: String,
    pub total_length: i64,
    pub user_id: i64,
    pub version: String,
    pub accuracy: f64,
    pub ar: f64,
    pub bpm: i64,
    pub convert: bool,
    pub count_circles: i64,
    pub count_sliders: i64,
    pub count_spinners: i64,
    pub cs: f64,
    pub drain: f64,
    pub hit_length: i64,
    pub is_scoreable: bool,
    pub last_updated: String,
    pub mode_int: i64,
    pub passcount: i64,
    pub playcount: i64,
    pub ranked: i64,
    pub url: String,
    pub checksum: String,
}

#[derive(Debug, Deserialize)]
pub struct NominationsSummary {
    pub current: i64,
    pub required: i64,
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

    let response_result = client
        .get(url)
        .header("Authorization", "Bearer ".to_string() + auth_token)
        .send()
        .await?;

    let response_body = response_result.json::<Vec<Beatmapset>>().await?;
    Ok(response_body)
}

pub async fn request_beatmap(
    client: &Client,
    auth_token: &str,
    beatmap_id: i64,
) -> Result<Beatmap, ReqwestError> {
    let url = "https://osu.ppy.sh/api/v2/beatmaps/".to_string() + &beatmap_id.to_string();

    let response_result = client
        .get(url)
        .header("Authorization", "Bearer ".to_string() + auth_token)
        .send()
        .await?;

    let response_body = response_result.json::<Beatmap>().await?;
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
        dbg!(request_beatmap(&client, token, 1304701).await.unwrap());
        dbg!(
            request_user_beatmapsets(&client, token, 3953470, BeatmapType::Graveyard)
                .await
                .unwrap()
        );
    }
}
