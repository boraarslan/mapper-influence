#![allow(dead_code)]

use std::fmt;

use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

#[derive(Debug, Deserialize)]
pub struct Beatmapset {
    artist: String,
    artist_unicode: String,
    creator: String,
    favourite_count: i64,
    id: i64,
    nsfw: bool,
    offset: i64,
    play_count: i64,
    preview_url: String,
    source: String,
    spotlight: bool,
    status: String,
    title: String,
    title_unicode: String,
    track_id: Option<i64>,
    user_id: i64,
    video: bool,
    bpm: i64,
    can_be_hyped: bool,
    discussion_enabled: bool,
    discussion_locked: bool,
    is_scoreable: bool,
    last_updated: String,
    legacy_thread_url: String,
    nominations_summary: NominationsSummary,
    ranked: i64,
    ranked_date: Option<String>,
    storyboard: bool,
    submitted_date: String,
    tags: String,
    beatmaps: Vec<Beatmap>,
}

#[derive(Debug, Deserialize)]
pub struct Beatmap {
    beatmapset_id: i64,
    difficulty_rating: f64,
    id: i64,
    mode: String,
    status: String,
    total_length: i64,
    user_id: i64,
    version: String,
    accuracy: f64,
    ar: f64,
    bpm: i64,
    convert: bool,
    count_circles: i64,
    count_sliders: i64,
    count_spinners: i64,
    cs: f64,
    drain: f64,
    hit_length: i64,
    is_scoreable: bool,
    last_updated: String,
    mode_int: i64,
    passcount: i64,
    playcount: i64,
    ranked: i64,
    url: String,
    checksum: String,
}

#[derive(Debug, Deserialize)]
pub struct NominationsSummary {
    current: i64,
    required: i64,
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
    // Url construction could be optimized with dedicated crates
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

// the same can also be done with lookup endpoint but I feel like this endpoint is more appropriate
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
        dbg!(request_beatmap(&client, &token, 1304701).await.unwrap());
        dbg!(
            request_user_beatmapsets(&client, token, 3953470, BeatmapType::Graveyard)
                .await
                .unwrap()
        );
    }
}
