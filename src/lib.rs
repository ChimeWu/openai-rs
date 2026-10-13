//! An independent asynchronous Rust SDK for the OpenAI API.

mod audio_stream;
mod chat_stream;
mod client;
mod error;
mod event;
mod generated;
mod image_stream;
mod stream;
#[cfg(feature = "websocket")]
mod websocket;

pub mod types {
    pub use openai_api_types::responses::*;
    pub use openai_api_types::*;
}

pub use audio_stream::{AudioContentStream, AudioEventPayload, AudioEventStream, AudioStreamEvent};
pub use chat_stream::{ChatCompletionStream, ChatCompletionStreamChunk};
#[cfg(feature = "multipart")]
pub use client::ContentProvenanceApi;
pub use client::{
    ApiResponse, AudioApi, AudioFile, AudioSpeechResponse, AudioTextResponse,
    AudioTranscriptionResponse, AudioTranslationResponse, BatchesApi, ChatCompletionsApi,
    ClientBuilder, ContainersApi, ConversationsApi, CreateOptions, EmbeddingsApi,
    FileContentStream, FilesApi, FineTuningApi, ImageFile, ImagesApi, LiveApi, ModelsApi,
    ModerationsApi, OpenAIClient, RealtimeApi, RealtimeCallAnswer, RequestOptions, ResponsesApi,
    RetryPolicy, UploadsApi, VectorStoresApi, WebhooksApi,
};
pub use error::Error;
pub use event::ResponseStreamEvent;
pub use image_stream::{ImageEventPayload, ImageEventStream, ImageStreamEvent};
pub use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
pub use stream::ResponseStream;
pub use types::{CreateResponse, Response};
#[cfg(feature = "websocket")]
pub use websocket::{
    ResponseWebSocketEvent, ResponseWebSocketStream, ResponsesWebSocket, WebSocketConnection,
    WebSocketMessage,
};
