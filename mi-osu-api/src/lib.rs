//! osu! API library for Mapper Influence backend.
//! It is used for requesting data from [official osu! API].
//!
//! It is not a complete implementation of the API,
//! rather, only the endpoints, which are relevant to the website, are present.
//!
//! [official osu! API]: <https://osu.ppy.sh/docs/index.html>

pub mod auth;
pub mod beatmap;
pub mod user;

pub use crate::beatmap::*;
pub use crate::user::*;

pub type ReqwestError = reqwest::Error;
