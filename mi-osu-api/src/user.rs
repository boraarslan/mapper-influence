#![allow(dead_code)]
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::ReqwestError;

static MI_CLIENT_ID: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_CLIENT_ID").expect("Environment variable MI_CLIENT_ID is not set.")
});
static MI_CLIENT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_CLIENT_SECRET").expect("Environment variable MI_CLIENT_SECRET is not set.")
});
static MI_REDIRECT_URI: Lazy<String> = Lazy::new(|| {
    std::env::var("MI_REDIRECT_URI").expect("Environment variable MI_REDIRECT_URI is not set.")
});

#[derive(Deserialize)]
pub struct User {
    avatar_url: String,
    country_code: String,
    default_group: String,
    id: i64,
    is_active: bool,
    is_bot: bool,
    is_deleted: bool,
    is_online: bool,
    is_supporter: bool,
    last_visit: String,
    pm_friends_only: bool,
    profile_colour: String,
    username: String,
    discord: Option<String>,
    has_supported: bool,
    interests: Option<String>,
    join_date: String,
    kudosu: Kudosu,
    location: Option<String>,
    max_blocks: i64,
    max_friends: i64,
    occupation: Option<String>,
    playmode: String,
    playstyle: Vec<String>,
    post_count: i64,
    profile_order: Vec<String>,
    title: Option<String>,
    twitter: Option<String>,
    website: Option<String>,
    country: Country,
    cover: Cover,
    is_restricted: bool,
    // not documented
    account_history: Vec<Option<serde_json::Value>>,
    // not documented
    active_tournament_banner: Option<serde_json::Value>,
    badges: Vec<Badge>,
    favourite_beatmapset_count: i64,
    follower_count: i64,
    graveyard_beatmapset_count: i64,
    groups: Vec<Group>,
    loved_beatmapset_count: i64,
    monthly_playcounts: Vec<Count>,
    page: Page,
    pending_beatmapset_count: i64,
    // not documented but assuming
    previous_usernames: Vec<String>,
    ranked_beatmapset_count: i64,
    replays_watched_counts: Vec<Count>,
    scores_first_count: i64,
    statistics: Statistics,
    support_level: i64,
    user_achievements: Vec<UserAchievement>,
    rank_history: RankHistory,
}

#[derive(Deserialize)]
pub struct Badge {
    awarded_at: String,
    description: String,
    image_url: String,
    url: String,
}

#[derive(Deserialize)]
pub struct Country {
    code: String,
    name: String,
}

#[derive(Deserialize)]
pub struct Cover {
    custom_url: String,
    url: String,
    // Not documented but assuming to be integer
    id: Option<i64>,
}

#[derive(Deserialize)]
pub struct Group {
    id: i64,
    identifier: String,
    name: String,
    short_name: String,
    description: String,
    colour: String,
}

#[derive(Deserialize)]
pub struct Kudosu {
    total: i64,
    available: i64,
}

#[derive(Deserialize)]
pub struct Count {
    start_date: String,
    count: i64,
}

#[derive(Deserialize)]
pub struct Page {
    html: String,
    raw: String,
}

#[derive(Deserialize)]
pub struct RankHistory {
    mode: String,
    data: Vec<i64>,
}

#[derive(Deserialize)]
pub struct Statistics {
    level: Level,
    pp: i64,
    global_rank: i64,
    ranked_score: i64,
    hit_accuracy: f64,
    play_count: i64,
    play_time: i64,
    total_score: i64,
    total_hits: i64,
    maximum_combo: i64,
    replays_watched_by_others: i64,
    is_ranked: bool,
    grade_counts: GradeCounts,
    rank: Rank,
}

#[derive(Deserialize)]
pub struct GradeCounts {
    ss: i64,
    ssh: i64,
    s: i64,
    sh: i64,
    a: i64,
}

#[derive(Deserialize)]
pub struct Level {
    current: i64,
    progress: i64,
}

#[derive(Deserialize)]
pub struct Rank {
    global: i64,
    country: i64,
}

#[derive(Deserialize)]
pub struct UserAchievement {
    achieved_at: String,
    achievement_id: i64,
}


