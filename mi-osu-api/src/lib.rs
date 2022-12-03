//! osu! API library for Mapper Influence backend.
//! Used to request data from [official osu! API](https://osu.ppy.sh/docs/index.html).
//!
//! This is not a complete implementation of the API,
//! rather only the endpoints, which are relevant in the website, are present.

pub mod auth;
pub mod beatmap;
pub mod user;

/// Type alias for [reqwest::Error]
pub type ReqwestError = reqwest::Error;
