use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP transport failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON encoding or decoding failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP request failed with status {status}: {body}")]
    HttpStatus {
        status: StatusCode,
        request_id: Option<String>,
        body: String,
    },
    #[error("failed to decode server-sent event: {0}")]
    Sse(String),
    #[error("Responses stream ended before a terminal event")]
    UnexpectedStreamEnd,
    #[error("invalid base URL: {0}")]
    InvalidBaseUrl(String),
    #[error("an API key is required")]
    MissingApiKey,
    #[error("request extension field conflicts with a typed request field: {0}")]
    ExtensionFieldConflict(String),
    #[error("header is managed by the SDK and cannot be overridden: {0}")]
    ReservedHeader(String),
}

impl Error {
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::HttpStatus { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }
}
