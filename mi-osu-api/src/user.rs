//! osu! user API implementation.
//!
//! It is used for requesting [user] data.
//! For more information about beatmap endpoints, visit
//! official osu! API documentation for
//! [user][get_user_beatmaps] endpoints.
//!
//! These endpoints require [access tokens](crate::auth).
//!
//! [user]: <https://osu.ppy.sh/docs/index.html#user>
//! [get_user_beatmaps]: <https://osu.ppy.sh/docs/index.html#get-user-beatmaps>

#![allow(dead_code)]
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{OsuApiError, ResponseWithBody};

/// Information about a user.
///
/// Only the relevant fields are implemented in this crate.
/// For more information about all of the fields, refer to
/// [the official osu! API] for more information.
///
/// [the official osu! API]: <https://osu.ppy.sh/docs/index.html#user>
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// User's profile picture link
    pub avatar_url: String,
    /// Unique ID of the user
    pub id: i64,
    /// Main playmode of the user
    pub playmode: String,
    /// Title of the user. Titles are rare profile text that is awarded when the user does
    /// something significant in the community
    pub title: Option<String>,
    /// Username of the user. Can be changed
    pub username: String,
    /// Country information of the user
    pub country: Country,
    /// Cover image information of the user
    pub cover: Cover,
    /// Information about the group the user might be part of. They are visible as small icons that
    /// are in user profiles
    pub groups: Vec<UserGroup>,
    /// Count of users that are subscribed to this user's beatmap updates
    #[serde(rename = "mapping_follower_count")]
    pub followers: i64,
    /// Count of maps this user has in a given category
    #[serde(flatten)]
    pub stats: BeatmapsetStats,
}

/// Beatmap stats for the user.
///
/// This information is originally a part of [`User`] data. But they are seperated in our
/// implementation for convenience.
///
/// Original field names are different in API and they are shorter in our implementation.
/// Check [the official osu! API] for more information.
///
/// [the official osu! API]: <https://osu.ppy.sh/docs/index.html#user>
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BeatmapsetStats {
    #[serde(rename = "ranked_beatmapset_count")]
    pub ranked: i32,
    #[serde(rename = "loved_beatmapset_count")]
    pub loved: i32,
    #[serde(rename = "nominated_beatmapset_count")]
    pub nominated: i32,
    #[serde(rename = "pending_beatmapset_count")]
    pub pending: i32,
    #[serde(rename = "graveyard_beatmapset_count")]
    pub graveyard: i32,
    #[serde(rename = "guest_beatmapset_count")]
    pub guest: i32,
}

/// Country information of a user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Country {
    pub code: String,
    pub name: String,
}

/// Profile cover image of a user.
///
/// Profile covers are the big rectengular images on top of a player profile.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cover {
    pub custom_url: Option<String>,
    pub url: Option<String>,
}

/// Information about the group the user might be part of.
///
/// Groups are various official organizations in the osu! community.
/// They are visible as small icons that are in user profiles.
/// This struct contains all the essential data to reconstruct these icons.
///
/// Only the relevant fields are implemented in this crate.
/// For more information about all of the fields, refer to
/// [the official osu! API] for more information.
///
/// [the official osu! API]: <https://osu.ppy.sh/docs/index.html#user>
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserGroup {
    /// Probationary users don't have small icons in their profiles
    pub is_probationary: bool,
    /// Name of the group
    pub name: String,
    /// Shortened name of the group. This is what is visible in the user profiles
    pub short_name: String,
    /// Background colour of the group icon
    pub colour: String,
    /// Playmode icons that are shown in the group icon
    pub playmodes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResultWrapper {
    user: SearchResult,
}

/// Wrapper for UserCompact. This struct also includes the number of possible users for this query.
///
/// For more information, refer to
/// [the official osu! API] for more information.
///
/// [the official osu! API]: <https://osu.ppy.sh/docs/index.html#search>
#[derive(Debug, Deserialize)]
pub struct SearchResult {
    /// Data of the users
    pub data: Vec<UserCompact>,
    /// Total number of result in the search query. The request only returns first 100 results but
    /// this field contains the number of all possible results
    pub total: i64,
}

/// Compact Information about a user. Used in search results.
///
/// Only the relevant fields are implemented in this crate.
/// For more information about all of the fields, refer to
/// [the official osu! API] for more information.
///
/// [the official osu! API]: <https://osu.ppy.sh/docs/index.html#usercompact>
#[derive(Debug, Deserialize)]
pub struct UserCompact {
    /// User's profile picture link
    pub avatar_url: String,
    /// 2 digit ISO country code
    pub country_code: String,
    /// Unique ID of the user
    pub id: i64,
    /// Username of the user
    pub username: String,
}

/// A request to get [`User`] data with an authorization token that belongs to the user.
pub async fn request_token_user(client: &Client, auth_token: &str) -> Result<User, OsuApiError> {
    let response_result = client
        .get("https://osu.ppy.sh/api/v2/me/")
        .bearer_auth(auth_token)
        .send()
        .await?;
    response_result.try_deser_api_response().await
}

/// A request to get [`User`] data with their ID.
pub async fn request_user(
    client: &Client,
    auth_token: &str,
    user_id: i64,
) -> Result<User, OsuApiError> {
    let url = format!("https://osu.ppy.sh/api/v2/users/{}", user_id);
    let response_result = client.get(url).bearer_auth(auth_token).send().await?;
    response_result.try_deser_api_response().await
}

/// A request to get [`SearchResult`] data.
///
/// This request returns only first 100 users in the query.
/// Each page has maximum 20 users in it.
pub async fn search_user(
    client: &Client,
    auth_token: &str,
    query: &str,
    page: i64,
) -> Result<SearchResultWrapper, OsuApiError> {
    let response_result = client
        .get("https://osu.ppy.sh/api/v2/search?mode=user")
        .bearer_auth(auth_token)
        .query(&[("query", query), ("page", &page.to_string())])
        .send()
        .await?;
    response_result.try_deser_api_response().await
}
