mod authentication;

use reqwest::{Response, StatusCode};
use thiserror::Error;

// Used in API call methods to determine HTTP errors in case the request goes through but returns with error.
fn evaluate_request(response: Response) -> Result<Response, APIError> {
    if !response.status().is_success() {
        println!("{}", response.status().as_u16());
        Err(APIError::HTTP(HTTPError::from(response.status())))
    } else {
        Ok(response)
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum APIError {
    #[error("The API request returned a response with HTTP error code.")]
    HTTP(HTTPError),
    #[error("There was an error before, during or after sending the request that is not related to HTTP errors.")]
    Request(RequestError),
    #[error("An unknown error has been occured.")]
    Unknown,
}

impl From<reqwest::Error> for APIError {
    fn from(reqwest_error: reqwest::Error) -> Self {
        if reqwest_error.is_status() {
            return Self::HTTP(HTTPError::from(reqwest_error.status().unwrap()));
        }
        if reqwest_error.is_decode() {
            return Self::Request(RequestError::Decode);
        }
        if reqwest_error.is_builder() {
            return Self::Request(RequestError::Builder);
        }
        if reqwest_error.is_redirect() {
            return Self::Request(RequestError::Redirect);
        }
        if reqwest_error.is_timeout() {
            return Self::Request(RequestError::Timeout);
        }
        if reqwest_error.is_request() {
            return Self::Request(RequestError::Request);
        }
        if reqwest_error.is_connect() {
            return Self::Request(RequestError::Connect);
        }
        if reqwest_error.is_body() {
            return Self::Request(RequestError::Body);
        }
        Self::Unknown
    }
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Error while building the request.")]
    Builder,
    #[error("Error while redirecting the request.")]
    Redirect,
    #[error("Request timed out.")]
    Timeout,
    #[error("Error during request process.")]
    Request,
    #[error("Error while connecting.")]
    Connect,
    #[error("Error while encoding the request body.")]
    Body,
    #[error("Error while decoding the response.")]
    Decode,
}

// visit https://developer.mozilla.org/en-US/docs/Web/HTTP/Status for more info
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum HTTPError {
    // Redirect Errors
    #[error("The source URL has been changed permanently.")]
    MovedPermanently,
    #[error("The URL has been changed temporarily.")]
    Found,
    // Client Errors
    #[error("The server will not process due to bad request.")]
    BadRequest,
    #[error("The client is not authorized or has wrong credentials in request.")]
    Unauthorized,
    #[error("The client has no access to this resource.")]
    Forbidden,
    #[error("The server can not find the requested resource.")]
    NotFound,
    #[error("The used method is not allowed on this resource.")]
    MethodNotAllowed,
    #[error("The requested content has been permanently deleted from the server.")]
    Gone,
    // Server Errors
    #[error("The server has encountered a situation it does not know how to handle.")]
    InternalServerError,
    #[error("The request is not supported by the server.")]
    NotImplemented,
    #[error("The gateway server has got an invalid response.")]
    BadGateway,
    #[error("The server is unavailable to handle requests.")]
    ServiceUnavailable,
    #[error("The gateway server had timeout while waiting for response.")]
    GatewayTimeout,
    #[error("The response has an unexpected error code {0}:{1}")]
    Other(u16, String),
}

impl From<StatusCode> for HTTPError {
    fn from(status_code: StatusCode) -> Self {
        match status_code.as_u16() {
            // Redirect errors
            301 => Self::MovedPermanently,
            302 => Self::Found,
            // Client errors
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            410 => Self::Gone,
            // Server errors
            500 => Self::InternalServerError,
            501 => Self::NotImplemented,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            // Other
            _ => Self::Other(
                status_code.as_u16(),
                status_code.canonical_reason().unwrap_or("").to_string(),
            ),
        }
    }
}
