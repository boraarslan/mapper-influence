use sqlx::{FromRow, PgPool};
use thiserror::Error;

use crate::PG_UNIQUE_KEY_VIOLATION;

#[derive(Debug, FromRow, Clone)]
pub struct User {
    /// Osu user ID of an user
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

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User with id `{0}` is not found.")]
    UserNotFound(i64),
    #[error("User with id `{0}` already exists.")]
    UserAlreadyExists(i64),
    #[error("Internal database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
