use thiserror::Error;

#[derive(Error, Debug)]
pub enum AiError {
    #[error("API Key missing for provider: {0}")]
    MissingApiKey(String),

    #[error("Unsupported provider: {0}")]
    UnsupportedProvider(String),

    #[error("Completion error: {0}")]
    CompletionError(String),

    #[error("Stream error: {0}")]
    StreamError(String),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Rig error: {0}")]
    RigError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

pub type Result<T> = std::result::Result<T, AiError>;
