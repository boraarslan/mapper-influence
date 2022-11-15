use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequestBody {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub grant_type: String,
    pub redirect_uri: String,
}

impl AuthRequestBody {
    #[allow(dead_code)]
    pub fn new(client_id: String, client_secret: String, code: String) -> Self {
        AuthRequestBody {
            client_id,
            client_secret,
            code,
            grant_type: "authorization_code".to_string(),
            redirect_uri: "https://mapper-influences.vercel.app/oauth".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponseBody {
    token_type: String,
    expires_in: u32,
    access_token: String,
    refresh_token: String,
}
#[allow(dead_code)]
#[derive(Debug, Error)]
enum AuthenticationError {
    #[error("Error during the request with status code {0}: {1}")]
    Status(u16, String),
    #[error("Error during serializing the body.")]
    Serialization,
    #[error("Error during deserializing the response.")]
    Deserialization,
    #[error("Error during sending the response.")]
    Request,
}

impl AuthenticationError {
    #[allow(dead_code)]
    pub fn error_from_status(status_code: StatusCode) -> AuthenticationError {
        AuthenticationError::Status(
            status_code.as_u16(),
            status_code.canonical_reason().unwrap_or("").to_string(),
        )
    }
}

#[allow(dead_code)]
async fn request_token(
    client: &Client,
    body: AuthRequestBody,
) -> Result<AuthResponseBody, AuthenticationError> {
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
            let response_body = response.json::<AuthResponseBody>().await;
            response_body.map_err(|_| AuthenticationError::Deserialization)
        }
        Err(error) => {
            if let Some(status) = error.status() {
                Err(AuthenticationError::error_from_status(status))
            } else {
                Err(AuthenticationError::Request)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test() {
        dotenv::dotenv().unwrap();

        let body = AuthRequestBody::new(
            std::env::var("CLIENT_ID").unwrap(),
            std::env::var("CLIENT_SECRET").unwrap(),
            "code".to_string(),
        );
        let client = reqwest::Client::new();
        request_token(&client, body).await.unwrap();
    }
}
