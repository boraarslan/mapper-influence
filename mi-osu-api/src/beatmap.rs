//! osu! Beatmap API implementation.
//!
//! Used to request [beatmap](https://osu.ppy.sh/wiki/en/Beatmap) data.
//! For more information about beatmap endpoints, visit
//! official osu! API documentation for [beatmaps](https://osu.ppy.sh/docs/index.html#beatmaps)
//! and [user](https://osu.ppy.sh/docs/index.html#get-user-beatmaps) endpoints.
//!
//! These endpoints require [access tokens](crate::auth).
//!
//! An important point is that a `beatmapset` and a `beatmap` are different concepts despite what
//! is stated in [official osu wiki](https://osu.ppy.sh/wiki/en/Beatmap).
//! This is especially true if one works with maps in a technical way like we do here.
//! It's because the API docs and wiki are conflicting. To avoid confusion,
//! we are following API definitions in our implementation.
//!
//! * Beatmaps in essence are the core of a beatmap data.
//! They are singular ["difficulties"](https://osu.ppy.sh/wiki/en/Beatmap/Difficulty) of a beatmapset.
//! They are the .osu files that contain actual beatmap data like object positions.
//!
//! * Beatmapsets are other hand, just a container for the beatmaps.
//! They contain shared information like name of the song, artist etc.
//!
//! In our implementation, [`Beatmapset`] contains shared information and individual
//! [`Beatmaps`](Beatmap) in a vector.

#![allow(dead_code)]
use std::fmt;

use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

/// Contains information about a [beatmapset](https://osu.ppy.sh/wiki/en/Beatmap).
///
/// Only the relevant fields are implemented in this crate.
/// To get information on all of the fields, refer to
/// [the official osu! API](https://osu.ppy.sh/docs/index.html#beatmapset).
///
/// Please refer to [module docs](crate::beatmap) for more information.
#[derive(Debug, Deserialize)]
pub struct Beatmapset {
    /// Unique ID of a beatmapset. Different from [beatmap ID](Beatmap::id).
    pub id: i64,
    /// Status of the beatmapset. Ranked, Qualified etc.
    pub status: String,
    /// Name of the mapper of this beatmapset.
    pub creator: String,
    /// Vector of beatmaps.
    pub beatmaps: Vec<Beatmap>,
    /// Beatmapset name data. Seperated from [Beatmapset] struct to make access easier.
    #[serde(flatten)]
    pub names: BeatmapsetNames,
}

/// Beatmapset name data. Seperated from [Beatmapset] struct to make access easier.
///
/// Unicode fields are for the names with non ascii characters. Mostly japanese characters.
#[derive(Debug, Deserialize)]
pub struct BeatmapsetNames {
    pub artist: String,
    pub artist_unicode: String,
    pub title: String,
    pub title_unicode: String,
}

/// Contains information about a [beatmap](https://osu.ppy.sh/wiki/en/Beatmap/Difficulty).
///
/// Only the relevant fields are implemented in this crate.
/// To get information on all of the fields, refer to
/// [the official osu! API](https://osu.ppy.sh/docs/index.html#beatmap).
///
/// Please refer to [module docs](crate::beatmap) for more information.
#[derive(Debug, Deserialize)]
pub struct Beatmap {
    /// [Star rating](https://osu.ppy.sh/wiki/en/Beatmap/Star_rating)
    pub difficulty_rating: f64,
    /// Unique ID of a beatmap. Different from [beatmapset ID](Beatmapset::id).
    pub id: i64,
    /// Url of the beatmap. Kept for the convenience of the front-end developers.
    pub url: String,
    /// Difficulty name. This field is actually named as "version" in the osu! API but it's not
    /// familiar for osu! people.
    #[serde(rename = "version")]
    pub diff_name: String,
}

/// Beatmap type to be given as parameter in [`request_user_beatmapsets`]
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

/// A request used to get a vector of [`Beatmapset`] which are related to a user.
///
/// * `client` - A [reqwest client](`reqwest::Client`).
/// * `auth_token` - Authorization token that has been acquired in [authorization
///   module](crate::auth).
/// * `user` - ID of the user which the beatmaps are related to.
/// * `beatmap_type` - Type of the map being requested. The types of beatmaps are defined in
///   [`BeatmapType`].
///
/// Because of the way this endpoint is implemented in osu! API, there is no way to get all types of
/// maps at once. The only way is to send seperate requests for [each type of
/// beatmaps](BeatmapType).
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

/// A request used to ge a singular [`Beatmap`] data.
///
/// * `client` - A [reqwest client](`reqwest::Client`).
/// * `auth_token` - Authorization token that has been acquired in [authorization
///   module](crate::auth).
/// * `user` - ID of the beatmap
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
