use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::APIError;
use crate::evaluate_request;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequestBody {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
    pub redirect_uri: String,
    pub code: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequestBuilder {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl AuthRequestBuilder {
    #[allow(dead_code)]
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    #[allow(dead_code)]
    pub fn request_body(&self, code: String) -> AuthRequestBody {
        AuthRequestBody {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            redirect_uri: self.redirect_uri.clone(),
            grant_type: "authorization_code".to_string(),
            code: Some(code),
            refresh_token: None,
        }
    }

    #[allow(dead_code)]
    pub fn refresh_body(&self, refresh_token: String) -> AuthRequestBody {
        AuthRequestBody {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            redirect_uri: self.redirect_uri.clone(),
            grant_type: "refresh_token".to_string(),
            code: None,
            refresh_token: Some(refresh_token),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponseBody {
    pub token_type: String,
    pub expires_in: u32,
    pub access_token: String,
    pub refresh_token: String,
}

#[allow(dead_code)]
async fn request_token(
    client: &Client,
    body: AuthRequestBody,
) -> Result<AuthResponseBody, APIError> {
    let response_result = client
        .post("https://osu.ppy.sh/oauth/token")
        .json(&body)
        .send()
        .await?;
    let response_result = evaluate_request(response_result)?;
    let response_body = response_result.json::<AuthResponseBody>().await?;
    Ok(response_body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test() {
        dotenv::dotenv().unwrap();

        let client = reqwest::Client::new();

        let auth_request_builder = AuthRequestBuilder::new(
            std::env::var("CLIENT_ID").unwrap(),
            std::env::var("CLIENT_SECRET").unwrap(),
            std::env::var("REDIRECT_URI").unwrap(),
        );

        // get fresh code from https://mapper-influences.vercel.app/oauth
        // It's single use. Without fresh code, it should panic at unwraps.
        let code = "code".to_string();
        let body = auth_request_builder.request_body(code);
        let first_response = dbg!(request_token(&client, body).await.unwrap());

        let body = auth_request_builder.refresh_body(first_response.refresh_token);
        let _ = dbg!(request_token(&client, body).await.unwrap());
    }
}
