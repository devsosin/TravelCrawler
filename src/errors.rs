use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    ParsingError(#[from] serde_json::Error),

    #[error("Authentication failed: {0}")]
    AuthError(#[from] AuthError),

    #[error("Validation error: {0}")]
    ValidationError(&'static str),

    #[error("Internal error: {0}")]
    InternalError(&'static str),
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid username, password")]
    InvalidLoginInfo,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Max retry attempts exceeded")]
    MaxRetryExceeded, // retry n회 시도
}
