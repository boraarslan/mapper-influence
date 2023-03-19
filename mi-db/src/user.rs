use chrono::Utc;
use futures::FutureExt;
use mi_osu_api::Beatmapset;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{FromRow, PgPool};
use thiserror::Error;
use tokio::try_join;
use utoipa::ToSchema;

use crate::PG_UNIQUE_KEY_VIOLATION;

#[derive(Debug, FromRow, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct User {
    /// Osu user ID of a user
    pub id: i64,
    /// Last known user name of the user
    pub user_name: String,
    /// Url to user profile picture
    pub profile_picture: String,
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// Osu user ID of a user (references user id from `users` table)
    pub user_id: i64,
    // User bio
    pub bio: Option<String>,
    // Featured maps of the user
    pub featured_maps: Option<Json<FeaturedMaps>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FeaturedMaps {
    pub maps: Vec<Maps>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Maps {
    // Beatmapset that featured map belongs to
    pub beatmapset: Beatmapset,
    // Id of the featued map
    pub featured_map_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOsuData {
    /// Osu user ID of a user (references user id from `users` table)
    pub user_id: i64,
    /// Ranked map count
    pub ranked_count: i32,
    /// Loved map count
    pub loved_count: i32,
    /// Nominated map count
    pub nominated_count: i32,
    /// Graveyard map count
    pub graveyard_count: i32,
    /// Guest map count
    pub guest_count: i32,
    // Last modified timestamp
    pub modified_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FullUser {
    /// Osu user ID of a user
    pub id: i64,
    /// Last known user name of the user
    pub user_name: String,
    /// Url to user profile picture
    pub profile_picture: String,
    // User bio
    pub bio: Option<String>,
    // Featured maps of the user
    #[schema(value_type = Option<FeaturedMaps>)]
    pub featured_maps: Option<Json<FeaturedMaps>>,
    /// Ranked map count
    pub ranked_count: i32,
    /// Loved map count
    pub loved_count: i32,
    /// Nominated map count
    pub nominated_count: i32,
    /// Graveyard map count
    pub graveyard_count: i32,
    /// Guest map count
    pub guest_count: i32,
    /// Last Osu! data modified timestamp
    pub osu_data_modified_at: chrono::DateTime<Utc>,
}

impl FullUser {
    pub fn is_outdated(&self) -> bool {
        let now = Utc::now();
        let diff = now - self.osu_data_modified_at;
        diff.num_hours() > 3
    }
}

impl From<mi_osu_api::User> for User {
    fn from(osu_user: mi_osu_api::User) -> Self {
        Self {
            id: osu_user.id,
            user_name: osu_user.username,
            profile_picture: osu_user.avatar_url,
        }
    }
}

pub async fn get_user(user_id: i64, db: &PgPool) -> Result<User, UserError> {
    let search_result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(db)
        .await;

    match search_result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn get_full_user(user_id: i64, db: &PgPool) -> Result<FullUser, UserError> {
    let search_result = sqlx::query_as!(
        FullUser,
        r#"
        SELECT 
            id, user_name, profile_picture, 
            profile.bio, 
            profile.featured_maps as "featured_maps: Json<FeaturedMaps>", 
            osu.ranked_count, osu.loved_count, osu.nominated_count, osu.graveyard_count, osu.guest_count,
            osu.modified_at as osu_data_modified_at
        FROM users 
        INNER JOIN user_profiles profile ON profile.user_id = $1 
        INNER JOIN users_osu_data osu ON osu.user_id = $1"#,
        user_id
    ).fetch_one(db).await;

    match search_result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn update_user_osu_data(
    user_osu_data: mi_osu_api::User,
    db: &PgPool,
) -> Result<(), UserError> {
    let query_result = sqlx::query!(
        r#"
        UPDATE 
            users_osu_data 
                SET (ranked_count, loved_count, nominated_count, graveyard_count, guest_count, modified_at) = 
                ($2 , $3, $4, $5, $6, DEFAULT) 
        WHERE 
            user_id = $1 "#,
        user_osu_data.id,
        user_osu_data.stats.ranked,
        user_osu_data.stats.loved,
        user_osu_data.stats.nominated,
        user_osu_data.stats.graveyard,
        user_osu_data.stats.guest,
    ).execute(db).await;

    match query_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_osu_data.id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn update_user_featured_maps(
    user_id: i64,
    maps: FeaturedMaps,
    db: &PgPool,
) -> Result<(), UserError> {
    let query_result = sqlx::query!(
        r#"
            UPDATE user_profiles SET featured_maps = $1 WHERE user_id = $2
        "#,
        serde_json::to_value(&maps)?,
        user_id
    )
    .execute(db)
    .await;

    match query_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn get_user_mapsets(user_id: i64, db: &PgPool) -> Result<Vec<Beatmapset>, UserError> {
    let result = sqlx::query!(
        r#"SELECT mapsets as "mapsets: Json<Vec<Beatmapset>>" FROM user_osu_maps WHERE user_id = $1"#,
        user_id
    )
    .fetch_optional(db)
    .await;

    match result {
        Ok(Some(row)) => match row.mapsets {
            Some(mapsets) => Ok(mapsets.0),
            None => Ok(vec![]),
        },
        Ok(None) => Ok(vec![]),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn upsert_user_mapsets(
    user_id: i64,
    mapsets: Vec<Beatmapset>,
    db: &PgPool,
) -> Result<(), UserError> {
    let result = sqlx::query!(
        r#"INSERT INTO user_osu_maps (user_id, mapsets) VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE SET mapsets = $2"#,
        user_id,
        serde_json::to_value(&mapsets)?,
    ).execute(db).await;

    match result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn init_user(user: User, db: &PgPool) -> Result<User, UserError> {
    let insert_user = sqlx::query!(
        r#"
        INSERT INTO users (id, user_name, profile_picture) VALUES ($1, $2, $3)"#,
        user.id,
        user.user_name,
        user.profile_picture,
    )
    .execute(db);

    let insert_profile = sqlx::query!(
        r#"
        INSERT INTO user_profiles (user_id) VALUES ($1)"#,
        user.id,
    )
    .execute(db);
    let insert_osu_data = sqlx::query!(
        r#"INSERT INTO users_osu_data (user_id) VALUES ($1)"#,
        user.id
    )
    .execute(db);

    let insert_result =
        insert_user.then(|_user_res| async move { try_join!(insert_profile, insert_osu_data) });

    match insert_result.await {
        Ok(_) => Ok(user),
        Err(db_err) if db_err.as_database_error().is_some() => {
            // We check if db_err can be casted to database_error.
            // PgError should always return a valid error code.
            let pg_db_error_code = db_err.as_database_error().unwrap().code().unwrap();

            if pg_db_error_code.eq(PG_UNIQUE_KEY_VIOLATION) {
                Err(UserError::UserAlreadyExists(user.id))
            } else {
                Err(UserError::from(db_err))
            }
        }
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn update_user_name(user_name: &str, user_id: i64, db: &PgPool) -> Result<(), UserError> {
    let update_result = sqlx::query!(
        "UPDATE users SET user_name = $1 WHERE id = $2 RETURNING id",
        user_name,
        user_id,
    )
    .fetch_one(db)
    .await;

    match update_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn update_user_picture(
    user_picture: &str,
    user_id: i64,
    db: &PgPool,
) -> Result<(), UserError> {
    let update_result = sqlx::query!(
        "UPDATE users SET profile_picture = $1 WHERE id = $2 RETURNING id",
        user_picture,
        user_id,
    )
    .fetch_one(db)
    .await;

    match update_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn update_user_bio(
    user_bio: Option<&str>,
    user_id: i64,
    db: &PgPool,
) -> Result<(), UserError> {
    let update_result = sqlx::query!(
        "UPDATE user_profiles SET bio = $1 WHERE user_id = $2 RETURNING user_id",
        user_bio,
        user_id,
    )
    .fetch_one(db)
    .await;

    match update_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

// Currently, we don't need this
//
// pub async fn delete_user(user_id: i64, db: &PgPool) -> Result<(), UserError> {
//     let delete_result = sqlx::query!("DELETE FROM users WHERE id = $1 RETURNING id", user_id)
//         .fetch_one(db)
//         .await;

//     match delete_result {
//         Ok(_) => Ok(()),
//         Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
//         Err(db_err) => Err(UserError::from(db_err)),
//     }
// }

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User with id `{0}` is not found.")]
    UserNotFound(i64),
    #[error("User with id `{0}` already exists.")]
    UserAlreadyExists(i64),
    #[error("Internal database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Failed to serialize Json: {0}")]
    SerdeError(#[from] serde_json::Error),
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use sqlx::PgPool;

    use super::*;

    const NOT_FOUND_ERROR_TEXT: &str = "Query against absent users should return NotFound error.";

    fn user_for_test(id: i64) -> User {
        User {
            id,
            user_name: "boraarslan".to_string(),
            profile_picture: "random.imageservice.com/boraarslan.jpg".to_string(),
        }
    }

    #[sqlx::test]
    async fn test_insert_user(db: PgPool) {
        // Test user insert
        let user = user_for_test(1);
        init_user(user.clone(), &db).await.unwrap();
        let db_user = get_user(user.id, &db).await.unwrap();
        assert_eq!(user, db_user);

        // Test user insert with duplicate keys
        let user_second = User {
            // Using the key of the previously inserted user for key violation test
            id: user.id,
            user_name: "fursum".to_string(),
            profile_picture: "random.imageservice.com/fursum.jpg".to_string(),
        };
        let error = init_user(user_second, &db).await.unwrap_err();
        match error {
            UserError::UserAlreadyExists(1) => {}
            _ => panic!("Database should return key violation error on duplicate entries."),
        }
    }

    #[sqlx::test]
    async fn test_update_user(db: PgPool) {
        // Test username update
        let user = user_for_test(1);
        init_user(user.clone(), &db).await.unwrap();
        update_user_name("fursum", user.id, &db).await.unwrap();
        let db_user = get_user(user.id, &db).await.unwrap();
        assert_eq!(db_user.user_name, "fursum".to_string());
        assert_eq!(user.profile_picture, db_user.profile_picture);

        // Test profile picture update
        let user = user_for_test(2);
        init_user(user.clone(), &db).await.unwrap();
        update_user_picture("random.someothersite.com/bora2.jpeg", user.id, &db)
            .await
            .unwrap();

        let db_user = get_user(user.id, &db).await.unwrap();
        assert_eq!(
            db_user.profile_picture,
            "random.someothersite.com/bora2.jpeg".to_string()
        );
        assert_eq!(user.user_name, db_user.user_name);

        // Test user bio update
        let user = user_for_test(3);
        init_user(user.clone(), &db).await.unwrap();
        update_user_bio(Some("I changed my mind."), user.id, &db)
            .await
            .unwrap();
        let db_user = get_user(user.id, &db).await.unwrap();
        assert_eq!(user.profile_picture, db_user.profile_picture);
        assert_eq!(user.user_name, db_user.user_name);
    }

    // #[sqlx::test]
    // async fn test_delete_user(db: PgPool) {
    //     let user = user_for_test(1);
    //     insert_user(user.clone(), &db).await.unwrap();
    //     let db_user = get_user(user.id, &db).await.unwrap();
    //     assert_eq!(user, db_user);

    //     delete_user(user.id, &db).await.unwrap();
    //     let err = get_user(user.id, &db).await.unwrap_err();

    //     match err {
    //         UserError::UserNotFound(db_user_id) => {
    //             assert_eq!(user.id, db_user_id)
    //         }
    //         _ => panic!("{}", NOT_FOUND_ERROR_TEXT),
    //     }
    // }

    #[sqlx::test]
    async fn test_non_existent(db: PgPool) {
        // Test search for non-existent user
        let err = get_user(-100, &db).await.unwrap_err();
        match err {
            UserError::UserNotFound(-100) => {}
            _ => panic!("{}", NOT_FOUND_ERROR_TEXT),
        }

        // Test username update for non-existent user
        let err = update_user_name("112does_not_matter", -100, &db)
            .await
            .unwrap_err();
        match err {
            UserError::UserNotFound(-100) => {}
            _ => panic!("{}", NOT_FOUND_ERROR_TEXT),
        }

        // Test user bio update for non-existent user
        let err = update_user_bio(None, -100, &db).await.unwrap_err();
        match err {
            UserError::UserNotFound(-100) => {}
            _ => panic!("{}", NOT_FOUND_ERROR_TEXT),
        }

        // Test user update for non-existent user
        let err = update_user_picture("does_not_matter.png", -100, &db)
            .await
            .unwrap_err();
        match err {
            UserError::UserNotFound(-100) => {}
            _ => panic!("{}", NOT_FOUND_ERROR_TEXT),
        }

        // Test user delete for non-existet user
        // let err = delete_user(-100, &db).await.unwrap_err();
        // match err {
        //     UserError::UserNotFound(-100) => {}
        //     _ => panic!("{}", NOT_FOUND_ERROR_TEXT),
        // }
    }
}
