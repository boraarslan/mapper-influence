//! osu! user API implementation.
//!
//! Used to request [user](https://osu.ppy.sh/docs/index.html#user) data.
//! For more information about beatmap endpoints, visit
//! official osu! API documentation for
//! [user](https://osu.ppy.sh/docs/index.html#get-user-beatmaps) endpoints.
//!
//! These endpoints require [access tokens](crate::auth).

#![allow(dead_code)]
use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

/// Contains information about a user.
///
/// Only the relevant fields are implemented in this crate.
/// To get information on all of the fields, refer to
/// [the official osu! API](https://osu.ppy.sh/docs/index.html#user).
#[derive(Debug, Deserialize)]
pub struct User {
    /// The square image in the user profile.
    pub avatar_url: String,
    /// Unique ID of the user. These numbers represent the order of account creation.
    pub id: i64,
    /// Main playmode of the user.
    pub playmode: String,
    /// Title of the user. Titles are rare profile text that is awarded after user does
    /// something significant in the community.
    pub title: Option<String>,
    pub username: String,
    pub country: Country,
    pub cover: Cover,
    pub groups: Vec<UserGroup>,
    /// Number of users that are subscribed to this users beatmap updates.
    #[serde(rename = "mapping_follower_count")]
    pub followers: i64,
    /// Amounts of maps this user is related for each categories.
    #[serde(flatten)]
    pub stats: BeatmapsetStats,
}

/// Beatmap stats for the user.
///
/// This information is originally a part of [user] data. But they are seperated in our
/// implementation for convenience.
///
/// Original field names are different in API and they are shorter in our implementation.
/// Check [the official osu! API](https://osu.ppy.sh/docs/index.html#user) for more information.
///
/// This struct is contained in [`User`] struct.
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

/// Country information of a user.
///
/// This struct is contained in [`User`] struct.
#[derive(Debug, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
}

/// User profile cover image information.
///
/// Profile covers are the big rectengular images on top of a player profile.
///
/// This struct is contained in [`User`] struct.
#[derive(Debug, Deserialize)]
pub struct Cover {
    pub custom_url: String,
    pub url: String,
}

/// Contains information about the group the user might be part of.
///
/// Groups are various official organizations in the osu! community.
/// They are visible as small icons that are in user profiles.
/// This struct contains all the essential data to reconstruct these icons.
///
/// This struct is contained in [`User`] struct inside a Vector as the user can have multiple
/// groups.
///
/// Only the relevant fields are implemented in this crate.
/// To get information on all of the fields, refer to
/// [the official osu! API](https://osu.ppy.sh/docs/index.html#usergroup).
#[derive(Debug, Deserialize)]
pub struct UserGroup {
    /// Probationary users doesn't display small icons in their profile.
    pub is_probationary: bool,
    /// Name of the group.
    pub name: String,
    /// Shortened name of the group. This is what is visible in the user profiles.
    pub short_name: String,
    /// Background colour of the group icon.
    pub colour: String,
    /// Playmode icons that are shown in group icon.
    pub playmodes: Vec<String>,
}

/// A request used to get a [`User`] data with an authorization token that belongs to the user.
///
/// * `client` - A [reqwest client](`reqwest::Client`).
/// * `auth_token` - Authorization token that has been acquired in [authorization
///   module](crate::auth).
pub async fn request_token_user(client: &Client, auth_token: &str) -> Result<User, ReqwestError> {
    let response_result = client
        .get("https://osu.ppy.sh/api/v2/me/")
        .bearer_auth(auth_token)
        .send()
        .await?;

    let response_body: User = response_result.json().await?;
    Ok(response_body)
}

/// A request used to get a [`User`] data with their ID.
///
/// * `client` - A [reqwest client](`reqwest::Client`).
/// * `auth_token` - Authorization token that has been acquired in [authorization
///   module](crate::auth).
/// * `user_id` - ID of the user which the beatmaps are related to.
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
