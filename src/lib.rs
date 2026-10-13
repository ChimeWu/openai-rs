//! An independent asynchronous Rust SDK for the OpenAI Responses API.

mod client;
mod error;
mod event;
mod generated;
mod stream;

pub mod types {
    pub use crate::generated::responses::*;
}

pub use client::{
    ApiResponse, ClientBuilder, CreateOptions, OpenAIClient, RequestOptions, ResponsesApi,
};
pub use error::Error;
pub use event::ResponseStreamEvent;
pub use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
pub use stream::ResponseStream;
pub use types::{CreateResponse, Response};
