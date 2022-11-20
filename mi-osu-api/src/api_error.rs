use thiserror::Error;

// visit https://developer.mozilla.org/en-US/docs/Web/HTTP/Status for more info

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum Redirect {
    #[error("The source URL has been changed permanently. New URL: {0}")]
    MovedPermanently(String),
    #[error("The URL has been changed temporarily.")]
    Found,
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ClientError {
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
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ServerError {
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
}
