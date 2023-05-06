use serde::{Serialize, Deserialize};
use sqlx::{PgPool};

struct DbErrorObject {
    pub error_type: ErrorType,
    pub message: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum ErrorType {
    SerializeError = 100,
}

impl ErrorType {
    pub fn category(&self) -> &'static str {
        match self {
            ErrorType::SerializeError => "InternalParsing",
        }
    }
}

impl DbErrorObject {
    pub fn new(error_type: ErrorType, message: String, data: impl Into<serde_json::Value>) -> Self {
        Self {
            error_type,
            message,
            data: data.into(),
        }
    }

    pub async fn insert_to_db(&self, db: &PgPool) -> Result<i32, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO error_table (error_message, error_data, error_code, error_category) VALUES ($1, $2, $3, $4) RETURNING id as \"id: i32\"",
            self.message,
            self.data,
            self.error_type as i32,
            self.error_type.category(),
        )
        .fetch_one(db)
        .await?;

        Ok(result.id)
    }
}
