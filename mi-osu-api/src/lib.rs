//! osu! API library for Mapper Influence backend.
//! Used to request data from official osu! API.
//!
//! This is not a complete implementation of the API,
//! rather only the endpoints, which are relevant in the website, are present.

mod auth;
mod beatmap;
mod user;

type ReqwestError = reqwest::Error;
