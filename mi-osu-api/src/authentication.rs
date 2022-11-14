//implement authentication endpoint with example
use dotenv;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenBody {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: String,
}

impl TokenBody {
    pub fn new(
        client_id: String,
        client_secret: String,
        code: String,
        grant_type: String,
        redirect_uri: String,
    ) -> Self {
        TokenBody {
            client_id,
            client_secret,
            code,
            grant_type,
            redirect_uri,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    token_type: String,
    expires_in: u32,
    access_token: String,
    refresh_token: String,
}

//Basic Example Error
#[derive(Debug, Error)]
enum AuthenticationError {
    #[error("Error during the request with status code {0}: {1}")]
    StatusError(u16, String),
    #[error("Error during serializing the body.")]
    SerializationError,
    #[error("Error during deserializing the response.")]
    DeserializationError,
    #[error("Error during sending the response.")]
    RequestError,
}

impl AuthenticationError {
    pub fn error_from_status(status_code: StatusCode) -> AuthenticationError {
        AuthenticationError::StatusError(
            status_code.as_u16(),
            status_code.canonical_reason().unwrap_or("").to_string(),
        )
    }
}

//example request function
async fn request_token(client: Client, code: String) -> Result<TokenResponse, AuthenticationError> {
    dotenv::dotenv();

    let body = TokenBody::new(
        std::env::var("CLIENT_ID").unwrap(),
        std::env::var("CLIENT_SECRET").unwrap(),
        code,
        "authorization_code".to_string(),
        "https://mapper-influences.vercel.app/oauth".to_string(),
    );

    let response_result = client
        .post("https://osu.ppy.sh/oauth/token")
        .json(&body)
        .send()
        .await;

    match response_result {
        Ok(response) => {
            let response_status = response.status();
            if !response_status.is_success() {
                return Err(AuthenticationError::error_from_status(response_status));
            }
            let response_body = response.json::<TokenResponse>().await;
            response_body.map_err(|_| AuthenticationError::DeserializationError)
        }
        Err(error) => {
            if let Some(status) = error.status() {
                Err(AuthenticationError::error_from_status(status))
            } else {
                Err(AuthenticationError::RequestError)
            }
        }
    }
}
