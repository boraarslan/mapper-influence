use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LeaderboardUser {
    pub id: i64,
    pub user_name: String,
    pub profile_picture: String,
    pub ranked_map_count: i32,
    pub influence_count: Option<i64>,
}

pub async fn get_user_leaderboard(db: &PgPool) -> Result<Vec<LeaderboardUser>, sqlx::Error> {
    sqlx::query_as!(
        LeaderboardUser,
        "WITH top_influencers AS (
            SELECT from_id, COUNT(*) AS influence_count
            FROM influences
            GROUP BY from_id
            ORDER BY influence_count DESC
            LIMIT 20
        )
        SELECT
            users.id,
            users.user_name,
            users.profile_picture,
            users_osu_data.ranked_count as ranked_map_count,
            top_influencers.influence_count
        FROM top_influencers
        INNER JOIN users ON id = from_id
        INNER JOIN users_osu_data ON users.id = users_osu_data.user_id"
    )
    .fetch_all(db)
    .await
}
