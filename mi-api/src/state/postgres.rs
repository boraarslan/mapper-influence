use axum::extract::FromRef;
use mi_core::future_log_ext::FutureLogExt;
use mi_db::{FeaturedMaps, FullUser, Influence, InfluenceError, LeaderboardUser, User, UserError};
use mi_osu_api::Beatmapset;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::instrument;

use super::{SharedState, DB_POOL};
use crate::api::leaderboard::LeaderboardError;

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

        DB_POOL.set(pool).expect("Failed to set DB_POOL");

        Self {
            pool: DB_POOL.get().unwrap().clone(),
        }
    }

    #[instrument(skip(self), fields(elapsed),ret)]
    pub async fn get_user(&self, user_id: i64) -> Result<User, UserError> {
        mi_db::get_user(user_id, &self.pool).log_elapsed().await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn get_full_user(&self, user_id: i64) -> Result<FullUser, UserError> {
        mi_db::get_full_user(user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn insert_user(&self, user: User) -> Result<User, UserError> {
        mi_db::init_user(user, &self.pool).log_elapsed().await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn update_user_name(&self, user_name: &str, user_id: i64) -> Result<(), UserError> {
        mi_db::update_user_name(user_name, user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn update_user_picture(
        &self,
        user_picture: &str,
        user_id: i64,
    ) -> Result<(), UserError> {
        mi_db::update_user_picture(user_picture, user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn update_user_bio(
        &self,
        user_bio: Option<&str>,
        user_id: i64,
    ) -> Result<(), UserError> {
        mi_db::update_user_bio(user_bio, user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn get_user_influencers(
        &self,
        user_id: i64,
    ) -> Result<Vec<Influence>, InfluenceError> {
        mi_db::get_all_influences_by_to_id(user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn insert_influence(&self, influence: Influence) -> Result<(), InfluenceError> {
        mi_db::insert_influence(influence, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn update_influence_level(
        &self,
        from_id: i64,
        to_id: i64,
        level: i32,
    ) -> Result<(), InfluenceError> {
        mi_db::update_influence_level(from_id, to_id, level, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn update_influence_info(
        &self,
        from_id: i64,
        to_id: i64,
        info: Option<&str>,
    ) -> Result<(), InfluenceError> {
        mi_db::update_influence_info(from_id, to_id, info, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn delete_influence(&self, from_id: i64, to_id: i64) -> Result<(), InfluenceError> {
        mi_db::delete_influence(from_id, to_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn update_user_osu_data(&self, user: mi_osu_api::User) -> Result<(), UserError> {
        mi_db::update_user_osu_data(user, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn update_user_featured_maps(
        &self,
        user_id: i64,
        maps: FeaturedMaps,
    ) -> Result<(), UserError> {
        mi_db::update_user_featured_maps(user_id, maps, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn get_user_mapsets(&self, user_id: i64) -> Result<Vec<Beatmapset>, UserError> {
        mi_db::get_user_mapsets(user_id, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self), fields(elapsed), ret)]
    pub async fn upsert_user_mapsets(
        &self,
        user_id: i64,
        mapsets: Vec<Beatmapset>,
    ) -> Result<(), UserError> {
        mi_db::upsert_user_mapsets(user_id, mapsets, &self.pool)
            .log_elapsed()
            .await
    }

    #[instrument(skip(self),  fields(elapsed), ret)]
    pub async fn get_user_leaderboard(&self) -> Result<Vec<LeaderboardUser>, LeaderboardError> {
        mi_db::get_user_leaderboard(&self.pool)
            .log_elapsed()
            .await
            .map_err(|e| e.into())
    }
}

impl FromRef<SharedState> for PgDb {
    fn from_ref(state: &SharedState) -> Self {
        state.postgres.clone()
    }
}
