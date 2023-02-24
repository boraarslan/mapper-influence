use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_macros::FromRequest;
use mi_db::auth::AuthError;
use mi_db::influence::InfluenceError;
use mi_db::user::UserError;
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(pub T);

#[derive(Debug)]
pub struct AppError(Kind);

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn cookie_error() -> Self {
        AppError(Kind::Cookie)
    }
}

#[derive(Debug)]
pub enum Kind {
    Reqwest,
    Auth { msg: String },
    User(UserError),
    Influence(InfluenceError),
    Validation { msg: String },
    Cookie,
    Json { code: StatusCode, msg: String },
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self.0 {
            Kind::Reqwest => (
                StatusCode::SERVICE_UNAVAILABLE,
                "API failed to make the HTTP request.",
            )
                .into_response(),
            Kind::Auth { msg } => {
                (StatusCode::UNAUTHORIZED, format!("Auth is failed: {}", msg)).into_response()
            }
            Kind::User(user_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error happened: {}", user_error),
            )
                .into_response(),
            Kind::Cookie => {
                (StatusCode::UNAUTHORIZED, "Token is invalid".to_string()).into_response()
            }
            Kind::Json { code, msg } => (code, msg).into_response(),
            Kind::Influence(influence_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, influence_err.to_string()).into_response()
            }
            Kind::Validation { msg } => (StatusCode::BAD_REQUEST, msg).into_response(),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(_: reqwest::Error) -> Self {
        AppError(Kind::Reqwest)
    }
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::ConnectionTimedOut => AppError(Kind::Auth {
                msg: "Database connection timed out.".to_string(),
            }),
            AuthError::RedisError(_) => AppError(Kind::Auth {
                msg: "Redis connection returned an error.".to_string(),
            }),
            AuthError::ValueNotFound => AppError(Kind::Auth {
                msg: "Value is not found.".to_string(),
            }),
        }
    }
}

impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        AppError(Kind::User(err))
    }
}

impl From<InfluenceError> for AppError {
    fn from(err: InfluenceError) -> Self {
        AppError(Kind::Influence(err))
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        let code = match rejection {
            JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
            JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        AppError(Kind::Json {
            code,
            msg: rejection.to_string(),
        })
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let error = errors
            .field_errors()
            .into_values()
            .next()
            .expect("Errors can't be empty")
            .first()
            .expect("Errors can't be empty.");

        AppError(Kind::Validation {
            msg: error
                .message
                .as_ref()
                .map(|cow| cow.to_string())
                .unwrap_or("Unknown validation error.".to_string()),
        })
    }
}

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
