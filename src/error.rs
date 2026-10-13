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
        retry_after: Option<String>,
        body: String,
    },
    #[error("failed to decode server-sent event: {0}")]
    Sse(String),
    #[error("server-sent event stream ended before its terminal marker")]
    UnexpectedStreamEnd,
    #[error("Responses API failed: {body}")]
    ResponseFailed {
        body: serde_json::Value,
        request_id: Option<String>,
    },
    #[error("HTTP request timed out before a response was received")]
    RequestTimeout,
    #[error("server-sent event stream was idle beyond its configured timeout")]
    StreamIdleTimeout,
    #[error("WebSocket transport failed: {0}")]
    WebSocket(String),
    #[error("WebSocket handshake failed: {0}")]
    WebSocketHandshake(String),
    #[error("WebSocket server returned an error event: {body}")]
    WebSocketServerEvent {
        body: serde_json::Value,
        request_id: Option<String>,
    },
    #[error("WebSocket connection closed before the stream completed")]
    WebSocketClosed,
    #[error("invalid base URL: {0}")]
    InvalidBaseUrl(String),
    #[error("an API key is required")]
    MissingApiKey,
    #[error("multipart filename or field name contains a line break")]
    InvalidMultipartHeader,
    #[error("image edit requires between 1 and 16 image files")]
    InvalidImageFileCount,
    #[error("request extension field conflicts with a typed request field: {0}")]
    ExtensionFieldConflict(String),
    #[error("unsupported query parameter value for: {0}")]
    InvalidQueryParameter(String),
    #[error("server returned an empty or repeated pagination cursor")]
    InvalidPaginationCursor,
    #[error("header is managed by the SDK and cannot be overridden: {0}")]
    ReservedHeader(String),
}

impl Error {
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::HttpStatus { request_id, .. }
            | Self::ResponseFailed { request_id, .. }
            | Self::WebSocketServerEvent { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }
}
