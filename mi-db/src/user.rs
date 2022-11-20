use sqlx::{FromRow, PgPool};
use thiserror::Error;

use crate::PG_UNIQUE_KEY_VIOLATION;

#[derive(Debug, FromRow, Clone, PartialEq, Eq)]
pub struct User {
    /// Osu user ID of a user
    pub id: i64,
    /// Last known user name of the user
    pub user_name: String,
    /// Url to user profile picture
    pub profile_picture: String,
    /// User biography
    pub bio: Option<String>,
}

pub async fn search_user(user_id: i64, db: &PgPool) -> Result<User, UserError> {
    let search_result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(db)
        .await;

    match search_result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

pub async fn insert_user(user: User, db: &PgPool) -> Result<(), UserError> {
    let insert_result = sqlx::query!(
        "
        INSERT INTO users (id, user_name, profile_picture, bio) VALUES ($1, $2, $3, $4)",
        user.id,
        user.user_name,
        user.profile_picture,
        user.bio
    )
    .execute(db)
    .await;

    match insert_result {
        Ok(_) => Ok(()),
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
        "UPDATE users SET bio = $1 WHERE id = $2 RETURNING id",
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

pub async fn delete_user(user_id: i64, db: &PgPool) -> Result<(), UserError> {
    let delete_result = sqlx::query!("DELETE FROM users WHERE id = $1 RETURNING id", user_id)
        .fetch_one(db)
        .await;

    match delete_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(UserError::UserNotFound(user_id)),
        Err(db_err) => Err(UserError::from(db_err)),
    }
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User with id `{0}` is not found.")]
    UserNotFound(i64),
    #[error("User with id `{0}` already exists.")]
    UserAlreadyExists(i64),
    #[error("Internal database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
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
            bio: Some("I am tired.".to_string()),
        }
    }

    #[sqlx::test]
    async fn test_insert_user(db: PgPool) {
        // Test user insert
        let user = user_for_test(1);
        insert_user(user.clone(), &db).await.unwrap();
        let db_user = search_user(user.id, &db).await.unwrap();
        assert_eq!(user, db_user);

        // Test user insert with optional field
        let mut user = user_for_test(2);
        user.bio = None;
        insert_user(user.clone(), &db).await.unwrap();
        let db_user = search_user(user.id, &db).await.unwrap();
        assert_eq!(user.clone(), db_user);

        // Test user insert with duplicate keys
        let user_second = User {
            // Using the key of the previously inserted user for key violation test
            id: user.id,
            user_name: "fursum".to_string(),
            profile_picture: "random.imageservice.com/fursum.jpg".to_string(),
            bio: None,
        };
        let error = insert_user(user_second, &db).await.unwrap_err();
        match error {
            UserError::UserAlreadyExists(2) => {}
            _ => panic!("Database should return key violation error on duplicate entries."),
        }
    }

    #[sqlx::test]
    async fn test_update_user(db: PgPool) {
        // Test username update
        let user = user_for_test(1);
        insert_user(user.clone(), &db).await.unwrap();
        update_user_name("fursum", user.id, &db).await.unwrap();
        let db_user = search_user(user.id, &db).await.unwrap();
        assert_eq!(db_user.user_name, "fursum".to_string());
        assert_eq!(user.bio, db_user.bio);
        assert_eq!(user.profile_picture, db_user.profile_picture);

        // Test profile picture update
        let user = user_for_test(2);
        insert_user(user.clone(), &db).await.unwrap();
        update_user_picture("random.someothersite.com/bora2.jpeg", user.id, &db)
            .await
            .unwrap();

        let db_user = search_user(user.id, &db).await.unwrap();
        assert_eq!(
            db_user.profile_picture,
            "random.someothersite.com/bora2.jpeg".to_string()
        );
        assert_eq!(user.user_name, db_user.user_name);
        assert_eq!(user.bio, db_user.bio);

        // Test user bio update
        let user = user_for_test(3);
        insert_user(user.clone(), &db).await.unwrap();
        update_user_bio(Some("I changed my mind."), user.id, &db)
            .await
            .unwrap();
        let db_user = search_user(user.id, &db).await.unwrap();
        assert_eq!(db_user.bio, Some("I changed my mind.".to_string()));
        assert_eq!(user.profile_picture, db_user.profile_picture);
        assert_eq!(user.user_name, db_user.user_name);

        // Test user bio update to none value
        update_user_bio(None, user.id, &db).await.unwrap();
        let db_user = search_user(user.id, &db).await.unwrap();
        assert_eq!(db_user.bio, None);
    }

    #[sqlx::test]
    async fn test_delete_user(db: PgPool) {
        let user = user_for_test(1);
        insert_user(user.clone(), &db).await.unwrap();
        let db_user = search_user(user.id, &db).await.unwrap();
        assert_eq!(user, db_user);

        delete_user(user.id, &db).await.unwrap();
        let err = search_user(user.id, &db).await.unwrap_err();

        match err {
            UserError::UserNotFound(db_user_id) => {
                assert_eq!(user.id, db_user_id)
            }
            _ => panic!("{}", NOT_FOUND_ERROR_TEXT),
        }
    }

    #[sqlx::test]
    async fn test_non_existent(db: PgPool) {
        // Test search for non-existent user
        let err = search_user(-100, &db).await.unwrap_err();
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
        let err = delete_user(-100, &db).await.unwrap_err();
        match err {
            UserError::UserNotFound(-100) => {}
            _ => panic!("{}", NOT_FOUND_ERROR_TEXT),
        }
    }
}
