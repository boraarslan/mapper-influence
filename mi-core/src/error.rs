use std::error::Error;

use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use tracing::{error, warn};
use validator::ValidationErrors;

pub const INTERNAL_DB_ERROR_MESSAGE: &str = "An internal database error occurred";
pub const INTERNAL_SERVER_ERROR_MESSAGE: &str = "An internal server error occurred";

/// Main error trait that needs to be implemented for all AppErrors.
///
/// This trait is used to convert errors into responses and log them.
/// It also provides a way to customize the error message that is sent to the user.
pub trait AppErrorExt: Error + Send + Sync + 'static {
    /// Message that the user sees on the response. Should be short and non-technical.
    fn user_message(&self) -> String;

    /// Type of the error. Used to categorize errors.
    fn error_type(&self) -> ErrorType;

    /// Log the error to the console.
    fn log_error(&self);

    /// Message that is logged to the database.
    fn error_message(&self) -> String {
        self.to_string()
    }

    /// Whether the error should be saved to the database.
    fn should_save(&self) -> bool {
        false
    }

    /// Error data that is saved to the database.
    fn error_data(&self) -> Option<serde_json::Value> {
        None
    }

    /// Convert the error into a [`DbErrorObject`].
    fn as_db_error_object(&self) -> DbErrorObject {
        DbErrorObject::new(
            self.error_type(),
            self.error_message(),
            self.error_data().unwrap_or_default(),
        )
    }

    /// Convert the error into a [`axum::response::Response`].
    fn as_response(&self) -> axum::response::Response {
        (self.error_type().http_status(), self.user_message()).into_response()
    }
}

/// Error to record on the database.
///
/// This is generated when an error occurs and needs to be recorded on the database.
pub struct DbErrorObject {
    pub error_type: ErrorType,
    pub message: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum ErrorType {
    DeserializeError = 100,
    DatabaseError = 200,
    HttpClientError = 300,
    OsuApiError = 400,
    OsuApiScopeError = 401,
    AuthorizatonError = 500,
    BadRequestData = 600,
    BadRequestSyntax = 601,
    UnsupportedType = 602,
    UnableToProcess = 603,
    DataNotFound = 900,
    DuplicateEntry = 901,
}

impl ErrorType {
    pub fn category(&self) -> &'static str {
        match self {
            ErrorType::DeserializeError => "InternalParsing",
            ErrorType::DatabaseError => "Database",
            ErrorType::AuthorizatonError => "Authorization",
            ErrorType::DataNotFound => "DataNotFound",
            ErrorType::DuplicateEntry => "DuplicateEnrty",
            ErrorType::BadRequestData => "BadRequest",
            ErrorType::BadRequestSyntax => "BadRequest",
            ErrorType::UnsupportedType => "BadRequest",
            ErrorType::UnableToProcess => "InternalProcessing",
            ErrorType::OsuApiError => "OsuApi",
            ErrorType::HttpClientError => "HttpClient",
            ErrorType::OsuApiScopeError => "OsuApi",
        }
    }

    pub fn http_status(&self) -> StatusCode {
        match self {
            ErrorType::DeserializeError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::AuthorizatonError => StatusCode::UNAUTHORIZED,
            ErrorType::DataNotFound => StatusCode::NOT_FOUND,
            ErrorType::DuplicateEntry => StatusCode::CONFLICT,
            ErrorType::BadRequestData => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorType::BadRequestSyntax => StatusCode::BAD_REQUEST,
            ErrorType::UnsupportedType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ErrorType::UnableToProcess => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::OsuApiError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::HttpClientError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::OsuApiScopeError => StatusCode::UNAUTHORIZED,
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
            "INSERT INTO error_table (error_message, error_data, error_code, error_category) \
             VALUES ($1, $2, $3, $4) RETURNING id as \"id: i32\"",
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

// Error Implementations

impl AppErrorExt for JsonRejection {
    fn user_message(&self) -> String {
        match self {
            JsonRejection::JsonDataError(_) => "Invalid JSON data".to_string(),
            JsonRejection::JsonSyntaxError(_) => "Invalid JSON syntax".to_string(),
            JsonRejection::MissingJsonContentType(_) => "Missing JSON content type".to_string(),
            JsonRejection::BytesRejection(_) => INTERNAL_SERVER_ERROR_MESSAGE.to_string(),
            _ => INTERNAL_SERVER_ERROR_MESSAGE.to_string(),
        }
    }

    fn error_type(&self) -> ErrorType {
        match self {
            JsonRejection::JsonDataError(_) => ErrorType::BadRequestData,
            JsonRejection::JsonSyntaxError(_) => ErrorType::BadRequestSyntax,
            JsonRejection::MissingJsonContentType(_) => ErrorType::UnsupportedType,
            JsonRejection::BytesRejection(_) => ErrorType::UnableToProcess,
            _ => ErrorType::UnableToProcess,
        }
    }

    fn log_error(&self) {
        error!("Unable to parse request body: {:?}", self.to_string())
    }
}

impl AppErrorExt for ValidationErrors {
    fn user_message(&self) -> String {
        "Invalid data has been returned".to_string()
    }

    fn error_type(&self) -> ErrorType {
        ErrorType::BadRequestData
    }

    fn log_error(&self) {
        let message = self.error_message();
        warn!("Invalid data has been provided: {}", message)
    }

    fn error_message(&self) -> String {
        let error = self
            .field_errors()
            .into_values()
            .next()
            .expect("Errors can't be empty")
            .first()
            .expect("Errors can't be empty.");

        error
            .message
            .as_ref()
            .map(|cow| cow.to_string())
            .unwrap_or("Unknown validation error.".to_string())
    }
}
