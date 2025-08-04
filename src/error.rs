use thiserror::Error;

/// Error type for Hermes SDK operations
#[derive(Error, Debug)]
pub enum HermesError {
    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("API request failed: {0}")]
    ApiRequest(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Invalid configuration: {0}")]
    Configuration(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for Hermes SDK operations
pub type HermesResult<T> = Result<T, HermesError>;

impl From<anyhow::Error> for HermesError {
    fn from(err: anyhow::Error) -> Self {
        HermesError::Unknown(err.to_string())
    }
}