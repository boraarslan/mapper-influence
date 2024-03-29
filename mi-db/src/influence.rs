use chrono::Utc;
use mi_core::error::{AppErrorExt, ErrorType};
use mi_core::INTERNAL_DB_ERROR_MESSAGE;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use thiserror::Error;
use tracing::{error, warn};
use utoipa::ToSchema;

use crate::{PG_FOREIGN_KEY_VIOLATION, PG_UNIQUE_KEY_VIOLATION};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct Influence {
    /// Id of the influencer user
    from_id: i64,
    /// Id of the influenced user
    to_id: i64,
    /// Level of influence
    influence_level: i32,
    /// Extra info/notes about influence
    info: Option<String>,
    /// Creation date. Not used during inserts and defaulted
    created_at: chrono::DateTime<Utc>,
    /// Last modification date. Not used during inserts and defaulted
    modified_at: chrono::DateTime<Utc>,
}

impl Influence {
    pub fn new(from_id: i64, to_id: i64, influence_level: i32, info: Option<String>) -> Self {
        Self {
            from_id,
            to_id,
            influence_level,
            info,
            ..Default::default()
        }
    }
}

pub async fn get_all_influences_by_from_id(
    user_id: i64,
    db: &PgPool,
) -> Result<Vec<Influence>, InfluenceError> {
    let search_result = sqlx::query_as!(
        Influence,
        "SELECT * FROM influences WHERE from_id = $1",
        user_id
    )
    .fetch_all(db)
    .await;

    match search_result {
        Ok(influences) => Ok(influences),
        Err(db_err) => Err(InfluenceError::from(db_err)),
    }
}

pub async fn get_all_influences_by_to_id(
    user_id: i64,
    db: &PgPool,
) -> Result<Vec<Influence>, InfluenceError> {
    let search_result = sqlx::query_as!(
        Influence,
        "SELECT * FROM influences WHERE to_id = $1",
        user_id
    )
    .fetch_all(db)
    .await;

    match search_result {
        Ok(influences) => Ok(influences),
        Err(db_err) => Err(InfluenceError::from(db_err)),
    }
}

pub async fn insert_influence(influence: Influence, db: &PgPool) -> Result<(), InfluenceError> {
    let insert_result = sqlx::query!(
        "INSERT INTO influences (from_id, to_id, influence_level, info) VALUES ($1, $2, $3, $4) \
         RETURNING from_id",
        influence.from_id,
        influence.to_id,
        influence.influence_level,
        influence.info,
    )
    .fetch_one(db)
    .await;

    match insert_result {
        Ok(_) => Ok(()),
        Err(db_err) if db_err.as_database_error().is_some() => {
            // We check if db_err can be casted to database_error.
            // PgError should always return a valid error code.
            let pg_db_error_code = db_err.as_database_error().unwrap().code().unwrap();

            match pg_db_error_code.as_ref() {
                PG_UNIQUE_KEY_VIOLATION => Err(InfluenceError::InfluenceAlreadyExists(
                    influence.from_id,
                    influence.to_id,
                )),
                PG_FOREIGN_KEY_VIOLATION => Err(InfluenceError::UserDoesNotExist(
                    influence.from_id,
                    influence.to_id,
                )),
                _ => Err(InfluenceError::from(db_err)),
            }
        }
        Err(db_err) => Err(InfluenceError::from(db_err)),
    }
}

pub async fn update_influence_level(
    from_id: i64,
    to_id: i64,
    influence_level: i32,
    db: &PgPool,
) -> Result<(), InfluenceError> {
    let update_result = sqlx::query!(
        "UPDATE influences SET (influence_level, modified_at) = ($1, DEFAULT) WHERE from_id = $2 \
         AND to_id = $3 RETURNING from_id",
        influence_level,
        from_id,
        to_id
    )
    .fetch_one(db)
    .await;

    match update_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(InfluenceError::InfluenceNotFoundWithPrimaryKey(
            from_id, to_id,
        )),
        Err(db_err) => Err(InfluenceError::from(db_err)),
    }
}

pub async fn update_influence_info(
    from_id: i64,
    to_id: i64,
    info: Option<&str>,
    db: &PgPool,
) -> Result<(), InfluenceError> {
    let update_result = sqlx::query!(
        "UPDATE influences SET (info, modified_at) = ($1, DEFAULT) WHERE from_id = $2 AND to_id = \
         $3 RETURNING from_id",
        info,
        from_id,
        to_id
    )
    .fetch_one(db)
    .await;

    match update_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(InfluenceError::InfluenceNotFoundWithPrimaryKey(
            from_id, to_id,
        )),
        Err(db_err) => Err(InfluenceError::from(db_err)),
    }
}

pub async fn delete_influence(from_id: i64, to_id: i64, db: &PgPool) -> Result<(), InfluenceError> {
    let delete_result = sqlx::query!(
        "DELETE FROM influences WHERE from_id = $1 AND to_id = $2 RETURNING from_id",
        from_id,
        to_id
    )
    .fetch_one(db)
    .await;

    match delete_result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::RowNotFound) => Err(InfluenceError::InfluenceNotFoundWithPrimaryKey(
            from_id, to_id,
        )),
        Err(db_err) => Err(InfluenceError::from(db_err)),
    }
}

#[derive(Debug, Error)]
pub enum InfluenceError {
    #[error("No influence with from_id `{0}` and to_id `{1}` exists.")]
    InfluenceNotFoundWithPrimaryKey(i64, i64),
    #[error("User does not exist on `users` table. from_id `{0}`, to_id `{1}`")]
    UserDoesNotExist(i64, i64),
    #[error("Influence already exist from user `{0}` to user `{1}`")]
    InfluenceAlreadyExists(i64, i64),
    #[error("Internal database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl AppErrorExt for InfluenceError {
    fn user_message(&self) -> String {
        match self {
            InfluenceError::InfluenceNotFoundWithPrimaryKey(_, _) => self.to_string(),
            InfluenceError::UserDoesNotExist(_, _) => self.to_string(),
            InfluenceError::InfluenceAlreadyExists(_, _) => self.to_string(),
            InfluenceError::DatabaseError(_) => INTERNAL_DB_ERROR_MESSAGE.to_string(),
        }
    }

    fn error_type(&self) -> ErrorType {
        match self {
            InfluenceError::InfluenceNotFoundWithPrimaryKey(_, _) => ErrorType::DataNotFound,
            InfluenceError::UserDoesNotExist(_, _) => ErrorType::DataNotFound,
            InfluenceError::InfluenceAlreadyExists(_, _) => ErrorType::DuplicateEntry,
            InfluenceError::DatabaseError(_) => ErrorType::DatabaseError,
        }
    }

    fn log_error(&self) {
        match self {
            InfluenceError::InfluenceNotFoundWithPrimaryKey(from_id, to_id) => {
                warn!(from_id, to_id, "{}", self.to_string())
            }
            InfluenceError::UserDoesNotExist(from_id, to_id) => {
                warn!(from_id, to_id, "{}", self.to_string())
            }
            InfluenceError::InfluenceAlreadyExists(from_id, to_id) => {
                warn!(from_id, to_id, "{}", self.to_string())
            }
            InfluenceError::DatabaseError(db_err) => error!("{}", db_err),
        }
    }
}

#[cfg(all(test, feature = "db-tests"))]
mod tests {
    use sqlx::PgPool;

    use super::*;
    use crate::user::{init_user, User};

    fn user_for_test(user_id: i64) -> User {
        User::new(
            user_id,
            "boraarslan".to_string(),
            "random.imageservice.com/boraarslan.jpg".to_string(),
        )
    }

    fn influence_for_test(first_id: i64, second_id: i64) -> Influence {
        Influence::new(first_id, second_id, 3, None)
    }

    #[sqlx::test]
    async fn test_influence_db(db: PgPool) {
        let response = get_all_influences_by_from_id(1, &db).await.unwrap();

        assert!(response.is_empty());

        let first_user = user_for_test(1);
        let second_user = user_for_test(2);
        init_user(first_user.clone(), &db).await.unwrap();
        init_user(second_user.clone(), &db).await.unwrap();

        let influence = influence_for_test(first_user.id, second_user.id);

        insert_influence(influence, &db).await.unwrap();

        let influence = get_all_influences_by_from_id(first_user.id, &db)
            .await
            .unwrap();

        assert_eq!(influence.len(), 1);
        assert_eq!(influence[0].to_id, second_user.id);
        assert_eq!(influence[0].info, None);

        let influence = influence_for_test(second_user.id, first_user.id);

        insert_influence(influence, &db).await.unwrap();

        let influence = get_all_influences_by_from_id(second_user.id, &db)
            .await
            .unwrap();

        assert_eq!(influence.len(), 1);
        assert_eq!(influence[0].to_id, first_user.id);
        assert_eq!(influence[0].info, None);

        let mut duplicate_influence = influence_for_test(first_user.id, second_user.id);
        // Only keep the primary key the same
        duplicate_influence.info = Some("Some text".to_string());
        duplicate_influence.influence_level = 1;
        let error = insert_influence(duplicate_influence, &db)
            .await
            .unwrap_err();

        match error {
            InfluenceError::InfluenceAlreadyExists(1, 2) => {}
            _ => panic!("Should return unique_key_violation error."),
        }

        let invalid_influence = influence_for_test(first_user.id, 3); // No user with id 3

        let error = insert_influence(invalid_influence, &db).await.unwrap_err();

        match error {
            InfluenceError::UserDoesNotExist(1, 3) => {}
            _ => panic!("Should return foreign_key_violation error."),
        }

        delete_influence(second_user.id, first_user.id, &db)
            .await
            .unwrap();
        let response = get_all_influences_by_from_id(second_user.id, &db)
            .await
            .unwrap();

        assert!(response.is_empty());

        update_influence_info(first_user.id, second_user.id, Some("Some info"), &db)
            .await
            .unwrap();
        update_influence_level(first_user.id, second_user.id, 1, &db)
            .await
            .unwrap();

        let influence = get_all_influences_by_from_id(first_user.id, &db)
            .await
            .unwrap();

        assert_eq!(influence.len(), 1);
        assert_eq!(influence[0].to_id, second_user.id);
        assert_eq!(influence[0].influence_level, 1);
        assert_eq!(influence[0].info, Some("Some info".to_string()));
    }
}
