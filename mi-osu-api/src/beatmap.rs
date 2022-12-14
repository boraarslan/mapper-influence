//! osu! Beatmap API implementation.
//!
//! It is used to request [beatmap] data.
//! For more information about beatmap endpoints, visit
//! official osu! API documentation for [beatmaps]
//! and [user] endpoints.
//!
//! These endpoints require [access tokens](crate::auth).
//!
//! The important point is that a `beatmapset` and a `beatmap` are different concepts despite what
//! is stated in [official osu wiki][beatmap].
//! It is especially important if one works with maps on a deep level like we do.
//! It's because the API docs and wiki are conflicting. To avoid confusion,
//! we are following API definitions in our implementation.
//!
//! * Beatmaps in essence are the core of a beatmap data.
//! Beatmaps are individual ["difficulties"] of a beatmapset.
//! Beatmaps are the .osu files that contain actual beatmap data, like object positions.
//!
//! * Beatmapsets are just a container for the beatmaps.
//! They contain shared information like name of the song, artist etc.
//!
//! In our implementation, [`Beatmapset`] contains shared information and individual
//! [`Beatmaps`][Beatmap] in a vector.
//!
//! [beatmap]: <https://osu.ppy.sh/wiki/en/Beatmap>
//! [beatmaps]: <https://osu.ppy.sh/docs/index.html#beatmaps>
//! [user]: <https://osu.ppy.sh/docs/index.html#get-user-beatmaps>
//! ["difficulties"]: <https://osu.ppy.sh/wiki/en/Beatmap/Difficulty>

#![allow(dead_code)]
use std::fmt;

use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

/// Information about a [beatmapset].
///
/// Only the relevant fields are implemented in this crate.
/// For more information about all of the fields, refer to
/// [the official osu! API].
///
/// Refer to the [module docs](crate::beatmap) for more information.
///
/// [beatmapset]: <https://osu.ppy.sh/wiki/en/Beatmap>
/// [the official osu! API]: <Https://osu.ppy.sh/docs/index.html#beatmapset>
#[derive(Debug, Deserialize)]
pub struct Beatmapset {
    /// Unique ID of a beatmapset. Different from [beatmap ID](Beatmap::id)
    pub id: i64,
    /// Status of the beatmapset. Ranked, Qualified etc
    pub status: BeatmapType,
    /// Name of the mapper of this beatmapset. The name of the mapper stays the same in beatmapset
    /// information even if the mapper changed their names.
    pub creator: String,
    /// Listof beatmaps
    pub beatmaps: Vec<Beatmap>,
    /// Beatmapset name data. Seperated from [Beatmapset] struct to make access easier
    #[serde(flatten)]
    pub names: BeatmapsetNames,
}

/// Beatmapset name data. Seperated from [Beatmapset] struct to make access easier.
///
/// Unicode fields are for the names with non-ASCII characters. It consists mostly of Japanese
/// characters.
#[derive(Debug, Deserialize)]
pub struct BeatmapsetNames {
    pub artist: String,
    pub artist_unicode: String,
    pub title: String,
    pub title_unicode: String,
}

/// Information about a [beatmap].
///
/// Only the relevant fields are implemented in this crate.
/// For moıre information about all of the fields, refer to
/// [the official osu! API].
///
/// Refer to the [module docs](crate::beatmap) for more information.
///
/// [beatmap]: <https://osu.ppy.sh/wiki/en/Beatmap/Difficulty>
/// [the official osu! API]: <Https://osu.ppy.sh/docs/index.html#beatmapset>
#[derive(Debug, Deserialize)]
pub struct Beatmap {
    /// [Star rating](https://osu.ppy.sh/wiki/en/Beatmap/Star_rating) of the beatmap
    pub difficulty_rating: f64,
    /// Unique ID of the beatmap. Different from [beatmapset ID](Beatmapset::id)
    pub id: i64,
    /// Url of the beatmap
    pub url: String,
    /// Difficulty name
    #[serde(rename = "version")]
    pub name: String,
}

/// Type of a beatmap.
///
/// These are the variants of map types that are in users profile.
#[derive(Debug, Deserialize)]
pub enum BeatmapType {
    Graveyard,
    Loved,
    /// Includes Pending and WIP maps.
    Pending,
    Ranked,
    Guest,
    Nominated,
}

impl fmt::Display for BeatmapType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BeatmapType::Graveyard => write!(f, "graveyard"),
            BeatmapType::Loved => write!(f, "loved"),
            BeatmapType::Pending => write!(f, "pending"),
            BeatmapType::Ranked => write!(f, "ranked"),
            BeatmapType::Guest => write!(f, "guest"),
            BeatmapType::Nominated => write!(f, "nominated"),
        }
    }
}

/// A request to get a list of [`Beatmapset`] related to a user.
///
/// Since osu! does not expose an API to retrieve all of the maps for a given user,
/// only way to fetch all maps is to send multiple requests for [each type of beatmap](BeatmapType).
///
/// Available variants for this method are Graveyard, Loved, Pending and Ranked.
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

/// A request to get individual [`Beatmap`] data.
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
