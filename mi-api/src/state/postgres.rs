use axum::extract::FromRef;
use mi_db::influence::{
    delete_influence, get_all_influences_by_to_id, insert_influence, update_influence_info,
    update_influence_level, Influence, InfluenceError,
};
use mi_db::leaderboard::{get_user_leaderboard, LeaderboardUser};
use mi_db::{
    get_full_user, get_user, get_user_mapsets, init_user, update_user_bio,
    update_user_featured_maps, update_user_name, update_user_osu_data, update_user_picture,
    upsert_user_mapsets, FeaturedMaps, FullUser, User, UserError,
};
use mi_osu_api::Beatmapset;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::instrument;

use super::SharedState;
use crate::future_log_ext::FutureLogExt;

#[derive(Debug, Clone)]
pub struct PgDb {
    pool: PgPool,
}

impl PgDb {
    pub async fn new() -> Self {
        let url = std::env::var("DATABASE_URL").expect("PostgreSQL URL is not set!");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .unwrap();

        Self { pool }
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn get_user(&self, user_id: i64) -> Result<User, UserError> {
        get_user(user_id, &self.pool).log_elapsed().await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn get_full_user(&self, user_id: i64) -> Result<FullUser, UserError> {
        get_full_user(user_id, &self.pool).log_elapsed().await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn insert_user(&self, user: User) -> Result<User, UserError> {
        init_user(user, &self.pool).log_elapsed().await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn update_user_name(&self, user_name: &str, user_id: i64) -> Result<(), UserError> {
        update_user_name(user_name, user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn update_user_picture(
        &self,
        user_picture: &str,
        user_id: i64,
    ) -> Result<(), UserError> {
        update_user_picture(user_picture, user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn update_user_bio(
        &self,
        user_bio: Option<&str>,
        user_id: i64,
    ) -> Result<(), UserError> {
        update_user_bio(user_bio, user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn get_user_influencers(
        &self,
        user_id: i64,
    ) -> Result<Vec<Influence>, InfluenceError> {
        get_all_influences_by_to_id(user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn insert_influence(&self, influence: Influence) -> Result<(), InfluenceError> {
        insert_influence(influence, &self.pool).log_elapsed().await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn update_influence_level(
        &self,
        from_id: i64,
        to_id: i64,
        level: i32,
    ) -> Result<(), InfluenceError> {
        update_influence_level(from_id, to_id, level, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn update_influence_info(
        &self,
        from_id: i64,
        to_id: i64,
        info: Option<&str>,
    ) -> Result<(), InfluenceError> {
        update_influence_info(from_id, to_id, info, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn delete_influence(&self, from_id: i64, to_id: i64) -> Result<(), InfluenceError> {
        delete_influence(from_id, to_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn update_user_osu_data(&self, user: mi_osu_api::User) -> Result<(), UserError> {
        update_user_osu_data(user, &self.pool).log_elapsed().await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn update_user_featured_maps(
        &self,
        user_id: i64,
        maps: FeaturedMaps,
    ) -> Result<(), UserError> {
        update_user_featured_maps(user_id, maps, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn get_user_mapsets(&self, user_id: i64) -> Result<Vec<Beatmapset>, UserError> {
        get_user_mapsets(user_id, &self.pool).log_elapsed().await
    }

    #[instrument(skip(self),fields(elapsed) err, ret)]
    pub async fn upsert_user_mapsets(
        &self,
        user_id: i64,
        mapsets: Vec<Beatmapset>,
    ) -> Result<(), UserError> {
        upsert_user_mapsets(user_id, mapsets, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed) err, ret)]
    pub async fn get_user_leaderboard(&self) -> Result<Vec<LeaderboardUser>, sqlx::Error> {
        get_user_leaderboard(&self.pool).await
    }
}

impl FromRef<SharedState> for PgDb {
    fn from_ref(state: &SharedState) -> Self {
        state.postgres.clone()
    }
}
