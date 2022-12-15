//! osu! search API implementation.
//!
//! It is used for searching osu users.
//! For more information about beatmap endpoints, visit
//! official osu! API documentation for
//! [search]endpoint.
//!
//! These endpoints require [access tokens](crate::auth).
//!
//! [search]: <https://osu.ppy.sh/docs/index.html#search>

use reqwest::Client;
use serde::Deserialize;

use crate::ReqwestError;

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
    /// Username of the user. Can be changed
    pub username: String,
}

/// A request to get [`SearchResult`] data.
///
/// This request returns only first 100 users in the query.
/// Each page has maximum 20 users in it.
pub async fn request_user(
    client: &Client,
    auth_token: &str,
    query: &str,
    page: i64,
) -> Result<SearchResult, ReqwestError> {
    let url = format!(
        "https://osu.ppy.sh/api/v2/search?mode=user&query={}&page={}",
        query, page
    );
    let response_result = client.get(url).bearer_auth(auth_token).send().await?;
    let response_body: SearchResult = response_result.json().await?;
    Ok(response_body)
}
