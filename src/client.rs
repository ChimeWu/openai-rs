use crate::AudioContentStream;
use crate::AudioEventStream;
use crate::ChatCompletionStream;
use crate::Error;
use crate::ImageEventStream;
use crate::ResponseStream;
use crate::image_stream::ImageEventKind;
use crate::types;
use futures_core::Stream;
use futures_util::StreamExt;
use reqwest::header::{
    ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, RETRY_AFTER,
};
use reqwest::{Method, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Clone)]
pub struct OpenAIClient {
    pub(crate) http: reqwest::Client,
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
    #[cfg(feature = "websocket")]
    pub(crate) default_headers: HeaderMap,
    #[cfg(feature = "websocket")]
    pub(crate) connect_timeout: Option<Duration>,
    pub(crate) request_timeout: Option<Duration>,
    pub(crate) stream_idle_timeout: Option<Duration>,
    pub(crate) retry_policy: Option<RetryPolicy>,
}

impl OpenAIClient {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Returns the Responses API resource bound to this client.
    pub fn responses(&self) -> ResponsesApi<'_> {
        ResponsesApi { client: self }
    }

    /// Returns the Models API resource bound to this client.
    pub fn models(&self) -> ModelsApi<'_> {
        ModelsApi { client: self }
    }

    /// Returns the Embeddings API resource bound to this client.
    pub fn embeddings(&self) -> EmbeddingsApi<'_> {
        EmbeddingsApi { client: self }
    }

    /// Returns the Moderations API resource bound to this client.
    pub fn moderations(&self) -> ModerationsApi<'_> {
        ModerationsApi { client: self }
    }

    /// Returns the Audio API resource bound to this client.
    pub fn audio(&self) -> AudioApi<'_> {
        AudioApi { client: self }
    }

    /// Returns the Files API resource bound to this client.
    pub fn files(&self) -> FilesApi<'_> {
        FilesApi { client: self }
    }

    /// Returns the Content Provenance API resource bound to this client.
    #[cfg(feature = "multipart")]
    pub fn content_provenance(&self) -> ContentProvenanceApi<'_> {
        ContentProvenanceApi { client: self }
    }

    /// Returns the Images API resource bound to this client.
    pub fn images(&self) -> ImagesApi<'_> {
        ImagesApi { client: self }
    }

    /// Returns the Conversations API resource bound to this client.
    pub fn conversations(&self) -> ConversationsApi<'_> {
        ConversationsApi { client: self }
    }

    /// Returns the Containers API resource bound to this client.
    pub fn containers(&self) -> ContainersApi<'_> {
        ContainersApi { client: self }
    }

    /// Returns the Batches API resource bound to this client.
    pub fn batches(&self) -> BatchesApi<'_> {
        BatchesApi { client: self }
    }

    /// Returns the Webhooks API resource bound to this client.
    pub fn webhooks(&self) -> WebhooksApi<'_> {
        WebhooksApi { client: self }
    }

    /// Returns the multipart Uploads API resource bound to this client.
    pub fn uploads(&self) -> UploadsApi<'_> {
        UploadsApi { client: self }
    }

    /// Returns the Vector Stores API resource bound to this client.
    pub fn vector_stores(&self) -> VectorStoresApi<'_> {
        VectorStoresApi { client: self }
    }

    /// Returns the Fine-tuning API resource bound to this client.
    pub fn fine_tuning(&self) -> FineTuningApi<'_> {
        FineTuningApi { client: self }
    }

    /// Returns the Live API resource bound to this client.
    pub fn live(&self) -> LiveApi<'_> {
        LiveApi { client: self }
    }

    /// Returns the GA Realtime API resource bound to this client.
    pub fn realtime(&self) -> RealtimeApi<'_> {
        RealtimeApi { client: self }
    }

    /// Returns the Chat Completions API resource bound to this client.
    pub fn chat_completions(&self) -> ChatCompletionsApi<'_> {
        ChatCompletionsApi { client: self }
    }

    async fn send(
        &self,
        method: Method,
        endpoint: Url,
        body: Option<Value>,
        accept: &'static str,
        headers: &HeaderMap,
    ) -> Result<reqwest::Response, Error> {
        validate_request_headers(headers)?;
        let can_retry_method = retry_allowed(&method, headers);
        let max_retries = self
            .retry_policy
            .as_ref()
            .filter(|_| can_retry_method)
            .map_or(0, |policy| policy.max_retries);

        retry_request(
            max_retries,
            || async {
                let request = self
                    .http
                    .request(method.clone(), endpoint.clone())
                    .headers(headers.clone())
                    .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
                    .header(CONTENT_TYPE, "application/json")
                    .header(ACCEPT, accept);
                let request = if let Some(body) = body.clone() {
                    request.json(&body)
                } else {
                    request
                };
                let send = request.send();
                if let Some(timeout) = self.request_timeout {
                    match tokio::time::timeout(timeout, send).await {
                        Ok(response) => response.map_err(Error::Http),
                        Err(_) => Err(Error::RequestTimeout),
                    }
                } else {
                    send.await.map_err(Error::Http)
                }
            },
            |result| match result {
                Err(Error::Http(error)) if error.is_connect() => RetryDecision::Retry(None),
                Ok(response) if is_retryable_status(response.status()) => {
                    RetryDecision::Retry(retry_after_delay(response.headers()))
                }
                _ => RetryDecision::Stop,
            },
            |retry_number, retry_after| self.retry_delay(retry_number, retry_after),
        )
        .await
    }

    #[cfg(feature = "multipart")]
    async fn send_multipart(
        &self,
        endpoint: Url,
        headers: &HeaderMap,
        filename: String,
        contents: bytes::Bytes,
        text_fields: Vec<(String, String)>,
    ) -> Result<reqwest::Response, Error> {
        self.send_multipart_part(
            endpoint,
            headers,
            "file",
            Some(filename),
            contents,
            text_fields,
            ACCEPT_JSON,
        )
        .await
    }

    #[cfg(feature = "multipart")]
    #[allow(clippy::too_many_arguments)]
    async fn send_multipart_part(
        &self,
        endpoint: Url,
        headers: &HeaderMap,
        field_name: &str,
        filename: Option<String>,
        contents: bytes::Bytes,
        text_fields: Vec<(String, String)>,
        accept: &'static str,
    ) -> Result<reqwest::Response, Error> {
        self.send_multipart_files(
            endpoint,
            headers,
            vec![MultipartFilePart {
                field_name: field_name.to_owned(),
                filename,
                mime_type: "application/octet-stream".to_owned(),
                contents,
            }],
            text_fields,
            accept,
        )
        .await
    }

    #[cfg(feature = "multipart")]
    async fn send_multipart_files(
        &self,
        endpoint: Url,
        headers: &HeaderMap,
        file_parts: Vec<MultipartFilePart>,
        text_fields: Vec<(String, String)>,
        accept: &'static str,
    ) -> Result<reqwest::Response, Error> {
        validate_request_headers(headers)?;
        let (content_type, body) = multipart_body_files(file_parts, text_fields)?;
        let retries = self
            .retry_policy
            .as_ref()
            .filter(|_| retry_allowed(&Method::POST, headers))
            .map_or(0, |policy| policy.max_retries);

        retry_request(
            retries,
            || async {
                let send = self
                    .http
                    .post(endpoint.clone())
                    .headers(headers.clone())
                    .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
                    .header(ACCEPT, accept)
                    .header(CONTENT_TYPE, content_type.clone())
                    .body(body.clone())
                    .send();
                if let Some(timeout) = self.request_timeout {
                    match tokio::time::timeout(timeout, send).await {
                        Ok(response) => response.map_err(Error::Http),
                        Err(_) => Err(Error::RequestTimeout),
                    }
                } else {
                    send.await.map_err(Error::Http)
                }
            },
            |result| match result {
                Err(Error::Http(error)) if error.is_connect() => RetryDecision::Retry(None),
                Ok(response) if is_retryable_status(response.status()) => {
                    RetryDecision::Retry(retry_after_delay(response.headers()))
                }
                _ => RetryDecision::Stop,
            },
            |retry_number, retry_after| self.retry_delay(retry_number, retry_after),
        )
        .await
    }

    #[cfg(feature = "multipart")]
    #[allow(clippy::too_many_arguments)]
    async fn send_multipart_stream<S>(
        &self,
        endpoint: Url,
        headers: &HeaderMap,
        field_name: &str,
        filename: Option<&str>,
        mime_type: &str,
        contents: S,
        text_fields: Vec<(String, String)>,
        accept: &'static str,
    ) -> Result<reqwest::Response, Error>
    where
        S: Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send + 'static,
    {
        validate_request_headers(headers)?;
        let (content_type, body) =
            multipart_stream_body(field_name, filename, mime_type, contents, text_fields)?;
        let send = self
            .http
            .post(endpoint)
            .headers(headers.clone())
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(ACCEPT, accept)
            .header(CONTENT_TYPE, content_type)
            .body(reqwest::Body::wrap_stream(body))
            .send();
        let response = if let Some(timeout) = self.request_timeout {
            tokio::time::timeout(timeout, send)
                .await
                .map_err(|_| Error::RequestTimeout)?
        } else {
            send.await
        }?;
        Ok(response)
    }

    fn retry_delay(&self, retry_number: u8, retry_after: Option<Duration>) -> Duration {
        let Some(policy) = &self.retry_policy else {
            return Duration::ZERO;
        };
        if let Some(delay) = retry_after {
            return delay.min(policy.max_delay);
        }
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as u64;
        jittered_backoff(policy, retry_number, seed)
    }

    #[cfg(feature = "websocket")]
    pub(crate) fn websocket_retry_count(&self) -> u8 {
        self.retry_policy
            .as_ref()
            .map_or(0, |policy| policy.max_retries)
    }

    #[cfg(feature = "websocket")]
    pub(crate) fn websocket_retry_delay(
        &self,
        retry_number: u8,
        retry_after: Option<Duration>,
    ) -> Duration {
        self.retry_delay(retry_number, retry_after)
    }

    async fn decode<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<ApiResponse<T>, Error> {
        if let Some(timeout) = self.request_timeout {
            return tokio::time::timeout(timeout, decode_response(response))
                .await
                .map_err(|_| Error::RequestTimeout)?;
        }
        decode_response(response).await
    }
}

/// Responses API methods for an [`OpenAIClient`].
pub struct ResponsesApi<'a> {
    client: &'a OpenAIClient,
}

impl ResponsesApi<'_> {
    /// Opens a Responses WebSocket with bounded routing by `stream_id`.
    #[cfg(feature = "websocket")]
    pub async fn connect_websocket(&self) -> Result<crate::ResponsesWebSocket, Error> {
        crate::websocket::ResponsesWebSocket::connect(self.client).await
    }

    /// Creates a response and preserves its original JSON body and request ID.
    pub async fn create(
        &self,
        request: &types::CreateResponse,
    ) -> Result<ApiResponse<types::Response>, Error> {
        self.create_with_options(request, CreateOptions::default())
            .await
    }

    /// Creates a response with per-request headers and extra top-level fields.
    pub async fn create_with_options(
        &self,
        request: &types::CreateResponse,
        options: CreateOptions,
    ) -> Result<ApiResponse<types::Response>, Error> {
        let mut request = request.clone();
        request.stream = Some(false);
        let body = request_body(&request, options.extensions)?;
        let response = self
            .client
            .send(
                Method::POST,
                responses_url(&self.client.base_url)?,
                Some(body),
                ACCEPT_JSON,
                &options.request.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Creates a streaming response, ending at a semantic terminal event.
    pub async fn create_stream(
        &self,
        request: &types::CreateResponse,
    ) -> Result<ResponseStream, Error> {
        self.create_stream_with_options(request, CreateOptions::default())
            .await
    }

    /// Creates a streaming response with per-request headers and extra fields.
    pub async fn create_stream_with_options(
        &self,
        request: &types::CreateResponse,
        options: CreateOptions,
    ) -> Result<ResponseStream, Error> {
        let mut request = request.clone();
        request.stream = Some(true);
        let body = request_body(&request, options.extensions)?;
        self.send_stream(
            Method::POST,
            responses_url(&self.client.base_url)?,
            Some(body),
            &options.request.headers,
        )
        .await
    }

    /// Retrieves a stored response as JSON.
    pub async fn retrieve(
        &self,
        response_id: &str,
        params: &types::RetrieveResponseParams,
    ) -> Result<ApiResponse<types::Response>, Error> {
        self.retrieve_with_options(response_id, params, RequestOptions::default())
            .await
    }

    /// Retrieves a stored response with per-request headers.
    pub async fn retrieve_with_options(
        &self,
        response_id: &str,
        params: &types::RetrieveResponseParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::Response>, Error> {
        let mut endpoint = response_url(&self.client.base_url, response_id, &[])?;
        append_query(&mut endpoint, params)?;
        let response = self
            .client
            .send(Method::GET, endpoint, None, ACCEPT_JSON, &options.headers)
            .await?;
        self.client.decode(response).await
    }

    /// Resumes a response event stream, optionally after a sequence number.
    pub async fn retrieve_stream(
        &self,
        response_id: &str,
        params: &types::RetrieveResponseStreamParams,
    ) -> Result<ResponseStream, Error> {
        self.retrieve_stream_with_options(response_id, params, RequestOptions::default())
            .await
    }

    /// Resumes a response event stream with per-request headers.
    pub async fn retrieve_stream_with_options(
        &self,
        response_id: &str,
        params: &types::RetrieveResponseStreamParams,
        options: RequestOptions,
    ) -> Result<ResponseStream, Error> {
        let endpoint = response_stream_url(&self.client.base_url, response_id, params)?;
        self.send_stream(Method::GET, endpoint, None, &options.headers)
            .await
    }

    /// Deletes a stored response.
    pub async fn delete(
        &self,
        response_id: &str,
    ) -> Result<ApiResponse<types::DeleteResponse>, Error> {
        self.delete_with_options(response_id, RequestOptions::default())
            .await
    }

    /// Deletes a stored response with per-request headers.
    pub async fn delete_with_options(
        &self,
        response_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::DeleteResponse>, Error> {
        let response = self
            .client
            .send(
                Method::DELETE,
                response_url(&self.client.base_url, response_id, &[])?,
                None,
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Cancels an in-progress response.
    pub async fn cancel(&self, response_id: &str) -> Result<ApiResponse<types::Response>, Error> {
        self.cancel_with_options(response_id, RequestOptions::default())
            .await
    }

    /// Cancels an in-progress response with per-request headers.
    pub async fn cancel_with_options(
        &self,
        response_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::Response>, Error> {
        let response = self
            .client
            .send(
                Method::POST,
                response_url(&self.client.base_url, response_id, &["cancel"])?,
                None,
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Lists one page of input items for a response.
    pub async fn list_input_items(
        &self,
        response_id: &str,
        params: &types::ListInputItemsParams,
    ) -> Result<ApiResponse<types::ResponseItemList>, Error> {
        self.list_input_items_with_options(response_id, params, RequestOptions::default())
            .await
    }

    /// Lists one page of input items with per-request headers.
    pub async fn list_input_items_with_options(
        &self,
        response_id: &str,
        params: &types::ListInputItemsParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::ResponseItemList>, Error> {
        let mut endpoint = response_url(&self.client.base_url, response_id, &["input_items"])?;
        append_query(&mut endpoint, params)?;
        let response = self
            .client
            .send(Method::GET, endpoint, None, ACCEPT_JSON, &options.headers)
            .await?;
        self.client.decode(response).await
    }

    /// Streams every page of input items, preserving each page response and request ID.
    pub fn list_input_items_pages<'a>(
        &'a self,
        response_id: &'a str,
        params: types::ListInputItemsParams,
    ) -> impl Stream<Item = Result<ApiResponse<types::ResponseItemList>, Error>> + Send + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_input_items(response_id, &params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone();
                yield page;
                let next = next_input_page_params(&params, has_more, &last_id)?;
                let Some(next) = next else { break };
                params = next;
            }
        }
    }

    /// Counts the input tokens for a Responses request.
    pub async fn count_input_tokens(
        &self,
        request: &types::TokenCountsBody,
    ) -> Result<ApiResponse<types::TokenCountsResource>, Error> {
        self.count_input_tokens_with_options(request, RequestOptions::default())
            .await
    }

    /// Counts input tokens with per-request headers.
    pub async fn count_input_tokens_with_options(
        &self,
        request: &types::TokenCountsBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::TokenCountsResource>, Error> {
        let response = self
            .client
            .send(
                Method::POST,
                responses_subresource_url(&self.client.base_url, "input_tokens")?,
                Some(serde_json::to_value(request)?),
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Compacts a conversation into a reduced context representation.
    pub async fn compact(
        &self,
        request: &types::CompactResponseMethodPublicBody,
    ) -> Result<ApiResponse<types::CompactResource>, Error> {
        self.compact_with_options(request, RequestOptions::default())
            .await
    }

    /// Compacts a conversation with per-request headers.
    pub async fn compact_with_options(
        &self,
        request: &types::CompactResponseMethodPublicBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::CompactResource>, Error> {
        let response = self
            .client
            .send(
                Method::POST,
                responses_subresource_url(&self.client.base_url, "compact")?,
                Some(serde_json::to_value(request)?),
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    async fn send_stream(
        &self,
        method: Method,
        endpoint: Url,
        body: Option<Value>,
        headers: &HeaderMap,
    ) -> Result<ResponseStream, Error> {
        let response = self
            .client
            .send(method, endpoint, body, ACCEPT_EVENTS, headers)
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        Ok(ResponseStream::new(
            response,
            self.client.stream_idle_timeout,
        ))
    }
}

/// Models API operations for an [`OpenAIClient`].
pub struct ModelsApi<'a> {
    client: &'a OpenAIClient,
}

impl ModelsApi<'_> {
    /// Lists models visible to the configured API key.
    pub async fn list(&self) -> Result<ApiResponse<types::models::ListModelsResponse>, Error> {
        self.list_with_options(RequestOptions::default()).await
    }

    /// Lists models with per-request headers.
    pub async fn list_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::models::ListModelsResponse>, Error> {
        let response = self
            .client
            .send(
                Method::GET,
                append_path_segments(&self.client.base_url, &["models"])?,
                None,
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Retrieves a model by its identifier.
    pub async fn retrieve(&self, model: &str) -> Result<ApiResponse<types::models::Model>, Error> {
        self.retrieve_with_options(model, RequestOptions::default())
            .await
    }

    /// Retrieves a model with per-request headers.
    pub async fn retrieve_with_options(
        &self,
        model: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::models::Model>, Error> {
        let response = self
            .client
            .send(
                Method::GET,
                append_path_segments(&self.client.base_url, &["models", model])?,
                None,
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Deletes a fine-tuned model.
    pub async fn delete(
        &self,
        model: &str,
    ) -> Result<ApiResponse<types::models::DeleteModelResponse>, Error> {
        self.delete_with_options(model, RequestOptions::default())
            .await
    }

    /// Deletes a fine-tuned model with per-request headers.
    pub async fn delete_with_options(
        &self,
        model: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::models::DeleteModelResponse>, Error> {
        let response = self
            .client
            .send(
                Method::DELETE,
                append_path_segments(&self.client.base_url, &["models", model])?,
                None,
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }
}

/// Embeddings API operations for an [`OpenAIClient`].
pub struct EmbeddingsApi<'a> {
    client: &'a OpenAIClient,
}

impl EmbeddingsApi<'_> {
    /// Creates embeddings for one or more inputs.
    pub async fn create(
        &self,
        request: &types::embeddings::CreateEmbeddingRequest,
    ) -> Result<ApiResponse<types::embeddings::CreateEmbeddingResponse>, Error> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    /// Creates embeddings with per-request headers.
    pub async fn create_with_options(
        &self,
        request: &types::embeddings::CreateEmbeddingRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::embeddings::CreateEmbeddingResponse>, Error> {
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(&self.client.base_url, &["embeddings"])?,
                Some(serde_json::to_value(request)?),
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }
}

/// Moderations API operations for an [`OpenAIClient`].
pub struct ModerationsApi<'a> {
    client: &'a OpenAIClient,
}

impl ModerationsApi<'_> {
    /// Classifies text or multimodal input for potentially harmful content.
    pub async fn create(
        &self,
        request: &types::moderations::CreateModerationRequest,
    ) -> Result<ApiResponse<types::moderations::CreateModerationResponse>, Error> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    /// Classifies input with per-request headers.
    pub async fn create_with_options(
        &self,
        request: &types::moderations::CreateModerationRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::moderations::CreateModerationResponse>, Error> {
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(&self.client.base_url, &["moderations"])?,
                Some(serde_json::to_value(request)?),
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }
}

/// Replayable audio input for multipart Audio API operations.
#[derive(Clone, Debug)]
pub struct AudioFile {
    pub filename: String,
    pub content_type: String,
    pub contents: bytes::Bytes,
}

/// Replayable image input for multipart Images API edits.
#[derive(Clone, Debug)]
pub struct ImageFile {
    pub filename: String,
    pub content_type: String,
    pub contents: bytes::Bytes,
}

impl ImageFile {
    pub fn new(
        filename: impl Into<String>,
        content_type: impl Into<String>,
        contents: bytes::Bytes,
    ) -> Self {
        Self {
            filename: filename.into(),
            content_type: content_type.into(),
            contents,
        }
    }
}

impl AudioFile {
    pub fn new(
        filename: impl Into<String>,
        content_type: impl Into<String>,
        contents: bytes::Bytes,
    ) -> Self {
        Self {
            filename: filename.into(),
            content_type: content_type.into(),
            contents,
        }
    }
}

/// Result of text-to-speech; either encoded audio bytes or typed SSE events.
pub enum AudioSpeechResponse {
    Audio(AudioContentStream),
    Events(AudioEventStream),
}

/// Result of a transcription request, selected by its response format.
pub enum AudioTranscriptionResponse {
    Json(ApiResponse<types::audio::CreateTranscriptionResponse>),
    Text(AudioTextResponse),
    Events(AudioEventStream),
}

/// Result of an audio translation request.
pub enum AudioTranslationResponse {
    Json(ApiResponse<types::audio::CreateTranslationResponse>),
    Text(AudioTextResponse),
}

/// Plain-text audio response paired with the originating request ID.
#[derive(Clone, Debug, PartialEq)]
pub struct AudioTextResponse {
    pub text: String,
    pub request_id: Option<String>,
}

/// Audio API operations for an [`OpenAIClient`].
pub struct AudioApi<'a> {
    client: &'a OpenAIClient,
}

impl AudioApi<'_> {
    /// Generates audio from text. `stream_format=sse` yields typed stream events.
    pub async fn create_speech(
        &self,
        request: &types::audio::CreateSpeechRequest,
    ) -> Result<AudioSpeechResponse, Error> {
        self.create_speech_with_options(request, RequestOptions::default())
            .await
    }

    /// Generates audio with per-request headers.
    pub async fn create_speech_with_options(
        &self,
        request: &types::audio::CreateSpeechRequest,
        options: RequestOptions,
    ) -> Result<AudioSpeechResponse, Error> {
        let events =
            request.stream_format == Some(types::audio::CreateSpeechRequestStreamFormat::Sse);
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(&self.client.base_url, &["audio", "speech"])?,
                Some(serde_json::to_value(request)?),
                if events { ACCEPT_EVENTS } else { ACCEPT_AUDIO },
                &options.headers,
            )
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        let request_id = request_id(response.headers());
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        if events {
            Ok(AudioSpeechResponse::Events(AudioEventStream::new(
                response,
                request_id,
                crate::audio_stream::AudioEventKind::Speech,
                self.client.stream_idle_timeout,
            )))
        } else {
            Ok(AudioSpeechResponse::Audio(AudioContentStream::new(
                response,
                request_id,
                content_type,
                self.client.stream_idle_timeout,
            )))
        }
    }

    /// Transcribes replayable audio bytes. The generated request's `file` field
    /// should match `audio.filename`; the supplied `AudioFile` provides its bytes
    /// and MIME type. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn transcribe(
        &self,
        request: &types::audio::CreateTranscriptionRequest,
        audio: AudioFile,
    ) -> Result<AudioTranscriptionResponse, Error> {
        self.transcribe_with_options(request, audio, RequestOptions::default())
            .await
    }

    /// Transcribes audio with per-request headers. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn transcribe_with_options(
        &self,
        request: &types::audio::CreateTranscriptionRequest,
        audio: AudioFile,
        options: RequestOptions,
    ) -> Result<AudioTranscriptionResponse, Error> {
        let streaming = request.stream.unwrap_or(false);
        let plain_text = matches!(
            request.response_format,
            Some(
                types::audio::AudioResponseFormat::Text
                    | types::audio::AudioResponseFormat::Srt
                    | types::audio::AudioResponseFormat::Vtt
            )
        );
        let accept = if streaming {
            ACCEPT_EVENTS
        } else if plain_text {
            ACCEPT_TEXT
        } else {
            ACCEPT_JSON
        };
        let response = self
            .client
            .send_multipart_files(
                append_path_segments(&self.client.base_url, &["audio", "transcriptions"])?,
                &options.headers,
                vec![audio.multipart("file")],
                multipart_fields(request, &["file"])?,
                accept,
            )
            .await?;
        if streaming {
            if !response.status().is_success() {
                return Err(read_http_error(response).await);
            }
            let request_id = request_id(response.headers());
            Ok(AudioTranscriptionResponse::Events(AudioEventStream::new(
                response,
                request_id,
                crate::audio_stream::AudioEventKind::Transcription,
                self.client.stream_idle_timeout,
            )))
        } else if plain_text {
            Ok(AudioTranscriptionResponse::Text(
                read_audio_text(response).await?,
            ))
        } else {
            Ok(AudioTranscriptionResponse::Json(
                self.client.decode(response).await?,
            ))
        }
    }

    /// Translates replayable audio bytes. The generated request's `file` field
    /// should match `audio.filename`; the supplied `AudioFile` provides its bytes
    /// and MIME type. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn translate(
        &self,
        request: &types::audio::CreateTranslationRequest,
        audio: AudioFile,
    ) -> Result<AudioTranslationResponse, Error> {
        self.translate_with_options(request, audio, RequestOptions::default())
            .await
    }

    /// Translates audio with per-request headers. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn translate_with_options(
        &self,
        request: &types::audio::CreateTranslationRequest,
        audio: AudioFile,
        options: RequestOptions,
    ) -> Result<AudioTranslationResponse, Error> {
        let plain_text = matches!(
            request.response_format,
            Some(
                types::audio::CreateTranslationRequestResponseFormat::Text
                    | types::audio::CreateTranslationRequestResponseFormat::Srt
                    | types::audio::CreateTranslationRequestResponseFormat::Vtt
            )
        );
        let response = self
            .client
            .send_multipart_files(
                append_path_segments(&self.client.base_url, &["audio", "translations"])?,
                &options.headers,
                vec![audio.multipart("file")],
                multipart_fields(request, &["file"])?,
                if plain_text { ACCEPT_TEXT } else { ACCEPT_JSON },
            )
            .await?;
        if plain_text {
            Ok(AudioTranslationResponse::Text(
                read_audio_text(response).await?,
            ))
        } else {
            Ok(AudioTranslationResponse::Json(
                self.client.decode(response).await?,
            ))
        }
    }

    /// Lists voice consent recordings.
    pub async fn list_voice_consents(
        &self,
        params: &types::audio::ListVoiceConsentsParams,
    ) -> Result<ApiResponse<types::audio::VoiceConsentListResource>, Error> {
        self.list_voice_consents_with_options(params, RequestOptions::default())
            .await
    }

    /// Lists voice consent recordings with per-request headers.
    pub async fn list_voice_consents_with_options(
        &self,
        params: &types::audio::ListVoiceConsentsParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::audio::VoiceConsentListResource>, Error> {
        let mut endpoint =
            append_path_segments(&self.client.base_url, &["audio", "voice_consents"])?;
        append_query(&mut endpoint, params)?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Streams voice consent pages, retaining each complete response and request ID.
    pub fn list_voice_consents_pages<'a>(
        &'a self,
        params: types::audio::ListVoiceConsentsParams,
    ) -> impl Stream<Item = Result<ApiResponse<types::audio::VoiceConsentListResource>, Error>> + Send + 'a
    {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_voice_consents(&params).await?;
                let has_more = page.data.has_more;
                let last_id = page.data.last_id.clone();
                yield page;
                let Some(next) = next_voice_consent_params(&params, has_more, last_id.as_deref())? else {
                    break;
                };
                params = next;
            }
        }
    }

    /// Adds a recording that documents consent for a custom voice. The generated
    /// `recording` field should match `recording.filename`; `AudioFile` supplies
    /// the bytes and MIME type. Requires `multipart`.
    #[cfg(feature = "multipart")]
    pub async fn create_voice_consent(
        &self,
        request: &types::audio::CreateVoiceConsentRequest,
        recording: AudioFile,
    ) -> Result<ApiResponse<types::audio::VoiceConsentResource>, Error> {
        self.create_voice_consent_with_options(request, recording, RequestOptions::default())
            .await
    }

    /// Adds a voice consent recording with per-request headers. Requires `multipart`.
    #[cfg(feature = "multipart")]
    pub async fn create_voice_consent_with_options(
        &self,
        request: &types::audio::CreateVoiceConsentRequest,
        recording: AudioFile,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::audio::VoiceConsentResource>, Error> {
        let response = self
            .client
            .send_multipart_files(
                append_path_segments(&self.client.base_url, &["audio", "voice_consents"])?,
                &options.headers,
                vec![recording.multipart("recording")],
                multipart_fields(request, &["recording"])?,
                ACCEPT_JSON,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Retrieves voice consent recording metadata.
    pub async fn retrieve_voice_consent(
        &self,
        consent_id: &str,
    ) -> Result<ApiResponse<types::audio::VoiceConsentResource>, Error> {
        self.retrieve_voice_consent_with_options(consent_id, RequestOptions::default())
            .await
    }

    /// Retrieves voice consent metadata with per-request headers.
    pub async fn retrieve_voice_consent_with_options(
        &self,
        consent_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::audio::VoiceConsentResource>, Error> {
        send_typed(
            self.client,
            Method::GET,
            append_path_segments(
                &self.client.base_url,
                &["audio", "voice_consents", consent_id],
            )?,
            None,
            &options.headers,
        )
        .await
    }

    /// Updates a voice consent recording's label.
    pub async fn update_voice_consent(
        &self,
        consent_id: &str,
        request: &types::audio::UpdateVoiceConsentRequest,
    ) -> Result<ApiResponse<types::audio::VoiceConsentResource>, Error> {
        self.update_voice_consent_with_options(consent_id, request, RequestOptions::default())
            .await
    }

    /// Updates voice consent metadata with per-request headers.
    pub async fn update_voice_consent_with_options(
        &self,
        consent_id: &str,
        request: &types::audio::UpdateVoiceConsentRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::audio::VoiceConsentResource>, Error> {
        send_typed(
            self.client,
            Method::POST,
            append_path_segments(
                &self.client.base_url,
                &["audio", "voice_consents", consent_id],
            )?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Deletes a voice consent recording.
    pub async fn delete_voice_consent(
        &self,
        consent_id: &str,
    ) -> Result<ApiResponse<types::audio::VoiceConsentDeletedResource>, Error> {
        self.delete_voice_consent_with_options(consent_id, RequestOptions::default())
            .await
    }

    /// Deletes a voice consent recording with per-request headers.
    pub async fn delete_voice_consent_with_options(
        &self,
        consent_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::audio::VoiceConsentDeletedResource>, Error> {
        send_typed(
            self.client,
            Method::DELETE,
            append_path_segments(
                &self.client.base_url,
                &["audio", "voice_consents", consent_id],
            )?,
            None,
            &options.headers,
        )
        .await
    }

    /// Creates a custom voice from a consent recording and sample. The generated
    /// `audio_sample` field should match `sample.filename`; `AudioFile` supplies
    /// the bytes and MIME type. Requires `multipart`.
    #[cfg(feature = "multipart")]
    pub async fn create_voice(
        &self,
        request: &types::audio::CreateVoiceRequest,
        sample: AudioFile,
    ) -> Result<ApiResponse<types::audio::VoiceResource>, Error> {
        self.create_voice_with_options(request, sample, RequestOptions::default())
            .await
    }

    /// Creates a custom voice with per-request headers. Requires `multipart`.
    #[cfg(feature = "multipart")]
    pub async fn create_voice_with_options(
        &self,
        request: &types::audio::CreateVoiceRequest,
        sample: AudioFile,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::audio::VoiceResource>, Error> {
        let response = self
            .client
            .send_multipart_files(
                append_path_segments(&self.client.base_url, &["audio", "voices"])?,
                &options.headers,
                vec![sample.multipart("audio_sample")],
                multipart_fields(request, &["audio_sample"])?,
                ACCEPT_JSON,
            )
            .await?;
        self.client.decode(response).await
    }
}

impl AudioFile {
    #[cfg(feature = "multipart")]
    fn multipart(&self, field_name: &str) -> MultipartFilePart {
        MultipartFilePart {
            field_name: field_name.to_owned(),
            filename: Some(self.filename.clone()),
            mime_type: self.content_type.clone(),
            contents: self.contents.clone(),
        }
    }
}

/// Files API operations for an [`OpenAIClient`].
pub struct FilesApi<'a> {
    client: &'a OpenAIClient,
}

impl FilesApi<'_> {
    /// Lists files visible to the configured API key.
    pub async fn list(
        &self,
        params: &types::files::ListFilesParams,
    ) -> Result<ApiResponse<types::files::ListFilesResponse>, Error> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// Lists files with per-request headers.
    pub async fn list_with_options(
        &self,
        params: &types::files::ListFilesParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::files::ListFilesResponse>, Error> {
        let mut endpoint = append_path_segments(&self.client.base_url, &["files"])?;
        append_query(&mut endpoint, params)?;
        let response = self
            .client
            .send(Method::GET, endpoint, None, ACCEPT_JSON, &options.headers)
            .await?;
        self.client.decode(response).await
    }

    /// Retrieves file metadata.
    pub async fn retrieve(
        &self,
        file_id: &str,
    ) -> Result<ApiResponse<types::files::OpenAiFile>, Error> {
        self.retrieve_with_options(file_id, RequestOptions::default())
            .await
    }

    /// Retrieves file metadata with per-request headers.
    pub async fn retrieve_with_options(
        &self,
        file_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::files::OpenAiFile>, Error> {
        let response = self
            .client
            .send(
                Method::GET,
                append_path_segments(&self.client.base_url, &["files", file_id])?,
                None,
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Deletes a file.
    pub async fn delete(
        &self,
        file_id: &str,
    ) -> Result<ApiResponse<types::files::DeleteFileResponse>, Error> {
        self.delete_with_options(file_id, RequestOptions::default())
            .await
    }

    /// Deletes a file with per-request headers.
    pub async fn delete_with_options(
        &self,
        file_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::files::DeleteFileResponse>, Error> {
        let response = self
            .client
            .send(
                Method::DELETE,
                append_path_segments(&self.client.base_url, &["files", file_id])?,
                None,
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Downloads file contents as an incremental byte stream.
    pub async fn download(&self, file_id: &str) -> Result<FileContentStream, Error> {
        self.download_with_options(file_id, RequestOptions::default())
            .await
    }

    /// Downloads file contents with per-request headers.
    pub async fn download_with_options(
        &self,
        file_id: &str,
        options: RequestOptions,
    ) -> Result<FileContentStream, Error> {
        let response = self
            .client
            .send(
                Method::GET,
                append_path_segments(&self.client.base_url, &["files", file_id, "content"])?,
                None,
                ACCEPT_OCTET_STREAM,
                &options.headers,
            )
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        let request_id = request_id(response.headers());
        Ok(FileContentStream {
            inner: Box::pin(response.bytes_stream()),
            request_id,
            idle_timeout: self.client.stream_idle_timeout,
            terminated: false,
        })
    }

    /// Uploads a file from replayable in-memory bytes. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn create(
        &self,
        request: &types::files::CreateFileRequest,
        contents: bytes::Bytes,
    ) -> Result<ApiResponse<types::files::OpenAiFile>, Error> {
        self.create_with_options(request, contents, RequestOptions::default())
            .await
    }

    /// Uploads a file with per-request headers. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn create_with_options(
        &self,
        request: &types::files::CreateFileRequest,
        contents: bytes::Bytes,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::files::OpenAiFile>, Error> {
        let mut fields = vec![("purpose".to_owned(), request.purpose.to_string())];
        if let Some(expiration) = &request.expires_after {
            fields.push((
                "expires_after[anchor]".to_owned(),
                expiration.anchor.to_string(),
            ));
            fields.push((
                "expires_after[seconds]".to_owned(),
                expiration.seconds.to_string(),
            ));
        }
        let response = self
            .client
            .send_multipart(
                append_path_segments(&self.client.base_url, &["files"])?,
                &options.headers,
                request.file.clone(),
                contents,
                fields,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Uploads a one-shot byte stream. The request body is consumed once and is
    /// never retried, even when a retry policy is configured.
    #[cfg(feature = "multipart")]
    pub async fn create_stream<S>(
        &self,
        request: &types::files::CreateFileRequest,
        contents: S,
    ) -> Result<ApiResponse<types::files::OpenAiFile>, Error>
    where
        S: Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send + 'static,
    {
        self.create_stream_with_options(request, contents, RequestOptions::default())
            .await
    }

    /// Uploads a one-shot byte stream with per-request headers. The body is
    /// never retried because it cannot be replayed.
    #[cfg(feature = "multipart")]
    pub async fn create_stream_with_options<S>(
        &self,
        request: &types::files::CreateFileRequest,
        contents: S,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::files::OpenAiFile>, Error>
    where
        S: Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send + 'static,
    {
        let mut fields = vec![("purpose".to_owned(), request.purpose.to_string())];
        if let Some(expiration) = &request.expires_after {
            fields.push((
                "expires_after[anchor]".to_owned(),
                expiration.anchor.to_string(),
            ));
            fields.push((
                "expires_after[seconds]".to_owned(),
                expiration.seconds.to_string(),
            ));
        }
        let response = self
            .client
            .send_multipart_stream(
                append_path_segments(&self.client.base_url, &["files"])?,
                &options.headers,
                "file",
                Some(&request.file),
                "application/octet-stream",
                contents,
                fields,
                ACCEPT_JSON,
            )
            .await?;
        self.client.decode(response).await
    }
}

/// Content Provenance API operations for an [`OpenAIClient`].
#[cfg(feature = "multipart")]
pub struct ContentProvenanceApi<'a> {
    client: &'a OpenAIClient,
}

#[cfg(feature = "multipart")]
impl ContentProvenanceApi<'_> {
    /// Checks a media file for supported OpenAI provenance signals.
    /// Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn check(
        &self,
        request: &types::content_provenance::CreateContentProvenanceBody,
        contents: bytes::Bytes,
    ) -> Result<ApiResponse<types::content_provenance::ProvenanceResource>, Error> {
        self.check_with_options(request, contents, RequestOptions::default())
            .await
    }

    /// Checks a media file with per-request headers. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn check_with_options(
        &self,
        request: &types::content_provenance::CreateContentProvenanceBody,
        contents: bytes::Bytes,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::content_provenance::ProvenanceResource>, Error> {
        let response = self
            .client
            .send_multipart(
                append_path_segments(&self.client.base_url, &["content_provenance_checks"])?,
                &options.headers,
                request.file.clone(),
                contents,
                Vec::new(),
            )
            .await?;
        self.client.decode(response).await
    }
}

/// Images API operations for an [`OpenAIClient`].
pub struct ImagesApi<'a> {
    client: &'a OpenAIClient,
}

impl ImagesApi<'_> {
    /// Generates images and returns their typed JSON response.
    pub async fn generate(
        &self,
        request: &types::images::CreateImageRequest,
    ) -> Result<ApiResponse<types::images::ImagesResponse>, Error> {
        self.generate_with_options(request, RequestOptions::default())
            .await
    }

    /// Generates images with per-request headers. This JSON method disables streaming.
    pub async fn generate_with_options(
        &self,
        request: &types::images::CreateImageRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::images::ImagesResponse>, Error> {
        let mut request = request.clone();
        request.stream = Some(false);
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(&self.client.base_url, &["images", "generations"])?,
                Some(serde_json::to_value(request)?),
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Generates images as typed SSE events, ending at `image_generation.completed`.
    pub async fn generate_stream(
        &self,
        request: &types::images::CreateImageRequest,
    ) -> Result<ImageEventStream, Error> {
        self.generate_stream_with_options(request, RequestOptions::default())
            .await
    }

    /// Generates image events with per-request headers.
    pub async fn generate_stream_with_options(
        &self,
        request: &types::images::CreateImageRequest,
        options: RequestOptions,
    ) -> Result<ImageEventStream, Error> {
        let mut request = request.clone();
        request.stream = Some(true);
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(&self.client.base_url, &["images", "generations"])?,
                Some(serde_json::to_value(request)?),
                ACCEPT_EVENTS,
                &options.headers,
            )
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        let request_id = request_id(response.headers());
        Ok(ImageEventStream::new(
            response,
            request_id,
            ImageEventKind::Generation,
            self.client.stream_idle_timeout,
        ))
    }

    /// Edits images using URL, data URL, or previously uploaded file references.
    pub async fn edit(
        &self,
        request: &types::images::EditImageBodyJsonParam,
    ) -> Result<ApiResponse<types::images::ImagesResponse>, Error> {
        self.edit_with_options(request, RequestOptions::default())
            .await
    }

    /// Edits images with per-request headers. This JSON method disables streaming.
    pub async fn edit_with_options(
        &self,
        request: &types::images::EditImageBodyJsonParam,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::images::ImagesResponse>, Error> {
        let mut request = request.clone();
        request.stream = Some(false);
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(&self.client.base_url, &["images", "edits"])?,
                Some(serde_json::to_value(request)?),
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Edits referenced images as typed SSE events.
    pub async fn edit_stream(
        &self,
        request: &types::images::EditImageBodyJsonParam,
    ) -> Result<ImageEventStream, Error> {
        self.edit_stream_with_options(request, RequestOptions::default())
            .await
    }

    /// Edits referenced images as events with per-request headers.
    pub async fn edit_stream_with_options(
        &self,
        request: &types::images::EditImageBodyJsonParam,
        options: RequestOptions,
    ) -> Result<ImageEventStream, Error> {
        let mut request = request.clone();
        request.stream = Some(true);
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(&self.client.base_url, &["images", "edits"])?,
                Some(serde_json::to_value(request)?),
                ACCEPT_EVENTS,
                &options.headers,
            )
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        let request_id = request_id(response.headers());
        Ok(ImageEventStream::new(
            response,
            request_id,
            ImageEventKind::Edit,
            self.client.stream_idle_timeout,
        ))
    }

    /// Edits uploaded image files. Images are sent as repeated `image[]` parts;
    /// JSON image references in `request` are ignored for this multipart call.
    #[cfg(feature = "multipart")]
    pub async fn edit_files(
        &self,
        request: &types::images::EditImageBodyJsonParam,
        images: &[ImageFile],
        mask: Option<ImageFile>,
    ) -> Result<ApiResponse<types::images::ImagesResponse>, Error> {
        self.edit_files_with_options(request, images, mask, RequestOptions::default())
            .await
    }

    /// Multipart file edit with per-request headers.
    #[cfg(feature = "multipart")]
    pub async fn edit_files_with_options(
        &self,
        request: &types::images::EditImageBodyJsonParam,
        images: &[ImageFile],
        mask: Option<ImageFile>,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::images::ImagesResponse>, Error> {
        if images.is_empty() || images.len() > 16 {
            return Err(Error::InvalidImageFileCount);
        }
        let mut fields = serde_json::to_value(request)?;
        let object = fields
            .as_object_mut()
            .expect("image edit request serializes as an object");
        object.remove("images");
        object.remove("mask");
        object.insert("stream".to_owned(), Value::Bool(false));
        let text_fields = object
            .iter()
            .filter(|(_, value)| !value.is_null())
            .map(|(name, value)| (name.clone(), multipart_text_value(value)))
            .collect();
        let mut file_parts = images
            .iter()
            .map(|image| MultipartFilePart {
                field_name: "image[]".to_owned(),
                filename: Some(image.filename.clone()),
                mime_type: image.content_type.clone(),
                contents: image.contents.clone(),
            })
            .collect::<Vec<_>>();
        if let Some(mask) = mask {
            file_parts.push(MultipartFilePart {
                field_name: "mask".to_owned(),
                filename: Some(mask.filename),
                mime_type: mask.content_type,
                contents: mask.contents,
            });
        }
        let response = self
            .client
            .send_multipart_files(
                append_path_segments(&self.client.base_url, &["images", "edits"])?,
                &options.headers,
                file_parts,
                text_fields,
                ACCEPT_JSON,
            )
            .await?;
        self.client.decode(response).await
    }
}

/// Conversations API operations for an [`OpenAIClient`].
pub struct ConversationsApi<'a> {
    client: &'a OpenAIClient,
}

impl ConversationsApi<'_> {
    fn endpoint(&self, conversation_id: Option<&str>, suffix: &[&str]) -> Result<Url, Error> {
        let mut segments = vec!["conversations"];
        if let Some(conversation_id) = conversation_id {
            segments.push(conversation_id);
        }
        segments.extend_from_slice(suffix);
        append_path_segments(&self.client.base_url, &segments)
    }

    /// Creates a conversation, optionally with initial items and metadata.
    pub async fn create(
        &self,
        request: &types::conversations::CreateConversationBody,
    ) -> Result<ApiResponse<types::conversations::ConversationResource>, Error> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    /// Creates a conversation with per-request headers.
    pub async fn create_with_options(
        &self,
        request: &types::conversations::CreateConversationBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::conversations::ConversationResource>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(None, &[])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Retrieves a conversation by ID.
    pub async fn retrieve(
        &self,
        conversation_id: &str,
    ) -> Result<ApiResponse<types::conversations::ConversationResource>, Error> {
        self.retrieve_with_options(conversation_id, RequestOptions::default())
            .await
    }

    /// Retrieves a conversation with per-request headers.
    pub async fn retrieve_with_options(
        &self,
        conversation_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::conversations::ConversationResource>, Error> {
        send_typed(
            self.client,
            Method::GET,
            self.endpoint(Some(conversation_id), &[])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Updates a conversation's metadata.
    pub async fn update(
        &self,
        conversation_id: &str,
        request: &types::conversations::UpdateConversationBody,
    ) -> Result<ApiResponse<types::conversations::ConversationResource>, Error> {
        self.update_with_options(conversation_id, request, RequestOptions::default())
            .await
    }

    /// Updates a conversation with per-request headers.
    pub async fn update_with_options(
        &self,
        conversation_id: &str,
        request: &types::conversations::UpdateConversationBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::conversations::ConversationResource>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(conversation_id), &[])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Deletes a conversation by ID.
    pub async fn delete(
        &self,
        conversation_id: &str,
    ) -> Result<ApiResponse<types::conversations::DeletedConversationResource>, Error> {
        self.delete_with_options(conversation_id, RequestOptions::default())
            .await
    }

    /// Deletes a conversation with per-request headers.
    pub async fn delete_with_options(
        &self,
        conversation_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::conversations::DeletedConversationResource>, Error> {
        send_typed(
            self.client,
            Method::DELETE,
            self.endpoint(Some(conversation_id), &[])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Lists one page of items in a conversation.
    pub async fn list_items(
        &self,
        conversation_id: &str,
        params: &types::conversations::ListConversationItemsParams,
    ) -> Result<ApiResponse<types::conversations::ConversationItemList>, Error> {
        self.list_items_with_options(conversation_id, params, RequestOptions::default())
            .await
    }

    /// Lists one page of conversation items with per-request headers.
    pub async fn list_items_with_options(
        &self,
        conversation_id: &str,
        params: &types::conversations::ListConversationItemsParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::conversations::ConversationItemList>, Error> {
        let mut endpoint = self.endpoint(Some(conversation_id), &["items"])?;
        append_query_omitting(&mut endpoint, params, &["conversation_id"])?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Streams conversation item pages while preserving each response and request ID.
    pub fn list_items_pages<'a>(
        &'a self,
        conversation_id: &'a str,
        params: types::conversations::ListConversationItemsParams,
    ) -> impl Stream<Item = Result<ApiResponse<types::conversations::ConversationItemList>, Error>>
    + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_items(conversation_id, &params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone();
                yield page;
                let Some(next) = next_conversation_items_params(&params, has_more, &last_id)? else {
                    break;
                };
                params = next;
            }
        }
    }

    /// Adds items to a conversation.
    pub async fn create_items(
        &self,
        conversation_id: &str,
        request: &types::conversations::CreateConversationItemsRequest,
        params: &types::conversations::CreateConversationItemsParams,
    ) -> Result<ApiResponse<types::conversations::ConversationItemList>, Error> {
        self.create_items_with_options(conversation_id, request, params, RequestOptions::default())
            .await
    }

    /// Adds items with per-request headers and include parameters.
    pub async fn create_items_with_options(
        &self,
        conversation_id: &str,
        request: &types::conversations::CreateConversationItemsRequest,
        params: &types::conversations::CreateConversationItemsParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::conversations::ConversationItemList>, Error> {
        let mut endpoint = self.endpoint(Some(conversation_id), &["items"])?;
        append_query_omitting(&mut endpoint, params, &["conversation_id"])?;
        send_typed(
            self.client,
            Method::POST,
            endpoint,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Retrieves an item from a conversation.
    pub async fn retrieve_item(
        &self,
        conversation_id: &str,
        item_id: &str,
        params: &types::conversations::GetConversationItemParams,
    ) -> Result<ApiResponse<types::conversations::ConversationItem>, Error> {
        self.retrieve_item_with_options(conversation_id, item_id, params, RequestOptions::default())
            .await
    }

    /// Retrieves a conversation item with per-request headers.
    pub async fn retrieve_item_with_options(
        &self,
        conversation_id: &str,
        item_id: &str,
        params: &types::conversations::GetConversationItemParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::conversations::ConversationItem>, Error> {
        let mut endpoint = self.endpoint(Some(conversation_id), &["items", item_id])?;
        append_query_omitting(&mut endpoint, params, &["conversation_id", "item_id"])?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Deletes an item from a conversation.
    pub async fn delete_item(
        &self,
        conversation_id: &str,
        item_id: &str,
    ) -> Result<ApiResponse<types::conversations::ConversationResource>, Error> {
        self.delete_item_with_options(conversation_id, item_id, RequestOptions::default())
            .await
    }

    /// Deletes a conversation item with per-request headers.
    pub async fn delete_item_with_options(
        &self,
        conversation_id: &str,
        item_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::conversations::ConversationResource>, Error> {
        send_typed(
            self.client,
            Method::DELETE,
            self.endpoint(Some(conversation_id), &["items", item_id])?,
            None,
            &options.headers,
        )
        .await
    }
}

/// Containers API operations for an [`OpenAIClient`].
pub struct ContainersApi<'a> {
    client: &'a OpenAIClient,
}

impl ContainersApi<'_> {
    fn endpoint(&self, container_id: Option<&str>, suffix: &[&str]) -> Result<Url, Error> {
        let mut segments = vec!["containers"];
        if let Some(container_id) = container_id {
            segments.push(container_id);
        }
        segments.extend_from_slice(suffix);
        append_path_segments(&self.client.base_url, &segments)
    }

    /// Lists one page of containers.
    pub async fn list(
        &self,
        params: &types::containers::ListContainersParams,
    ) -> Result<ApiResponse<types::containers::ContainerListResource>, Error> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// Lists one page of containers with per-request headers.
    pub async fn list_with_options(
        &self,
        params: &types::containers::ListContainersParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::containers::ContainerListResource>, Error> {
        let mut endpoint = self.endpoint(None, &[])?;
        append_query(&mut endpoint, params)?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Streams pages of containers and preserves each page response and request ID.
    pub fn list_pages<'a>(
        &'a self,
        params: types::containers::ListContainersParams,
    ) -> impl Stream<Item = Result<ApiResponse<types::containers::ContainerListResource>, Error>>
    + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list(&params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone();
                yield page;
                let Some(next) = next_container_params(&params, has_more, &last_id)? else {
                    break;
                };
                params = next;
            }
        }
    }

    /// Creates a container.
    pub async fn create(
        &self,
        request: &types::containers::CreateContainerBody,
    ) -> Result<ApiResponse<types::containers::ContainerResource>, Error> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    /// Creates a container with per-request headers.
    pub async fn create_with_options(
        &self,
        request: &types::containers::CreateContainerBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::containers::ContainerResource>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(None, &[])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Retrieves a container by ID.
    pub async fn retrieve(
        &self,
        container_id: &str,
    ) -> Result<ApiResponse<types::containers::ContainerResource>, Error> {
        self.retrieve_with_options(container_id, RequestOptions::default())
            .await
    }

    /// Retrieves a container with per-request headers.
    pub async fn retrieve_with_options(
        &self,
        container_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::containers::ContainerResource>, Error> {
        send_typed(
            self.client,
            Method::GET,
            self.endpoint(Some(container_id), &[])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Deletes a container by ID.
    pub async fn delete(
        &self,
        container_id: &str,
    ) -> Result<ApiResponse<types::containers::DeleteContainerResponse>, Error> {
        self.delete_with_options(container_id, RequestOptions::default())
            .await
    }

    /// Deletes a container with per-request headers.
    pub async fn delete_with_options(
        &self,
        container_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::containers::DeleteContainerResponse>, Error> {
        send_typed(
            self.client,
            Method::DELETE,
            self.endpoint(Some(container_id), &[])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Lists one page of files attached to a container.
    pub async fn list_files(
        &self,
        container_id: &str,
        params: &types::containers::ListContainerFilesParams,
    ) -> Result<ApiResponse<types::containers::ContainerFileListResource>, Error> {
        self.list_files_with_options(container_id, params, RequestOptions::default())
            .await
    }

    /// Lists container files with per-request headers.
    pub async fn list_files_with_options(
        &self,
        container_id: &str,
        params: &types::containers::ListContainerFilesParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::containers::ContainerFileListResource>, Error> {
        let mut endpoint = self.endpoint(Some(container_id), &["files"])?;
        append_query_omitting(&mut endpoint, params, &["container_id"])?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Streams pages of files attached to a container.
    pub fn list_files_pages<'a>(
        &'a self,
        container_id: &'a str,
        params: types::containers::ListContainerFilesParams,
    ) -> impl Stream<Item = Result<ApiResponse<types::containers::ContainerFileListResource>, Error>>
    + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_files(container_id, &params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone();
                yield page;
                let Some(next) = next_container_file_params(&params, has_more, &last_id)? else {
                    break;
                };
                params = next;
            }
        }
    }

    /// Adds an existing Files API object to the container.
    pub async fn create_file(
        &self,
        container_id: &str,
        request: &types::containers::CreateContainerFileBody,
    ) -> Result<ApiResponse<types::containers::ContainerFileResource>, Error> {
        self.create_file_with_options(container_id, request, RequestOptions::default())
            .await
    }

    /// Adds an existing Files API object with per-request headers.
    pub async fn create_file_with_options(
        &self,
        container_id: &str,
        request: &types::containers::CreateContainerFileBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::containers::ContainerFileResource>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(container_id), &["files"])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Uploads a file directly to a container. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn upload_file(
        &self,
        container_id: &str,
        filename: &str,
        contents: bytes::Bytes,
    ) -> Result<ApiResponse<types::containers::ContainerFileResource>, Error> {
        self.upload_file_with_options(container_id, filename, contents, RequestOptions::default())
            .await
    }

    /// Uploads a file to a container with per-request headers.
    #[cfg(feature = "multipart")]
    pub async fn upload_file_with_options(
        &self,
        container_id: &str,
        filename: &str,
        contents: bytes::Bytes,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::containers::ContainerFileResource>, Error> {
        let response = self
            .client
            .send_multipart(
                self.endpoint(Some(container_id), &["files"])?,
                &options.headers,
                filename.to_owned(),
                contents,
                Vec::new(),
            )
            .await?;
        self.client.decode(response).await
    }

    /// Retrieves metadata for a container file.
    pub async fn retrieve_file(
        &self,
        container_id: &str,
        file_id: &str,
    ) -> Result<ApiResponse<types::containers::ContainerFileResource>, Error> {
        self.retrieve_file_with_options(container_id, file_id, RequestOptions::default())
            .await
    }

    /// Retrieves container file metadata with per-request headers.
    pub async fn retrieve_file_with_options(
        &self,
        container_id: &str,
        file_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::containers::ContainerFileResource>, Error> {
        send_typed(
            self.client,
            Method::GET,
            self.endpoint(Some(container_id), &["files", file_id])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Deletes a file from a container.
    pub async fn delete_file(
        &self,
        container_id: &str,
        file_id: &str,
    ) -> Result<ApiResponse<types::containers::DeleteContainerFileResponse>, Error> {
        self.delete_file_with_options(container_id, file_id, RequestOptions::default())
            .await
    }

    /// Deletes a container file with per-request headers.
    pub async fn delete_file_with_options(
        &self,
        container_id: &str,
        file_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::containers::DeleteContainerFileResponse>, Error> {
        send_typed(
            self.client,
            Method::DELETE,
            self.endpoint(Some(container_id), &["files", file_id])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Downloads the contents of a container file incrementally.
    pub async fn download_file(
        &self,
        container_id: &str,
        file_id: &str,
    ) -> Result<FileContentStream, Error> {
        self.download_file_with_options(container_id, file_id, RequestOptions::default())
            .await
    }

    /// Downloads container file contents with per-request headers.
    pub async fn download_file_with_options(
        &self,
        container_id: &str,
        file_id: &str,
        options: RequestOptions,
    ) -> Result<FileContentStream, Error> {
        let response = self
            .client
            .send(
                Method::GET,
                self.endpoint(Some(container_id), &["files", file_id, "content"])?,
                None,
                ACCEPT_OCTET_STREAM,
                &options.headers,
            )
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        let request_id = request_id(response.headers());
        Ok(FileContentStream {
            inner: Box::pin(response.bytes_stream()),
            request_id,
            idle_timeout: self.client.stream_idle_timeout,
            terminated: false,
        })
    }
}

/// Realtime API operations for an [`OpenAIClient`].
pub struct RealtimeApi<'a> {
    client: &'a OpenAIClient,
}

impl RealtimeApi<'_> {
    /// Opens the GA Realtime WebSocket for a model. Events remain raw JSON so
    /// callers can handle newly added server and client event types.
    #[cfg(feature = "websocket")]
    pub async fn connect_websocket(
        &self,
        model: &str,
    ) -> Result<crate::WebSocketConnection, Error> {
        let mut endpoint = append_path_segments(&self.client.base_url, &["realtime"])?;
        endpoint.query_pairs_mut().append_pair("model", model);
        crate::websocket::WebSocketConnection::connect(self.client, endpoint).await
    }

    fn endpoint(&self, suffix: &[&str]) -> Result<Url, Error> {
        let mut segments = vec!["realtime"];
        segments.extend_from_slice(suffix);
        append_path_segments(&self.client.base_url, &segments)
    }

    /// Creates an ephemeral Realtime client secret.
    pub async fn create_client_secret(
        &self,
        request: &types::realtime::RealtimeCreateClientSecretRequest,
    ) -> Result<ApiResponse<types::realtime::RealtimeCreateClientSecretResponse>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(&["client_secrets"])?,
            Some(serde_json::to_value(request)?),
            &HeaderMap::new(),
        )
        .await
    }

    /// Creates a GA Realtime session and client secret.
    pub async fn create_session(
        &self,
        request: &types::realtime::RealtimeSessionCreateRequest,
    ) -> Result<ApiResponse<types::realtime::RealtimeSessionCreateResponse>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(&["sessions"])?,
            Some(serde_json::to_value(request)?),
            &HeaderMap::new(),
        )
        .await
    }

    /// Creates a transcription session and client secret.
    pub async fn create_transcription_session(
        &self,
        request: &types::realtime::RealtimeTranscriptionSessionCreateRequest,
    ) -> Result<ApiResponse<types::realtime::RealtimeTranscriptionSessionCreateResponse>, Error>
    {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(&["transcription_sessions"])?,
            Some(serde_json::to_value(request)?),
            &HeaderMap::new(),
        )
        .await
    }

    /// Creates a translation session and client secret.
    pub async fn create_translation_client_secret(
        &self,
        request: &types::realtime::RealtimeTranslationClientSecretCreateRequest,
    ) -> Result<ApiResponse<types::realtime::RealtimeTranslationClientSecretCreateResponse>, Error>
    {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(&["translations", "client_secrets"])?,
            Some(serde_json::to_value(request)?),
            &HeaderMap::new(),
        )
        .await
    }

    /// Creates a WebRTC call from an SDP offer and returns the SDP answer.
    pub async fn create_call(&self, sdp_offer: &str) -> Result<RealtimeCallAnswer, Error> {
        let response = self
            .client
            .http
            .post(self.endpoint(&["calls"])?)
            .header(AUTHORIZATION, format!("Bearer {}", self.client.api_key))
            .header(CONTENT_TYPE, "application/sdp")
            .header(ACCEPT, "application/sdp")
            .body(sdp_offer.to_owned());
        let response = if let Some(timeout) = self.client.request_timeout {
            tokio::time::timeout(timeout, response.send())
                .await
                .map_err(|_| Error::RequestTimeout)??
        } else {
            response.send().await?
        };
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        let request_id = request_id(response.headers());
        let sdp = response.text().await?;
        Ok(RealtimeCallAnswer { sdp, request_id })
    }

    /// Creates a WebRTC call with session configuration using multipart form data.
    /// Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn create_call_with_session(
        &self,
        sdp_offer: &str,
        session: &types::realtime::RealtimeSessionCreateRequestGa,
    ) -> Result<RealtimeCallAnswer, Error> {
        let session = serde_json::to_string(session)?;
        let response = self
            .client
            .send_multipart_files(
                self.endpoint(&["calls"])?,
                &HeaderMap::new(),
                Vec::new(),
                vec![
                    ("sdp".into(), sdp_offer.into()),
                    ("session".into(), session),
                ],
                "application/sdp",
            )
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        let request_id = request_id(response.headers());
        let sdp = response.text().await?;
        Ok(RealtimeCallAnswer { sdp, request_id })
    }

    /// Accepts an incoming SIP call with GA Realtime configuration.
    pub async fn accept_call(
        &self,
        call_id: &str,
        request: &types::realtime::RealtimeSessionCreateRequestGa,
    ) -> Result<(), Error> {
        self.send_empty(
            Method::POST,
            self.endpoint(&["calls", call_id, "accept"])?,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    /// Hangs up an active Realtime SIP call.
    pub async fn hangup_call(&self, call_id: &str) -> Result<(), Error> {
        self.send_empty(
            Method::POST,
            self.endpoint(&["calls", call_id, "hangup"])?,
            None,
        )
        .await
    }

    /// Transfers a Realtime SIP call to another destination.
    pub async fn refer_call(
        &self,
        call_id: &str,
        request: &types::realtime::RealtimeCallReferRequest,
    ) -> Result<(), Error> {
        self.send_empty(
            Method::POST,
            self.endpoint(&["calls", call_id, "refer"])?,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    /// Rejects an incoming Realtime SIP call.
    pub async fn reject_call(
        &self,
        call_id: &str,
        request: &types::realtime::RealtimeCallRejectRequest,
    ) -> Result<(), Error> {
        self.send_empty(
            Method::POST,
            self.endpoint(&["calls", call_id, "reject"])?,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    async fn send_empty(
        &self,
        method: Method,
        endpoint: Url,
        body: Option<Value>,
    ) -> Result<(), Error> {
        let response = self
            .client
            .send(method, endpoint, body, ACCEPT_JSON, &HeaderMap::new())
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(read_http_error(response).await)
        }
    }
}

/// SDP answer returned when creating a WebRTC call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeCallAnswer {
    pub sdp: String,
    pub request_id: Option<String>,
}

/// Live API operations for an [`OpenAIClient`].
pub struct LiveApi<'a> {
    client: &'a OpenAIClient,
}

impl LiveApi<'_> {
    /// Opens the Live WebSocket session endpoint for bidirectional JSON/audio events.
    #[cfg(feature = "websocket")]
    pub async fn connect_websocket(&self) -> Result<crate::WebSocketConnection, Error> {
        let endpoint = append_path_segments(&self.client.base_url, &["live", "sessions"])?;
        crate::websocket::WebSocketConnection::connect(self.client, endpoint).await
    }

    fn endpoint(&self, session_id: Option<&str>, suffix: &[&str]) -> Result<Url, Error> {
        let mut segments = vec!["live", "sessions"];
        if let Some(session_id) = session_id {
            segments.push(session_id);
        }
        segments.extend_from_slice(suffix);
        append_path_segments(&self.client.base_url, &segments)
    }

    /// Creates a Live WebRTC or outbound SIP session.
    pub async fn create_session(
        &self,
        request: &types::live::LiveSessionCreateRequest,
    ) -> Result<ApiResponse<types::live::LiveSessionCreateResponse>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(None, &[])?,
            Some(serde_json::to_value(request)?),
            &HeaderMap::new(),
        )
        .await
    }

    /// Accepts an incoming SIP call and starts its Live session.
    pub async fn accept_session(
        &self,
        session_id: &str,
        request: &types::live::LiveCallAcceptRequest,
    ) -> Result<(), Error> {
        self.send_empty(
            Method::POST,
            self.endpoint(Some(session_id), &["accept"])?,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    /// Downloads a session recording in chunks.
    pub async fn download_recording(&self, session_id: &str) -> Result<FileContentStream, Error> {
        let response = self
            .client
            .send(
                Method::GET,
                self.endpoint(Some(session_id), &["content"])?,
                None,
                ACCEPT_AUDIO,
                &HeaderMap::new(),
            )
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        let request_id = request_id(response.headers());
        Ok(FileContentStream {
            inner: Box::pin(response.bytes_stream()),
            request_id,
            idle_timeout: self.client.stream_idle_timeout,
            terminated: false,
        })
    }

    /// Forks a stored Live session onto a new WebRTC connection.
    pub async fn fork_session(
        &self,
        session_id: &str,
        request: &types::live::LiveForkRequest,
    ) -> Result<ApiResponse<types::live::LiveCreateResponse>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(session_id), &["fork"])?,
            Some(serde_json::to_value(request)?),
            &HeaderMap::new(),
        )
        .await
    }

    /// Hangs up an active Live session.
    pub async fn hangup_session(&self, session_id: &str) -> Result<(), Error> {
        self.send_empty(
            Method::POST,
            self.endpoint(Some(session_id), &["hangup"])?,
            None,
        )
        .await
    }

    /// Transfers a SIP call to another destination.
    pub async fn refer_session(
        &self,
        session_id: &str,
        request: &types::live::LiveCallReferRequest,
    ) -> Result<(), Error> {
        self.send_empty(
            Method::POST,
            self.endpoint(Some(session_id), &["refer"])?,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    /// Rejects an incoming SIP call.
    pub async fn reject_session(
        &self,
        session_id: &str,
        request: &types::live::LiveCallRejectRequest,
    ) -> Result<(), Error> {
        self.send_empty(
            Method::POST,
            self.endpoint(Some(session_id), &["reject"])?,
            Some(serde_json::to_value(request)?),
        )
        .await
    }

    async fn send_empty(
        &self,
        method: Method,
        endpoint: Url,
        body: Option<Value>,
    ) -> Result<(), Error> {
        let response = self
            .client
            .send(method, endpoint, body, ACCEPT_JSON, &HeaderMap::new())
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(read_http_error(response).await)
        }
    }
}

/// Fine-tuning API operations for an [`OpenAIClient`].
pub struct FineTuningApi<'a> {
    client: &'a OpenAIClient,
}

impl FineTuningApi<'_> {
    fn endpoint(&self, suffix: &[&str]) -> Result<Url, Error> {
        let mut segments = vec!["fine_tuning"];
        segments.extend_from_slice(suffix);
        append_path_segments(&self.client.base_url, &segments)
    }

    /// Lists one page of fine-tuning jobs.
    pub async fn list_jobs(
        &self,
        params: &types::fine_tuning::ListPaginatedFineTuningJobsParams,
    ) -> Result<ApiResponse<types::fine_tuning::ListPaginatedFineTuningJobsResponse>, Error> {
        let mut endpoint = self.endpoint(&["jobs"])?;
        append_query(&mut endpoint, params)?;
        send_typed(self.client, Method::GET, endpoint, None, &HeaderMap::new()).await
    }

    /// Streams all pages of fine-tuning jobs.
    pub fn list_job_pages<'a>(
        &'a self,
        params: types::fine_tuning::ListPaginatedFineTuningJobsParams,
    ) -> impl Stream<
        Item = Result<ApiResponse<types::fine_tuning::ListPaginatedFineTuningJobsResponse>, Error>,
    > + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_jobs(&params).await?;
                let has_more = page.has_more;
                let last_id = page.data.data.last().map(|job| job.id.clone());
                yield page;
                if !has_more { break; }
                let last_id = match last_id.filter(|id| !id.is_empty()) { Some(id) => id, None => { Err(Error::InvalidPaginationCursor)?; unreachable!() } };
                if params.after.as_deref() == Some(&last_id) { Err(Error::InvalidPaginationCursor)?; }
                params.after = Some(last_id);
            }
        }
    }

    /// Creates a fine-tuning job.
    pub async fn create_job(
        &self,
        request: &types::fine_tuning::CreateFineTuningJobRequest,
    ) -> Result<ApiResponse<types::fine_tuning::FineTuningJob>, Error> {
        self.create_job_with_options(request, RequestOptions::default())
            .await
    }

    pub async fn create_job_with_options(
        &self,
        request: &types::fine_tuning::CreateFineTuningJobRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::fine_tuning::FineTuningJob>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(&["jobs"])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Retrieves a fine-tuning job.
    pub async fn retrieve_job(
        &self,
        job_id: &str,
    ) -> Result<ApiResponse<types::fine_tuning::FineTuningJob>, Error> {
        send_typed(
            self.client,
            Method::GET,
            self.endpoint(&["jobs", job_id])?,
            None,
            &HeaderMap::new(),
        )
        .await
    }

    /// Cancels a fine-tuning job.
    pub async fn cancel_job(
        &self,
        job_id: &str,
    ) -> Result<ApiResponse<types::fine_tuning::FineTuningJob>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(&["jobs", job_id, "cancel"])?,
            None,
            &HeaderMap::new(),
        )
        .await
    }

    /// Pauses a fine-tuning job.
    pub async fn pause_job(
        &self,
        job_id: &str,
    ) -> Result<ApiResponse<types::fine_tuning::FineTuningJob>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(&["jobs", job_id, "pause"])?,
            None,
            &HeaderMap::new(),
        )
        .await
    }

    /// Resumes a fine-tuning job.
    pub async fn resume_job(
        &self,
        job_id: &str,
    ) -> Result<ApiResponse<types::fine_tuning::FineTuningJob>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(&["jobs", job_id, "resume"])?,
            None,
            &HeaderMap::new(),
        )
        .await
    }

    /// Lists one page of fine-tuning events for a job.
    pub async fn list_events(
        &self,
        job_id: &str,
        params: &types::fine_tuning::ListFineTuningEventsParams,
    ) -> Result<ApiResponse<types::fine_tuning::ListFineTuningJobEventsResponse>, Error> {
        let mut endpoint = self.endpoint(&["jobs", job_id, "events"])?;
        append_query_omitting(&mut endpoint, params, &["fine_tuning_job_id"])?;
        send_typed(self.client, Method::GET, endpoint, None, &HeaderMap::new()).await
    }

    /// Streams fine-tuning event pages.
    pub fn list_event_pages<'a>(
        &'a self,
        job_id: &'a str,
        params: types::fine_tuning::ListFineTuningEventsParams,
    ) -> impl Stream<
        Item = Result<ApiResponse<types::fine_tuning::ListFineTuningJobEventsResponse>, Error>,
    > + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_events(job_id, &params).await?;
                let has_more = page.has_more;
                let last_id = page.data.data.last().map(|event| event.id.clone());
                yield page;
                if !has_more { break; }
                let last_id = match last_id.filter(|id| !id.is_empty()) { Some(id) => id, None => { Err(Error::InvalidPaginationCursor)?; unreachable!() } };
                if params.after.as_deref() == Some(&last_id) { Err(Error::InvalidPaginationCursor)?; }
                params.after = Some(last_id);
            }
        }
    }

    /// Lists one page of job checkpoints.
    pub async fn list_checkpoints(
        &self,
        job_id: &str,
        params: &types::fine_tuning::ListFineTuningJobCheckpointsParams,
    ) -> Result<ApiResponse<types::fine_tuning::ListFineTuningJobCheckpointsResponse>, Error> {
        let mut endpoint = self.endpoint(&["jobs", job_id, "checkpoints"])?;
        append_query_omitting(&mut endpoint, params, &["fine_tuning_job_id"])?;
        send_typed(self.client, Method::GET, endpoint, None, &HeaderMap::new()).await
    }

    /// Streams job checkpoint pages.
    pub fn list_checkpoint_pages<'a>(
        &'a self,
        job_id: &'a str,
        params: types::fine_tuning::ListFineTuningJobCheckpointsParams,
    ) -> impl Stream<
        Item = Result<ApiResponse<types::fine_tuning::ListFineTuningJobCheckpointsResponse>, Error>,
    > + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_checkpoints(job_id, &params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone().or_else(|| page.data.data.last().map(|item| item.id.clone()));
                yield page;
                if !has_more { break; }
                let last_id = match last_id.filter(|id| !id.is_empty()) { Some(id) => id, None => { Err(Error::InvalidPaginationCursor)?; unreachable!() } };
                if params.after.as_deref() == Some(&last_id) { Err(Error::InvalidPaginationCursor)?; }
                params.after = Some(last_id);
            }
        }
    }

    /// Lists one page of checkpoint permissions.
    pub async fn list_checkpoint_permissions(
        &self,
        checkpoint: &str,
        params: &types::fine_tuning::ListFineTuningCheckpointPermissionsParams,
    ) -> Result<ApiResponse<types::fine_tuning::ListFineTuningCheckpointPermissionResponse>, Error>
    {
        let mut endpoint = self.endpoint(&["checkpoints", checkpoint, "permissions"])?;
        append_query_omitting(&mut endpoint, params, &["fine_tuned_model_checkpoint"])?;
        send_typed(self.client, Method::GET, endpoint, None, &HeaderMap::new()).await
    }

    /// Streams pages of checkpoint permissions.
    pub fn list_checkpoint_permission_pages<'a>(
        &'a self,
        checkpoint: &'a str,
        params: types::fine_tuning::ListFineTuningCheckpointPermissionsParams,
    ) -> impl Stream<
        Item = Result<
            ApiResponse<types::fine_tuning::ListFineTuningCheckpointPermissionResponse>,
            Error,
        >,
    > + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_checkpoint_permissions(checkpoint, &params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone().or_else(|| page.data.data.last().map(|item| item.id.clone()));
                yield page;
                if !has_more { break; }
                let last_id = match last_id.filter(|id| !id.is_empty()) { Some(id) => id, None => { Err(Error::InvalidPaginationCursor)?; unreachable!() } };
                if params.after.as_deref() == Some(&last_id) { Err(Error::InvalidPaginationCursor)?; }
                params.after = Some(last_id);
            }
        }
    }

    /// Grants projects access to a fine-tuned model checkpoint.
    pub async fn create_checkpoint_permissions(
        &self,
        checkpoint: &str,
        request: &types::fine_tuning::CreateFineTuningCheckpointPermissionRequest,
    ) -> Result<ApiResponse<types::fine_tuning::ListFineTuningCheckpointPermissionResponse>, Error>
    {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(&["checkpoints", checkpoint, "permissions"])?,
            Some(serde_json::to_value(request)?),
            &HeaderMap::new(),
        )
        .await
    }

    /// Deletes a permission from a fine-tuned model checkpoint.
    pub async fn delete_checkpoint_permission(
        &self,
        checkpoint: &str,
        permission_id: &str,
    ) -> Result<ApiResponse<types::fine_tuning::DeleteFineTuningCheckpointPermissionResponse>, Error>
    {
        send_typed(
            self.client,
            Method::DELETE,
            self.endpoint(&["checkpoints", checkpoint, "permissions", permission_id])?,
            None,
            &HeaderMap::new(),
        )
        .await
    }
}

/// Vector Stores API operations for an [`OpenAIClient`].
pub struct VectorStoresApi<'a> {
    client: &'a OpenAIClient,
}

impl VectorStoresApi<'_> {
    fn endpoint(&self, vector_store_id: Option<&str>, suffix: &[&str]) -> Result<Url, Error> {
        let mut segments = vec!["vector_stores"];
        if let Some(vector_store_id) = vector_store_id {
            segments.push(vector_store_id);
        }
        segments.extend_from_slice(suffix);
        append_path_segments(&self.client.base_url, &segments)
    }

    /// Lists one page of vector stores.
    pub async fn list(
        &self,
        params: &types::vector_stores::ListVectorStoresParams,
    ) -> Result<ApiResponse<types::vector_stores::ListVectorStoresResponse>, Error> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    pub async fn list_with_options(
        &self,
        params: &types::vector_stores::ListVectorStoresParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::ListVectorStoresResponse>, Error> {
        let mut endpoint = self.endpoint(None, &[])?;
        append_query(&mut endpoint, params)?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Streams vector-store pages, preserving each page and its request ID.
    pub fn list_pages<'a>(
        &'a self,
        params: types::vector_stores::ListVectorStoresParams,
    ) -> impl Stream<
        Item = Result<ApiResponse<types::vector_stores::ListVectorStoresResponse>, Error>,
    > + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list(&params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone();
                yield page;
                if !has_more { break; }
                if last_id.is_empty() || params.after.as_deref() == Some(last_id.as_str()) {
                    Err(Error::InvalidPaginationCursor)?;
                }
                params.after = Some(last_id);
                params.before = None;
            }
        }
    }

    /// Creates a vector store.
    pub async fn create(
        &self,
        request: &types::vector_stores::CreateVectorStoreRequest,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreObject>, Error> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    pub async fn create_with_options(
        &self,
        request: &types::vector_stores::CreateVectorStoreRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreObject>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(None, &[])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Retrieves a vector store by ID.
    pub async fn retrieve(
        &self,
        vector_store_id: &str,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreObject>, Error> {
        self.retrieve_with_options(vector_store_id, RequestOptions::default())
            .await
    }

    pub async fn retrieve_with_options(
        &self,
        vector_store_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreObject>, Error> {
        send_typed(
            self.client,
            Method::GET,
            self.endpoint(Some(vector_store_id), &[])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Updates a vector store.
    pub async fn update(
        &self,
        vector_store_id: &str,
        request: &types::vector_stores::UpdateVectorStoreRequest,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreObject>, Error> {
        self.update_with_options(vector_store_id, request, RequestOptions::default())
            .await
    }

    pub async fn update_with_options(
        &self,
        vector_store_id: &str,
        request: &types::vector_stores::UpdateVectorStoreRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreObject>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(vector_store_id), &[])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Deletes a vector store.
    pub async fn delete(
        &self,
        vector_store_id: &str,
    ) -> Result<ApiResponse<types::vector_stores::DeleteVectorStoreResponse>, Error> {
        self.delete_with_options(vector_store_id, RequestOptions::default())
            .await
    }

    pub async fn delete_with_options(
        &self,
        vector_store_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::DeleteVectorStoreResponse>, Error> {
        send_typed(
            self.client,
            Method::DELETE,
            self.endpoint(Some(vector_store_id), &[])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Creates a file batch in a vector store.
    pub async fn create_file_batch(
        &self,
        vector_store_id: &str,
        request: &types::vector_stores::CreateVectorStoreFileBatchRequest,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileBatchObject>, Error> {
        self.create_file_batch_with_options(vector_store_id, request, RequestOptions::default())
            .await
    }

    pub async fn create_file_batch_with_options(
        &self,
        vector_store_id: &str,
        request: &types::vector_stores::CreateVectorStoreFileBatchRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileBatchObject>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(vector_store_id), &["file_batches"])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Retrieves a vector-store file batch.
    pub async fn retrieve_file_batch(
        &self,
        vector_store_id: &str,
        batch_id: &str,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileBatchObject>, Error> {
        self.retrieve_file_batch_with_options(vector_store_id, batch_id, RequestOptions::default())
            .await
    }

    pub async fn retrieve_file_batch_with_options(
        &self,
        vector_store_id: &str,
        batch_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileBatchObject>, Error> {
        send_typed(
            self.client,
            Method::GET,
            self.endpoint(Some(vector_store_id), &["file_batches", batch_id])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Cancels a vector-store file batch.
    pub async fn cancel_file_batch(
        &self,
        vector_store_id: &str,
        batch_id: &str,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileBatchObject>, Error> {
        self.cancel_file_batch_with_options(vector_store_id, batch_id, RequestOptions::default())
            .await
    }

    pub async fn cancel_file_batch_with_options(
        &self,
        vector_store_id: &str,
        batch_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileBatchObject>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(vector_store_id), &["file_batches", batch_id, "cancel"])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Lists one page of files in a vector-store batch.
    pub async fn list_file_batch_files(
        &self,
        vector_store_id: &str,
        batch_id: &str,
        params: &types::vector_stores::ListFilesInVectorStoreBatchParams,
    ) -> Result<ApiResponse<types::vector_stores::ListVectorStoreFilesResponse>, Error> {
        self.list_file_batch_files_with_options(
            vector_store_id,
            batch_id,
            params,
            RequestOptions::default(),
        )
        .await
    }

    pub async fn list_file_batch_files_with_options(
        &self,
        vector_store_id: &str,
        batch_id: &str,
        params: &types::vector_stores::ListFilesInVectorStoreBatchParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::ListVectorStoreFilesResponse>, Error> {
        let mut endpoint =
            self.endpoint(Some(vector_store_id), &["file_batches", batch_id, "files"])?;
        append_query_omitting(&mut endpoint, params, &["vector_store_id", "batch_id"])?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Streams pages of files included in a vector-store batch.
    pub fn list_file_batch_file_pages<'a>(
        &'a self,
        vector_store_id: &'a str,
        batch_id: &'a str,
        params: types::vector_stores::ListFilesInVectorStoreBatchParams,
    ) -> impl Stream<
        Item = Result<ApiResponse<types::vector_stores::ListVectorStoreFilesResponse>, Error>,
    > + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_file_batch_files(vector_store_id, batch_id, &params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone();
                yield page;
                if !has_more { break; }
                if last_id.is_empty() || params.after.as_deref() == Some(last_id.as_str()) {
                    Err(Error::InvalidPaginationCursor)?;
                }
                params.after = Some(last_id);
                params.before = None;
            }
        }
    }

    /// Lists one page of files attached to a vector store.
    pub async fn list_files(
        &self,
        vector_store_id: &str,
        params: &types::vector_stores::ListVectorStoreFilesParams,
    ) -> Result<ApiResponse<types::vector_stores::ListVectorStoreFilesResponse>, Error> {
        self.list_files_with_options(vector_store_id, params, RequestOptions::default())
            .await
    }

    pub async fn list_files_with_options(
        &self,
        vector_store_id: &str,
        params: &types::vector_stores::ListVectorStoreFilesParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::ListVectorStoreFilesResponse>, Error> {
        let mut endpoint = self.endpoint(Some(vector_store_id), &["files"])?;
        append_query_omitting(&mut endpoint, params, &["vector_store_id"])?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Streams vector-store file pages.
    pub fn list_files_pages<'a>(
        &'a self,
        vector_store_id: &'a str,
        params: types::vector_stores::ListVectorStoreFilesParams,
    ) -> impl Stream<
        Item = Result<ApiResponse<types::vector_stores::ListVectorStoreFilesResponse>, Error>,
    > + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list_files(vector_store_id, &params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone();
                yield page;
                if !has_more { break; }
                if last_id.is_empty() || params.after.as_deref() == Some(last_id.as_str()) {
                    Err(Error::InvalidPaginationCursor)?;
                }
                params.after = Some(last_id);
                params.before = None;
            }
        }
    }

    /// Adds an existing Files API object to a vector store.
    pub async fn create_file(
        &self,
        vector_store_id: &str,
        request: &types::vector_stores::CreateVectorStoreFileRequest,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileObject>, Error> {
        self.create_file_with_options(vector_store_id, request, RequestOptions::default())
            .await
    }

    pub async fn create_file_with_options(
        &self,
        vector_store_id: &str,
        request: &types::vector_stores::CreateVectorStoreFileRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileObject>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(vector_store_id), &["files"])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Retrieves metadata for a file attached to a vector store.
    pub async fn retrieve_file(
        &self,
        vector_store_id: &str,
        file_id: &str,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileObject>, Error> {
        self.retrieve_file_with_options(vector_store_id, file_id, RequestOptions::default())
            .await
    }

    pub async fn retrieve_file_with_options(
        &self,
        vector_store_id: &str,
        file_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileObject>, Error> {
        send_typed(
            self.client,
            Method::GET,
            self.endpoint(Some(vector_store_id), &["files", file_id])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Updates a vector-store file's attributes.
    pub async fn update_file_attributes(
        &self,
        vector_store_id: &str,
        file_id: &str,
        request: &types::vector_stores::UpdateVectorStoreFileAttributesRequest,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileObject>, Error> {
        self.update_file_attributes_with_options(
            vector_store_id,
            file_id,
            request,
            RequestOptions::default(),
        )
        .await
    }

    pub async fn update_file_attributes_with_options(
        &self,
        vector_store_id: &str,
        file_id: &str,
        request: &types::vector_stores::UpdateVectorStoreFileAttributesRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileObject>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(vector_store_id), &["files", file_id])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Removes a file from a vector store.
    pub async fn delete_file(
        &self,
        vector_store_id: &str,
        file_id: &str,
    ) -> Result<ApiResponse<types::vector_stores::DeleteVectorStoreFileResponse>, Error> {
        self.delete_file_with_options(vector_store_id, file_id, RequestOptions::default())
            .await
    }

    pub async fn delete_file_with_options(
        &self,
        vector_store_id: &str,
        file_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::DeleteVectorStoreFileResponse>, Error> {
        send_typed(
            self.client,
            Method::DELETE,
            self.endpoint(Some(vector_store_id), &["files", file_id])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Retrieves one page of parsed file content.
    pub async fn retrieve_file_content(
        &self,
        vector_store_id: &str,
        file_id: &str,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileContentResponse>, Error> {
        self.retrieve_file_content_with_options(vector_store_id, file_id, RequestOptions::default())
            .await
    }

    pub async fn retrieve_file_content_with_options(
        &self,
        vector_store_id: &str,
        file_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileContentResponse>, Error> {
        self.retrieve_file_content_page(vector_store_id, file_id, None, &options.headers)
            .await
    }

    async fn retrieve_file_content_page(
        &self,
        vector_store_id: &str,
        file_id: &str,
        page: Option<&str>,
        headers: &HeaderMap,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreFileContentResponse>, Error> {
        let mut endpoint = self.endpoint(Some(vector_store_id), &["files", file_id, "content"])?;
        if let Some(page) = page {
            endpoint.query_pairs_mut().append_pair("page", page);
        }
        send_typed(self.client, Method::GET, endpoint, None, headers).await
    }

    /// Streams pages of parsed file content.
    pub fn retrieve_file_content_pages<'a>(
        &'a self,
        vector_store_id: &'a str,
        file_id: &'a str,
    ) -> impl Stream<
        Item = Result<ApiResponse<types::vector_stores::VectorStoreFileContentResponse>, Error>,
    > + Send
    + 'a {
        async_stream::try_stream! {
            let mut cursor: Option<String> = None;
            loop {
                let page = self.retrieve_file_content_page(vector_store_id, file_id, cursor.as_deref(), &HeaderMap::new()).await?;
                let has_more = page.has_more;
                let next = page.next_page.clone();
                yield page;
                if !has_more { break; }
                let next = match next.filter(|next| !next.is_empty()) {
                    Some(next) => next,
                    None => { Err(Error::InvalidPaginationCursor)?; unreachable!() }
                };
                if cursor.as_deref() == Some(next.as_str()) {
                    Err(Error::InvalidPaginationCursor)?;
                }
                cursor = Some(next);
            }
        }
    }

    /// Searches a vector store.
    pub async fn search(
        &self,
        vector_store_id: &str,
        request: &types::vector_stores::VectorStoreSearchRequest,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreSearchResultsPage>, Error> {
        self.search_with_options(vector_store_id, request, None, RequestOptions::default())
            .await
    }

    pub async fn search_with_options(
        &self,
        vector_store_id: &str,
        request: &types::vector_stores::VectorStoreSearchRequest,
        page: Option<&str>,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::vector_stores::VectorStoreSearchResultsPage>, Error> {
        let mut endpoint = self.endpoint(Some(vector_store_id), &["search"])?;
        if let Some(page) = page {
            endpoint.query_pairs_mut().append_pair("page", page);
        }
        send_typed(
            self.client,
            Method::POST,
            endpoint,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Streams pages of vector-store search results.
    pub fn search_pages<'a>(
        &'a self,
        vector_store_id: &'a str,
        request: types::vector_stores::VectorStoreSearchRequest,
    ) -> impl Stream<
        Item = Result<ApiResponse<types::vector_stores::VectorStoreSearchResultsPage>, Error>,
    > + Send
    + 'a {
        async_stream::try_stream! {
            let mut cursor: Option<String> = None;
            loop {
                let page = self.search_with_options(vector_store_id, &request, cursor.as_deref(), RequestOptions::default()).await?;
                let has_more = page.has_more;
                let next = page.next_page.clone();
                yield page;
                if !has_more { break; }
                let next = match next.filter(|next| !next.is_empty()) {
                    Some(next) => next,
                    None => { Err(Error::InvalidPaginationCursor)?; unreachable!() }
                };
                if cursor.as_deref() == Some(next.as_str()) {
                    Err(Error::InvalidPaginationCursor)?;
                }
                cursor = Some(next);
            }
        }
    }
}

/// Batches API operations for an [`OpenAIClient`].
pub struct BatchesApi<'a> {
    client: &'a OpenAIClient,
}

impl BatchesApi<'_> {
    fn endpoint(&self, batch_id: Option<&str>, suffix: &[&str]) -> Result<Url, Error> {
        let mut segments = vec!["batches"];
        if let Some(batch_id) = batch_id {
            segments.push(batch_id);
        }
        segments.extend_from_slice(suffix);
        append_path_segments(&self.client.base_url, &segments)
    }

    /// Creates a batch from an uploaded JSONL file.
    pub async fn create(
        &self,
        request: &types::batches::CreateBatchRequest,
    ) -> Result<ApiResponse<types::batches::Batch>, Error> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    /// Creates a batch with per-request headers.
    pub async fn create_with_options(
        &self,
        request: &types::batches::CreateBatchRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::batches::Batch>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(None, &[])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Lists one page of batches.
    pub async fn list(
        &self,
        params: &types::batches::ListBatchesParams,
    ) -> Result<ApiResponse<types::batches::ListBatchesResponse>, Error> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// Lists batches with per-request headers.
    pub async fn list_with_options(
        &self,
        params: &types::batches::ListBatchesParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::batches::ListBatchesResponse>, Error> {
        let mut endpoint = self.endpoint(None, &[])?;
        append_query(&mut endpoint, params)?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Streams pages of batches, preserving each page response and request ID.
    pub fn list_pages<'a>(
        &'a self,
        params: types::batches::ListBatchesParams,
    ) -> impl Stream<Item = Result<ApiResponse<types::batches::ListBatchesResponse>, Error>> + Send + 'a
    {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list(&params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone();
                yield page;
                let Some(next) = next_batch_params(&params, has_more, last_id.as_deref())? else {
                    break;
                };
                params = next;
            }
        }
    }

    /// Retrieves a batch by ID.
    pub async fn retrieve(
        &self,
        batch_id: &str,
    ) -> Result<ApiResponse<types::batches::Batch>, Error> {
        self.retrieve_with_options(batch_id, RequestOptions::default())
            .await
    }

    /// Retrieves a batch with per-request headers.
    pub async fn retrieve_with_options(
        &self,
        batch_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::batches::Batch>, Error> {
        send_typed(
            self.client,
            Method::GET,
            self.endpoint(Some(batch_id), &[])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Cancels a batch that is in progress.
    pub async fn cancel(
        &self,
        batch_id: &str,
    ) -> Result<ApiResponse<types::batches::Batch>, Error> {
        self.cancel_with_options(batch_id, RequestOptions::default())
            .await
    }

    /// Cancels a batch with per-request headers.
    pub async fn cancel_with_options(
        &self,
        batch_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::batches::Batch>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(batch_id), &["cancel"])?,
            None,
            &options.headers,
        )
        .await
    }
}

/// Multipart Uploads API operations for an [`OpenAIClient`].
pub struct UploadsApi<'a> {
    client: &'a OpenAIClient,
}

impl UploadsApi<'_> {
    /// Creates an upload that accepts ordered byte parts.
    pub async fn create(
        &self,
        request: &types::uploads::CreateUploadRequest,
    ) -> Result<ApiResponse<types::uploads::Upload>, Error> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    /// Creates an upload with per-request headers.
    pub async fn create_with_options(
        &self,
        request: &types::uploads::CreateUploadRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::uploads::Upload>, Error> {
        send_typed(
            self.client,
            Method::POST,
            append_path_segments(&self.client.base_url, &["uploads"])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Adds a byte chunk as one multipart `data` part. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn add_part(
        &self,
        upload_id: &str,
        contents: bytes::Bytes,
    ) -> Result<ApiResponse<types::uploads::UploadPart>, Error> {
        self.add_part_with_options(upload_id, contents, RequestOptions::default())
            .await
    }

    /// Adds a byte chunk with per-request headers. Requires the `multipart` feature.
    #[cfg(feature = "multipart")]
    pub async fn add_part_with_options(
        &self,
        upload_id: &str,
        contents: bytes::Bytes,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::uploads::UploadPart>, Error> {
        let response = self
            .client
            .send_multipart_part(
                append_path_segments(&self.client.base_url, &["uploads", upload_id, "parts"])?,
                &options.headers,
                "data",
                None,
                contents,
                Vec::new(),
                ACCEPT_JSON,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Adds a one-shot byte stream as one multipart `data` part. The body is
    /// consumed once and is never retried.
    #[cfg(feature = "multipart")]
    pub async fn add_part_stream<S>(
        &self,
        upload_id: &str,
        contents: S,
    ) -> Result<ApiResponse<types::uploads::UploadPart>, Error>
    where
        S: Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send + 'static,
    {
        self.add_part_stream_with_options(upload_id, contents, RequestOptions::default())
            .await
    }

    /// Adds a one-shot byte stream with per-request headers. The body is never
    /// retried because it cannot be replayed.
    #[cfg(feature = "multipart")]
    pub async fn add_part_stream_with_options<S>(
        &self,
        upload_id: &str,
        contents: S,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::uploads::UploadPart>, Error>
    where
        S: Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send + 'static,
    {
        let response = self
            .client
            .send_multipart_stream(
                append_path_segments(&self.client.base_url, &["uploads", upload_id, "parts"])?,
                &options.headers,
                "data",
                None,
                "application/octet-stream",
                contents,
                Vec::new(),
                ACCEPT_JSON,
            )
            .await?;
        self.client.decode(response).await
    }

    /// Completes an upload using part IDs in the intended byte order.
    pub async fn complete(
        &self,
        upload_id: &str,
        request: &types::uploads::CompleteUploadRequest,
    ) -> Result<ApiResponse<types::uploads::Upload>, Error> {
        self.complete_with_options(upload_id, request, RequestOptions::default())
            .await
    }

    /// Completes an upload with per-request headers.
    pub async fn complete_with_options(
        &self,
        upload_id: &str,
        request: &types::uploads::CompleteUploadRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::uploads::Upload>, Error> {
        send_typed(
            self.client,
            Method::POST,
            append_path_segments(&self.client.base_url, &["uploads", upload_id, "complete"])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Cancels an upload that has not been completed.
    pub async fn cancel(
        &self,
        upload_id: &str,
    ) -> Result<ApiResponse<types::uploads::Upload>, Error> {
        self.cancel_with_options(upload_id, RequestOptions::default())
            .await
    }

    /// Cancels an upload with per-request headers.
    pub async fn cancel_with_options(
        &self,
        upload_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::uploads::Upload>, Error> {
        send_typed(
            self.client,
            Method::POST,
            append_path_segments(&self.client.base_url, &["uploads", upload_id, "cancel"])?,
            None,
            &options.headers,
        )
        .await
    }
}

/// Webhooks API operations for an [`OpenAIClient`].
pub struct WebhooksApi<'a> {
    client: &'a OpenAIClient,
}

impl WebhooksApi<'_> {
    fn endpoint(&self, endpoint_id: Option<&str>, suffix: &[&str]) -> Result<Url, Error> {
        let mut segments = vec!["webhook_endpoints"];
        if let Some(endpoint_id) = endpoint_id {
            segments.push(endpoint_id);
        }
        segments.extend_from_slice(suffix);
        append_path_segments(&self.client.base_url, &segments)
    }

    /// Lists one page of webhook endpoints.
    pub async fn list(
        &self,
        params: &types::webhooks::ListWebhookEndpointsParams,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointListResource>, Error> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// Lists webhook endpoints with per-request headers.
    pub async fn list_with_options(
        &self,
        params: &types::webhooks::ListWebhookEndpointsParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointListResource>, Error> {
        let mut endpoint = self.endpoint(None, &[])?;
        append_query(&mut endpoint, params)?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }

    /// Streams pages of webhook endpoints, preserving each response and request ID.
    pub fn list_pages<'a>(
        &'a self,
        params: types::webhooks::ListWebhookEndpointsParams,
    ) -> impl Stream<Item = Result<ApiResponse<types::webhooks::WebhookEndpointListResource>, Error>>
    + Send
    + 'a {
        async_stream::try_stream! {
            let mut params = params;
            loop {
                let page = self.list(&params).await?;
                let has_more = page.has_more;
                let last_id = page.last_id.clone();
                yield page;
                let Some(next) = next_webhook_endpoint_params(&params, has_more, last_id.as_deref())? else {
                    break;
                };
                params = next;
            }
        }
    }

    /// Creates a webhook endpoint and returns its signing secret once.
    pub async fn create(
        &self,
        request: &types::webhooks::PublicCreateEndpointBody,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointWithSecretResource>, Error> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    /// Creates an endpoint with per-request headers.
    pub async fn create_with_options(
        &self,
        request: &types::webhooks::PublicCreateEndpointBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointWithSecretResource>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(None, &[])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Retrieves webhook endpoint configuration.
    pub async fn retrieve(
        &self,
        endpoint_id: &str,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointBody>, Error> {
        self.retrieve_with_options(endpoint_id, RequestOptions::default())
            .await
    }

    /// Retrieves an endpoint with per-request headers.
    pub async fn retrieve_with_options(
        &self,
        endpoint_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointBody>, Error> {
        send_typed(
            self.client,
            Method::GET,
            self.endpoint(Some(endpoint_id), &[])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Updates endpoint name, URL, or subscribed event types.
    pub async fn update(
        &self,
        endpoint_id: &str,
        request: &types::webhooks::PublicUpdateEndpointBody,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointBody>, Error> {
        self.update_with_options(endpoint_id, request, RequestOptions::default())
            .await
    }

    /// Updates an endpoint with per-request headers.
    pub async fn update_with_options(
        &self,
        endpoint_id: &str,
        request: &types::webhooks::PublicUpdateEndpointBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointBody>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(endpoint_id), &[])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Deletes a webhook endpoint.
    pub async fn delete(
        &self,
        endpoint_id: &str,
    ) -> Result<ApiResponse<types::webhooks::DeletedWebhookEndpointResource>, Error> {
        self.delete_with_options(endpoint_id, RequestOptions::default())
            .await
    }

    /// Deletes an endpoint with per-request headers.
    pub async fn delete_with_options(
        &self,
        endpoint_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::webhooks::DeletedWebhookEndpointResource>, Error> {
        send_typed(
            self.client,
            Method::DELETE,
            self.endpoint(Some(endpoint_id), &[])?,
            None,
            &options.headers,
        )
        .await
    }

    /// Rotates an endpoint signing secret and returns the new secret once.
    pub async fn rotate_secret(
        &self,
        endpoint_id: &str,
        request: &types::webhooks::PublicRotateSecretBody,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointWithSecretResource>, Error> {
        self.rotate_secret_with_options(endpoint_id, request, RequestOptions::default())
            .await
    }

    /// Rotates the signing secret with per-request headers.
    pub async fn rotate_secret_with_options(
        &self,
        endpoint_id: &str,
        request: &types::webhooks::PublicRotateSecretBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointWithSecretResource>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(endpoint_id), &["rotate_secret"])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Sends a test event to a webhook endpoint.
    pub async fn test(
        &self,
        endpoint_id: &str,
        request: &types::webhooks::PublicTestEndpointBody,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointTestResultResource>, Error> {
        self.test_with_options(endpoint_id, request, RequestOptions::default())
            .await
    }

    /// Tests an endpoint with per-request headers.
    pub async fn test_with_options(
        &self,
        endpoint_id: &str,
        request: &types::webhooks::PublicTestEndpointBody,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::webhooks::WebhookEndpointTestResultResource>, Error> {
        send_typed(
            self.client,
            Method::POST,
            self.endpoint(Some(endpoint_id), &["test"])?,
            Some(serde_json::to_value(request)?),
            &options.headers,
        )
        .await
    }

    /// Lists event types available to the authenticated project.
    pub async fn list_event_types(
        &self,
    ) -> Result<ApiResponse<types::webhooks::WebhookEventTypeListResource>, Error> {
        self.list_event_types_with_options(RequestOptions::default())
            .await
    }

    /// Lists event types with per-request headers.
    pub async fn list_event_types_with_options(
        &self,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::webhooks::WebhookEventTypeListResource>, Error> {
        let endpoint = append_path_segments(&self.client.base_url, &["webhook_event_types"])?;
        send_typed(self.client, Method::GET, endpoint, None, &options.headers).await
    }
}

/// Incremental content returned by [`FilesApi::download`].
pub struct FileContentStream {
    inner: Pin<Box<dyn Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send>>,
    request_id: Option<String>,
    idle_timeout: Option<Duration>,
    terminated: bool,
}

impl FileContentStream {
    /// Returns the server request ID, if the response included one.
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    /// Reads the next content chunk. Dropping this value cancels remaining reads.
    pub async fn next_chunk(&mut self) -> Option<Result<bytes::Bytes, Error>> {
        if self.terminated {
            return None;
        }
        let next = if let Some(timeout) = self.idle_timeout {
            match tokio::time::timeout(timeout, self.inner.next()).await {
                Ok(next) => next,
                Err(_) => {
                    self.terminated = true;
                    return Some(Err(Error::StreamIdleTimeout));
                }
            }
        } else {
            self.inner.next().await
        };
        match next {
            Some(Ok(bytes)) => Some(Ok(bytes)),
            Some(Err(error)) => {
                self.terminated = true;
                Some(Err(Error::Http(error)))
            }
            None => {
                self.terminated = true;
                None
            }
        }
    }
}

/// Chat Completions API operations for an [`OpenAIClient`].
pub struct ChatCompletionsApi<'a> {
    client: &'a OpenAIClient,
}

impl ChatCompletionsApi<'_> {
    pub async fn list(
        &self,
        params: &types::chat_completions::ListChatCompletionsParams,
    ) -> Result<ApiResponse<types::chat_completions::ChatCompletionList>, Error> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    pub async fn list_with_options(
        &self,
        params: &types::chat_completions::ListChatCompletionsParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::chat_completions::ChatCompletionList>, Error> {
        let mut endpoint = append_path_segments(&self.client.base_url, &["chat", "completions"])?;
        append_query(&mut endpoint, params)?;
        let response = self
            .client
            .send(Method::GET, endpoint, None, ACCEPT_JSON, &options.headers)
            .await?;
        self.client.decode(response).await
    }

    pub async fn create(
        &self,
        request: &types::chat_completions::CreateChatCompletionRequest,
    ) -> Result<ApiResponse<types::chat_completions::CreateChatCompletionResponse>, Error> {
        self.create_with_options(request, RequestOptions::default())
            .await
    }

    pub async fn create_with_options(
        &self,
        request: &types::chat_completions::CreateChatCompletionRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::chat_completions::CreateChatCompletionResponse>, Error> {
        let mut request = request.clone();
        request.stream = Some(false);
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(&self.client.base_url, &["chat", "completions"])?,
                Some(serde_json::to_value(request)?),
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    /// The returned stream completes when the server sends `data: [DONE]`.
    pub async fn create_stream(
        &self,
        request: &types::chat_completions::CreateChatCompletionRequest,
    ) -> Result<ChatCompletionStream, Error> {
        self.create_stream_with_options(request, RequestOptions::default())
            .await
    }

    pub async fn create_stream_with_options(
        &self,
        request: &types::chat_completions::CreateChatCompletionRequest,
        options: RequestOptions,
    ) -> Result<ChatCompletionStream, Error> {
        let mut request = request.clone();
        request.stream = Some(true);
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(&self.client.base_url, &["chat", "completions"])?,
                Some(serde_json::to_value(request)?),
                ACCEPT_EVENTS,
                &options.headers,
            )
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        Ok(ChatCompletionStream::new(
            response,
            self.client.stream_idle_timeout,
        ))
    }

    pub async fn retrieve(
        &self,
        completion_id: &str,
    ) -> Result<ApiResponse<types::chat_completions::CreateChatCompletionResponse>, Error> {
        self.retrieve_with_options(completion_id, RequestOptions::default())
            .await
    }

    pub async fn retrieve_with_options(
        &self,
        completion_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::chat_completions::CreateChatCompletionResponse>, Error> {
        let response = self
            .client
            .send(
                Method::GET,
                append_path_segments(
                    &self.client.base_url,
                    &["chat", "completions", completion_id],
                )?,
                None,
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    pub async fn update(
        &self,
        completion_id: &str,
        request: &types::chat_completions::UpdateChatCompletionRequest,
    ) -> Result<ApiResponse<types::chat_completions::CreateChatCompletionResponse>, Error> {
        self.update_with_options(completion_id, request, RequestOptions::default())
            .await
    }

    pub async fn update_with_options(
        &self,
        completion_id: &str,
        request: &types::chat_completions::UpdateChatCompletionRequest,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::chat_completions::CreateChatCompletionResponse>, Error> {
        let response = self
            .client
            .send(
                Method::POST,
                append_path_segments(
                    &self.client.base_url,
                    &["chat", "completions", completion_id],
                )?,
                Some(serde_json::to_value(request)?),
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    pub async fn delete(
        &self,
        completion_id: &str,
    ) -> Result<ApiResponse<types::chat_completions::ChatCompletionDeleted>, Error> {
        self.delete_with_options(completion_id, RequestOptions::default())
            .await
    }

    pub async fn delete_with_options(
        &self,
        completion_id: &str,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::chat_completions::ChatCompletionDeleted>, Error> {
        let response = self
            .client
            .send(
                Method::DELETE,
                append_path_segments(
                    &self.client.base_url,
                    &["chat", "completions", completion_id],
                )?,
                None,
                ACCEPT_JSON,
                &options.headers,
            )
            .await?;
        self.client.decode(response).await
    }

    pub async fn list_messages(
        &self,
        completion_id: &str,
        params: &types::chat_completions::GetChatCompletionMessagesParams,
    ) -> Result<ApiResponse<types::chat_completions::ChatCompletionMessageList>, Error> {
        self.list_messages_with_options(completion_id, params, RequestOptions::default())
            .await
    }

    pub async fn list_messages_with_options(
        &self,
        completion_id: &str,
        params: &types::chat_completions::GetChatCompletionMessagesParams,
        options: RequestOptions,
    ) -> Result<ApiResponse<types::chat_completions::ChatCompletionMessageList>, Error> {
        let mut endpoint = append_path_segments(
            &self.client.base_url,
            &["chat", "completions", completion_id, "messages"],
        )?;
        append_query_omitting(&mut endpoint, params, &["completion_id"])?;
        let response = self
            .client
            .send(Method::GET, endpoint, None, ACCEPT_JSON, &options.headers)
            .await?;
        self.client.decode(response).await
    }
}

/// Per-call HTTP headers. Protocol headers managed by the SDK are rejected.
#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    pub headers: HeaderMap,
}

/// Retry configuration. Retries are disabled until a policy is set on the client builder.
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    max_retries: u8,
    initial_delay: Duration,
    max_delay: Duration,
}

impl RetryPolicy {
    /// Creates a policy with at most three retries, an initial backoff, and a delay cap.
    pub fn new(max_retries: u8, initial_delay: Duration, max_delay: Duration) -> Self {
        Self {
            max_retries: max_retries.min(3),
            initial_delay,
            max_delay,
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(3, Duration::from_millis(200), Duration::from_secs(30))
    }
}

impl RequestOptions {
    pub fn header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(name, value);
        self
    }

    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }
}

/// Per-call options for response creation.
#[derive(Clone, Debug, Default)]
pub struct CreateOptions {
    pub request: RequestOptions,
    pub extensions: Map<String, Value>,
}

impl CreateOptions {
    pub fn extension(mut self, name: impl Into<String>, value: Value) -> Self {
        self.extensions.insert(name.into(), value);
        self
    }

    pub fn request(mut self, request: RequestOptions) -> Self {
        self.request = request;
        self
    }
}

/// A typed API response paired with its original JSON and request ID.
#[derive(Clone, Debug, PartialEq)]
pub struct ApiResponse<T> {
    pub data: T,
    raw: Value,
    request_id: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn raw(&self) -> &Value {
        &self.raw
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl<T> Deref for ApiResponse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

const ACCEPT_JSON: &str = "application/json";
const ACCEPT_EVENTS: &str = "text/event-stream";
const ACCEPT_OCTET_STREAM: &str = "application/octet-stream";
const ACCEPT_AUDIO: &str = "audio/*, application/octet-stream";
#[cfg(feature = "multipart")]
const ACCEPT_TEXT: &str = "text/plain";

pub(crate) fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    status == reqwest::StatusCode::REQUEST_TIMEOUT
        || status == reqwest::StatusCode::TOO_MANY_REQUESTS
        || status.is_server_error()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RetryDecision {
    Stop,
    Retry(Option<Duration>),
}

async fn retry_request<T, E, Send, SendFuture, Decide, Delay>(
    max_retries: u8,
    mut send: Send,
    mut decide: Decide,
    mut delay: Delay,
) -> Result<T, E>
where
    Send: FnMut() -> SendFuture,
    SendFuture: Future<Output = Result<T, E>>,
    Decide: FnMut(&Result<T, E>) -> RetryDecision,
    Delay: FnMut(u8, Option<Duration>) -> Duration,
{
    for attempt in 0..=max_retries {
        let result = send().await;
        if attempt < max_retries
            && let RetryDecision::Retry(retry_after) = decide(&result)
        {
            tokio::time::sleep(delay(attempt + 1, retry_after)).await;
            continue;
        }
        return result;
    }
    unreachable!("retry loop always returns its final result")
}

fn retry_allowed(method: &Method, headers: &HeaderMap) -> bool {
    method == Method::GET
        || method == Method::HEAD
        || headers
            .get("idempotency-key")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| !value.trim().is_empty())
}

pub(crate) fn retry_after_delay(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(RETRY_AFTER)?.to_str().ok()?;
    parse_retry_after(value, SystemTime::now())
}

fn parse_retry_after(value: &str, now: SystemTime) -> Option<Duration> {
    let value = value.trim();
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    let date = parse_http_date(value, now)?;
    Some(date.duration_since(now).unwrap_or(Duration::ZERO))
}

fn parse_http_date(value: &str, now: SystemTime) -> Option<SystemTime> {
    if let Some((_, date)) = value.split_once(',') {
        let mut fields = date.split_ascii_whitespace();
        let first = fields.next()?;
        let (day, month, year) = if first.contains('-') {
            let mut date = first.split('-');
            let day = date.next()?.parse::<u32>().ok()?;
            let month = parse_month(date.next()?)?;
            let year = parse_two_digit_year(date.next()?.parse().ok()?, now)?;
            if date.next().is_some() {
                return None;
            }
            (day, month, year)
        } else {
            let day = first.parse::<u32>().ok()?;
            let month = parse_month(fields.next()?)?;
            let year = fields.next()?.parse::<i64>().ok()?;
            if !(1601..=9999).contains(&year) {
                return None;
            }
            (day, month, year)
        };
        let time = fields.next()?;
        if fields.next()? != "GMT" || fields.next().is_some() {
            return None;
        }
        return make_http_date(day, month, year, time);
    }

    // Obsolete asctime format remains part of the HTTP-date grammar.
    let mut fields = value.split_ascii_whitespace();
    let _weekday = fields.next()?;
    let month = parse_month(fields.next()?)?;
    let day = fields.next()?.parse::<u32>().ok()?;
    let time = fields.next()?;
    let year = fields.next()?.parse::<i64>().ok()?;
    if fields.next().is_some() || !(1601..=9999).contains(&year) {
        return None;
    }
    make_http_date(day, month, year, time)
}

fn parse_month(value: &str) -> Option<u32> {
    Some(match value {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => return None,
    })
}

fn parse_two_digit_year(year: u32, now: SystemTime) -> Option<i64> {
    if year > 99 {
        return None;
    }
    let now_days = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() / 86_400;
    let current_year = civil_year_from_days(i64::try_from(now_days).ok()?);
    let mut full_year = current_year / 100 * 100 + i64::from(year);
    if full_year > current_year + 50 {
        full_year -= 100;
    }
    Some(full_year)
}

fn civil_year_from_days(days: i64) -> i64 {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    year + i64::from(month_prime >= 10)
}

fn make_http_date(day: u32, month: u32, year: i64, time: &str) -> Option<SystemTime> {
    let mut clock = time.split(':');
    let hour = clock.next()?.parse::<u64>().ok()?;
    let minute = clock.next()?.parse::<u64>().ok()?;
    let second = clock.next()?.parse::<u64>().ok()?;
    if clock.next().is_some() || hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    let days = days_from_civil(year, month, day)?;
    let seconds = days
        .checked_mul(86_400)?
        .checked_add((hour * 3_600 + minute * 60 + second) as i64)?;
    if seconds < 0 {
        return UNIX_EPOCH.checked_sub(Duration::from_secs(seconds.unsigned_abs()));
    }
    UNIX_EPOCH.checked_add(Duration::from_secs(seconds as u64))
}

fn days_from_civil(year: i64, month: u32, day: u32) -> Option<i64> {
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => return None,
    };
    if day == 0 || day > days_in_month {
        return None;
    }
    let year = year.checked_sub(i64::from(month <= 2))?;
    let era = if year >= 0 {
        year
    } else {
        year.checked_sub(399)?
    } / 400;
    let year_of_era = year - era * 400;
    let month = i64::from(month);
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Some(era * 146_097 + day_of_era - 719_468)
}

fn jittered_backoff(policy: &RetryPolicy, retry_number: u8, seed: u64) -> Duration {
    let shift = u32::from(retry_number.saturating_sub(1).min(31));
    let multiplier = 1u32 << shift;
    let exponential = policy
        .initial_delay
        .checked_mul(multiplier)
        .unwrap_or(policy.max_delay)
        .min(policy.max_delay);
    let jitter_ppm = 500_000u128 + u128::from(seed % 1_000_001);
    let nanos = (exponential.as_nanos() * jitter_ppm / 1_000_000).min(policy.max_delay.as_nanos());
    Duration::new(
        (nanos / 1_000_000_000) as u64,
        (nanos % 1_000_000_000) as u32,
    )
}

fn validate_request_headers(headers: &HeaderMap) -> Result<(), Error> {
    for name in [AUTHORIZATION, CONTENT_TYPE, ACCEPT] {
        if headers.contains_key(&name) {
            return Err(Error::ReservedHeader(name.as_str().to_string()));
        }
    }
    Ok(())
}

#[derive(Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: String,
    default_headers: HeaderMap,
    connect_timeout: Option<Duration>,
    request_timeout: Option<Duration>,
    stream_idle_timeout: Option<Duration>,
    retry_policy: Option<RetryPolicy>,
}

impl ClientBuilder {
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the API root. For example, OpenAI uses `https://api.openai.com/v1`;
    /// DeepSeek uses `https://api.deepseek.com`.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers = headers;
        self
    }

    pub fn default_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.default_headers.insert(name, value);
        self
    }

    /// Sets a response timeout for JSON requests and for establishing a stream.
    /// The timeout does not limit the lifetime of an established stream.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }

    /// Sets the maximum time to establish a TCP/TLS connection.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Sets a response timeout for JSON requests and for establishing a stream.
    /// It does not limit the lifetime of an established stream.
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }

    /// Sets the maximum interval between SSE events or WebSocket messages. Disabled by default.
    pub fn stream_idle_timeout(mut self, timeout: Duration) -> Self {
        self.stream_idle_timeout = Some(timeout);
        self
    }

    /// Enables bounded retries for replayable requests that satisfy the policy.
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = Some(policy);
        self
    }

    pub fn build(self) -> Result<OpenAIClient, Error> {
        validate_request_headers(&self.default_headers)?;
        let api_key = self
            .api_key
            .filter(|key| !key.trim().is_empty())
            .ok_or(Error::MissingApiKey)?;
        let base = if self.base_url.trim().is_empty() {
            DEFAULT_BASE_URL
        } else {
            self.base_url.trim()
        };
        let mut base_url =
            Url::parse(base).map_err(|error| Error::InvalidBaseUrl(error.to_string()))?;
        if !matches!(base_url.scheme(), "http" | "https")
            || base_url.query().is_some()
            || base_url.fragment().is_some()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
        {
            return Err(Error::InvalidBaseUrl(base.to_string()));
        }
        let path = base_url.path().trim_end_matches('/').to_string();
        base_url.set_path(if path.is_empty() { "/" } else { &path });

        #[cfg(feature = "websocket")]
        let default_headers = self.default_headers.clone();
        let mut http = reqwest::Client::builder().default_headers(self.default_headers.clone());
        if let Some(timeout) = self.connect_timeout {
            http = http.connect_timeout(timeout);
        }

        Ok(OpenAIClient {
            http: http.build()?,
            api_key,
            base_url,
            #[cfg(feature = "websocket")]
            default_headers,
            #[cfg(feature = "websocket")]
            connect_timeout: self.connect_timeout,
            request_timeout: self.request_timeout,
            stream_idle_timeout: self.stream_idle_timeout,
            retry_policy: self.retry_policy,
        })
    }
}

fn request_body(
    request: &types::CreateResponse,
    extensions: Map<String, Value>,
) -> Result<Value, Error> {
    let mut value = serde_json::to_value(request)?;
    let body = value
        .as_object_mut()
        .expect("a generated CreateResponse serializes to a JSON object");
    for (key, value) in extensions {
        if body.contains_key(&key) {
            return Err(Error::ExtensionFieldConflict(key));
        }
        body.insert(key, value);
    }
    Ok(value)
}

fn next_input_page_params(
    params: &types::ListInputItemsParams,
    has_more: bool,
    last_id: &str,
) -> Result<Option<types::ListInputItemsParams>, Error> {
    if !has_more {
        return Ok(None);
    }
    if last_id.is_empty() || params.after.as_deref() == Some(last_id) {
        return Err(Error::InvalidPaginationCursor);
    }
    let mut next = params.clone();
    next.after = Some(last_id.to_owned());
    Ok(Some(next))
}

fn next_conversation_items_params(
    params: &types::conversations::ListConversationItemsParams,
    has_more: bool,
    last_id: &str,
) -> Result<Option<types::conversations::ListConversationItemsParams>, Error> {
    if !has_more {
        return Ok(None);
    }
    if last_id.is_empty() || params.after.as_deref() == Some(last_id) {
        return Err(Error::InvalidPaginationCursor);
    }
    let mut next = params.clone();
    next.after = Some(last_id.to_owned());
    Ok(Some(next))
}

fn next_container_params(
    params: &types::containers::ListContainersParams,
    has_more: bool,
    last_id: &str,
) -> Result<Option<types::containers::ListContainersParams>, Error> {
    if !has_more {
        return Ok(None);
    }
    if last_id.is_empty() || params.after.as_deref() == Some(last_id) {
        return Err(Error::InvalidPaginationCursor);
    }
    let mut next = params.clone();
    next.after = Some(last_id.to_owned());
    Ok(Some(next))
}

fn next_container_file_params(
    params: &types::containers::ListContainerFilesParams,
    has_more: bool,
    last_id: &str,
) -> Result<Option<types::containers::ListContainerFilesParams>, Error> {
    if !has_more {
        return Ok(None);
    }
    if last_id.is_empty() || params.after.as_deref() == Some(last_id) {
        return Err(Error::InvalidPaginationCursor);
    }
    let mut next = params.clone();
    next.after = Some(last_id.to_owned());
    Ok(Some(next))
}

fn next_batch_params(
    params: &types::batches::ListBatchesParams,
    has_more: bool,
    last_id: Option<&str>,
) -> Result<Option<types::batches::ListBatchesParams>, Error> {
    if !has_more {
        return Ok(None);
    }
    let Some(last_id) = last_id.filter(|last_id| !last_id.is_empty()) else {
        return Err(Error::InvalidPaginationCursor);
    };
    if params.after.as_deref() == Some(last_id) {
        return Err(Error::InvalidPaginationCursor);
    }
    let mut next = params.clone();
    next.after = Some(last_id.to_owned());
    Ok(Some(next))
}

fn next_voice_consent_params(
    params: &types::audio::ListVoiceConsentsParams,
    has_more: bool,
    last_id: Option<&str>,
) -> Result<Option<types::audio::ListVoiceConsentsParams>, Error> {
    if !has_more {
        return Ok(None);
    }
    let cursor = last_id
        .filter(|id| !id.trim().is_empty())
        .ok_or(Error::InvalidPaginationCursor)?;
    if params.after.as_deref() == Some(cursor) {
        return Err(Error::InvalidPaginationCursor);
    }
    Ok(Some(types::audio::ListVoiceConsentsParams {
        after: Some(cursor.to_owned()),
        limit: params.limit,
    }))
}

#[cfg(feature = "multipart")]
fn multipart_fields<T: Serialize>(
    request: &T,
    excluded: &[&str],
) -> Result<Vec<(String, String)>, Error> {
    let value = serde_json::to_value(request)?;
    let Value::Object(fields) = value else {
        return Err(Error::InvalidQueryParameter("multipart request".to_owned()));
    };
    let mut output = Vec::new();
    for (name, value) in fields {
        if excluded.contains(&name.as_str()) || value.is_null() {
            continue;
        }
        match value {
            Value::Array(values) => {
                for value in values {
                    output.push((name.clone(), multipart_scalar(value)?));
                }
            }
            value => output.push((name, multipart_scalar(value)?)),
        }
    }
    Ok(output)
}

#[cfg(feature = "multipart")]
fn multipart_scalar(value: Value) -> Result<String, Error> {
    match value {
        Value::String(value) => Ok(value),
        Value::Number(value) => Ok(value.to_string()),
        Value::Bool(value) => Ok(value.to_string()),
        Value::Null => Ok(String::new()),
        value => Ok(serde_json::to_string(&value)?),
    }
}

#[cfg(feature = "multipart")]
async fn read_audio_text(response: reqwest::Response) -> Result<AudioTextResponse, Error> {
    if !response.status().is_success() {
        return Err(read_http_error(response).await);
    }
    let request_id = request_id(response.headers());
    let bytes = response.bytes().await?;
    Ok(AudioTextResponse {
        text: String::from_utf8_lossy(&bytes).into_owned(),
        request_id,
    })
}

fn next_webhook_endpoint_params(
    params: &types::webhooks::ListWebhookEndpointsParams,
    has_more: bool,
    last_id: Option<&str>,
) -> Result<Option<types::webhooks::ListWebhookEndpointsParams>, Error> {
    if !has_more {
        return Ok(None);
    }
    let Some(last_id) = last_id.filter(|last_id| !last_id.is_empty()) else {
        return Err(Error::InvalidPaginationCursor);
    };
    if params.after.as_deref() == Some(last_id) {
        return Err(Error::InvalidPaginationCursor);
    }
    let mut next = params.clone();
    next.after = Some(last_id.to_owned());
    Ok(Some(next))
}

fn responses_url(base_url: &Url) -> Result<Url, Error> {
    append_path_segments(base_url, &["responses"])
}

fn responses_subresource_url(base_url: &Url, resource: &str) -> Result<Url, Error> {
    append_path_segments(base_url, &["responses", resource])
}

fn response_url(base_url: &Url, response_id: &str, suffix: &[&str]) -> Result<Url, Error> {
    let mut segments = vec!["responses", response_id];
    segments.extend_from_slice(suffix);
    append_path_segments(base_url, &segments)
}

fn response_stream_url(
    base_url: &Url,
    response_id: &str,
    params: &types::RetrieveResponseStreamParams,
) -> Result<Url, Error> {
    let mut endpoint = response_url(base_url, response_id, &[])?;
    append_query(&mut endpoint, params)?;
    endpoint.query_pairs_mut().append_pair("stream", "true");
    Ok(endpoint)
}

fn append_path_segments(base_url: &Url, segments: &[&str]) -> Result<Url, Error> {
    let mut endpoint = base_url.clone();
    let mut path = endpoint
        .path_segments_mut()
        .map_err(|_| Error::InvalidBaseUrl(base_url.to_string()))?;
    path.pop_if_empty();
    path.extend(segments.iter().copied());
    drop(path);
    Ok(endpoint)
}

fn append_query<T: Serialize>(endpoint: &mut Url, params: &T) -> Result<(), Error> {
    append_query_omitting(endpoint, params, &[])
}

fn append_query_omitting<T: Serialize>(
    endpoint: &mut Url,
    params: &T,
    omitted: &[&str],
) -> Result<(), Error> {
    let params = serde_json::to_value(params)?;
    let Some(params) = params.as_object() else {
        return Ok(());
    };
    let mut query = endpoint.query_pairs_mut();
    for (key, value) in params {
        if omitted.contains(&key.as_str()) {
            continue;
        }
        append_query_value(&mut query, key, value)?;
    }
    Ok(())
}

fn append_query_value(
    query: &mut url::form_urlencoded::Serializer<'_, url::UrlQuery>,
    key: &str,
    value: &Value,
) -> Result<(), Error> {
    let value = match value {
        Value::String(value) => value.clone(),
        Value::Number(value) => value.to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Null => return Ok(()),
        Value::Array(values) => {
            for value in values {
                append_query_value(query, key, value)?;
            }
            return Ok(());
        }
        Value::Object(values) => {
            for (nested_key, value) in values {
                append_query_value(query, &format!("{key}[{nested_key}]"), value)?;
            }
            return Ok(());
        }
    };
    query.append_pair(key, &value);
    Ok(())
}

#[cfg(feature = "multipart")]
struct MultipartFilePart {
    field_name: String,
    filename: Option<String>,
    mime_type: String,
    contents: bytes::Bytes,
}

#[cfg(feature = "multipart")]
#[cfg(test)]
fn multipart_body(
    filename: String,
    contents: bytes::Bytes,
    text_fields: Vec<(String, String)>,
) -> Result<(String, bytes::Bytes), Error> {
    multipart_body_part("file", Some(&filename), contents, text_fields)
}

#[cfg(feature = "multipart")]
#[cfg(test)]
fn multipart_body_part(
    field_name: &str,
    filename: Option<&str>,
    contents: bytes::Bytes,
    text_fields: Vec<(String, String)>,
) -> Result<(String, bytes::Bytes), Error> {
    multipart_body_files(
        vec![MultipartFilePart {
            field_name: field_name.to_owned(),
            filename: filename.map(str::to_owned),
            mime_type: "application/octet-stream".to_owned(),
            contents,
        }],
        text_fields,
    )
}

#[cfg(feature = "multipart")]
fn multipart_body_files(
    file_parts: Vec<MultipartFilePart>,
    text_fields: Vec<(String, String)>,
) -> Result<(String, bytes::Bytes), Error> {
    fn quoted(value: &str) -> Result<String, Error> {
        if value.contains('\r') || value.contains('\n') {
            return Err(Error::InvalidMultipartHeader);
        }
        Ok(value.replace('\\', "\\\\").replace('"', "\\\""))
    }

    let boundary = format!(
        "openai-rs-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let mut body = bytes::BytesMut::new();
    for file in file_parts {
        let field_name = quoted(&file.field_name)?;
        let filename = file.filename.as_deref().map(quoted).transpose()?;
        let mime_type = quoted(&file.mime_type)?;
        let disposition = match filename {
            Some(filename) => format!(
                "Content-Disposition: form-data; name=\"{field_name}\"; filename=\"{filename}\""
            ),
            None => format!("Content-Disposition: form-data; name=\"{field_name}\""),
        };
        body.extend_from_slice(
            format!("--{boundary}\r\n{disposition}\r\nContent-Type: {mime_type}\r\n\r\n")
                .as_bytes(),
        );
        body.extend_from_slice(&file.contents);
        body.extend_from_slice(b"\r\n");
    }
    for (name, value) in text_fields {
        let name = quoted(&name)?;
        body.extend_from_slice(
            format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n")
                .as_bytes(),
        );
        body.extend_from_slice(value.as_bytes());
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    Ok((
        format!("multipart/form-data; boundary={boundary}"),
        body.freeze(),
    ))
}

#[cfg(feature = "multipart")]
fn multipart_stream_body<S>(
    field_name: &str,
    filename: Option<&str>,
    mime_type: &str,
    contents: S,
    text_fields: Vec<(String, String)>,
) -> Result<
    (
        String,
        impl Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send + 'static,
    ),
    Error,
>
where
    S: Stream<Item = Result<bytes::Bytes, std::io::Error>> + Send + 'static,
{
    fn quoted(value: &str) -> Result<String, Error> {
        if value.contains('\r') || value.contains('\n') {
            return Err(Error::InvalidMultipartHeader);
        }
        Ok(value.replace('\\', "\\\\").replace('"', "\\\""))
    }

    let boundary = format!(
        "openai-rs-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );
    let field_name = quoted(field_name)?;
    let filename = filename.map(quoted).transpose()?;
    let mime_type = quoted(mime_type)?;
    let disposition = match filename {
        Some(filename) => format!(
            "Content-Disposition: form-data; name=\"{field_name}\"; filename=\"{filename}\""
        ),
        None => format!("Content-Disposition: form-data; name=\"{field_name}\""),
    };
    let prefix = bytes::Bytes::from(format!(
        "--{boundary}\r\n{disposition}\r\nContent-Type: {mime_type}\r\n\r\n"
    ));
    let mut suffix = bytes::BytesMut::from(&b"\r\n"[..]);
    for (name, value) in text_fields {
        let name = quoted(&name)?;
        suffix.extend_from_slice(
            format!("--{boundary}\r\nContent-Disposition: form-data; name=\"{name}\"\r\n\r\n")
                .as_bytes(),
        );
        suffix.extend_from_slice(value.as_bytes());
        suffix.extend_from_slice(b"\r\n");
    }
    suffix.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());
    let suffix = suffix.freeze();
    let body = async_stream::try_stream! {
        yield prefix;
        let mut contents = Box::pin(contents);
        while let Some(chunk) = contents.next().await {
            yield chunk?;
        }
        yield suffix;
    };
    Ok((format!("multipart/form-data; boundary={boundary}"), body))
}

#[cfg(feature = "multipart")]
fn multipart_text_value(value: &Value) -> String {
    match value {
        Value::String(value) => value.clone(),
        Value::Number(value) => value.to_string(),
        Value::Bool(value) => value.to_string(),
        _ => value.to_string(),
    }
}

async fn decode_response<T: DeserializeOwned>(
    response: reqwest::Response,
) -> Result<ApiResponse<T>, Error> {
    if !response.status().is_success() {
        return Err(read_http_error(response).await);
    }
    let request_id = request_id(response.headers());
    let bytes = response.bytes().await?;
    let raw: Value = serde_json::from_slice(&bytes)?;
    let data = serde_json::from_value(raw.clone())?;
    Ok(ApiResponse {
        data,
        raw,
        request_id,
    })
}

async fn send_typed<T: DeserializeOwned>(
    client: &OpenAIClient,
    method: Method,
    endpoint: Url,
    body: Option<Value>,
    headers: &HeaderMap,
) -> Result<ApiResponse<T>, Error> {
    let response = client
        .send(method, endpoint, body, ACCEPT_JSON, headers)
        .await?;
    client.decode(response).await
}

async fn read_http_error(response: reqwest::Response) -> Error {
    let status = response.status();
    let request_id = request_id(response.headers());
    let retry_after = response
        .headers()
        .get(RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let body = response
        .bytes()
        .await
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .unwrap_or_else(|error| format!("failed to read error body: {error}"));
    Error::HttpStatus {
        status,
        request_id,
        retry_after,
        body,
    }
}

pub(crate) fn request_id(headers: &HeaderMap) -> Option<String> {
    ["x-request-id", "openai-request-id"]
        .into_iter()
        .find_map(|name| headers.get(name).and_then(|value| value.to_str().ok()))
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::{
        ClientBuilder, CreateOptions, FileContentStream, RequestOptions, RetryDecision,
        RetryPolicy, append_path_segments, append_query, append_query_omitting, days_from_civil,
        is_retryable_status, jittered_backoff, make_http_date, next_batch_params,
        next_container_file_params, next_container_params, next_conversation_items_params,
        next_input_page_params, next_webhook_endpoint_params, parse_http_date, parse_retry_after,
        request_body, response_stream_url, retry_allowed, retry_request, validate_request_headers,
    };
    use crate::Error;
    use crate::types;
    use bytes::Bytes;
    use futures_util::stream;
    use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
    use reqwest::{Method, StatusCode};
    use serde_json::{Map, json};
    use std::time::{Duration, UNIX_EPOCH};
    use url::Url;

    #[test]
    fn serializes_generated_request_and_keeps_compatible_extensions() {
        let request: types::CreateResponse = serde_json::from_value(json!({
            "model": "deepseek-flash",
            "input": "hello"
        }))
        .expect("the generated request type accepts a compatible provider model id");
        let body = request_body(
            &request,
            Map::from_iter([("provider_option".to_string(), json!(true))]),
        )
        .unwrap();

        assert_eq!(body["model"], "deepseek-flash");
        assert_eq!(body["input"], "hello");
        assert_eq!(body["provider_option"], true);
    }

    #[test]
    fn vector_store_requests_and_cursor_queries_match_openapi_shapes() {
        let create: types::vector_stores::CreateVectorStoreRequest =
            serde_json::from_value(json!({"name":"knowledge","file_ids":["file_1"]})).unwrap();
        let body = serde_json::to_value(create).unwrap();
        assert_eq!(body["name"], "knowledge");
        assert_eq!(body["file_ids"][0], "file_1");

        let search: types::vector_stores::VectorStoreSearchRequest =
            serde_json::from_value(json!({"query":"refund policy","max_num_results":5})).unwrap();
        let body = serde_json::to_value(search).unwrap();
        assert_eq!(body["query"], "refund policy");
        assert_eq!(body["max_num_results"], 5);

        let params = types::vector_stores::ListVectorStoresParams {
            after: Some("vs_cursor".into()),
            ..Default::default()
        };
        let mut endpoint = Url::parse("https://api.example.test/v1/vector_stores").unwrap();
        append_query(&mut endpoint, &params).unwrap();
        assert_eq!(endpoint.query(), Some("after=vs_cursor"));
    }

    #[test]
    fn vector_store_content_and_search_pages_preserve_cursor_fields() {
        let content: types::vector_stores::VectorStoreFileContentResponse =
            serde_json::from_value(json!({
                "object":"vector_store.file_content.page",
                "data":[{"type":"text","text":"chunk"}],
                "has_more":true,
                "next_page":"content_cursor"
            }))
            .unwrap();
        assert_eq!(content.next_page.as_deref(), Some("content_cursor"));
        assert_eq!(content.data[0].text.as_deref(), Some("chunk"));

        let results: types::vector_stores::VectorStoreSearchResultsPage =
            serde_json::from_value(json!({
                "object":"vector_store.search_results.page",
                "search_query":["refund"],
                "data":[],
                "has_more":true,
                "next_page":"search_cursor"
            }))
            .unwrap();
        assert_eq!(results.next_page.as_deref(), Some("search_cursor"));
    }

    #[test]
    fn fine_tuning_job_requests_and_paginated_responses_match_schema() {
        let request: types::fine_tuning::CreateFineTuningJobRequest =
            serde_json::from_value(json!({
                "model":"gpt-4o-mini",
                "training_file":"file_train"
            }))
            .unwrap();
        let body = serde_json::to_value(request).unwrap();
        assert_eq!(body["model"], "gpt-4o-mini");
        assert_eq!(body["training_file"], "file_train");

        let page: types::fine_tuning::ListPaginatedFineTuningJobsResponse =
            serde_json::from_value(json!({
                "object":"list",
                "data":[{"id":"ftjob_1","object":"fine_tuning.job","created_at":1,
                    "model":"gpt-4o-mini","organization_id":"org_1","status":"queued",
                    "training_file":"file_train","result_files":[],"error":{},"fine_tuned_model":null,
                    "finished_at":null,"seed":null,"trained_tokens":null,"validation_file":null} ],
                "has_more":true
            }))
            .unwrap();
        assert_eq!(page.data[0].id, "ftjob_1");
        assert!(page.has_more);
    }

    #[test]
    fn live_sip_request_types_serialize_and_paths_keep_session_ids_opaque() {
        let refer: types::live::LiveCallReferRequest =
            serde_json::from_value(json!({"target_uri":"sip:agent@example.com"})).unwrap();
        let body = serde_json::to_value(refer).unwrap();
        assert_eq!(body["target_uri"], "sip:agent@example.com");

        let client = ClientBuilder::default().api_key("test").build().unwrap();
        let endpoint = client.live().endpoint(Some("call/a"), &["refer"]).unwrap();
        assert!(endpoint.path().ends_with("/live/sessions/call%2Fa/refer"));
    }

    #[test]
    fn realtime_sdp_and_client_secret_types_serialize() {
        let call: types::realtime::RealtimeCallCreateRequest = serde_json::from_value(json!({
            "sdp":"v=0\\r\\n",
            "session":null
        }))
        .unwrap();
        let body = serde_json::to_value(call).unwrap();
        assert_eq!(body["sdp"], "v=0\\r\\n");

        let secret: types::realtime::RealtimeCreateClientSecretRequest =
            serde_json::from_value(json!({})).unwrap();
        assert_eq!(serde_json::to_value(secret).unwrap(), json!({}));
    }

    #[test]
    fn rejects_request_extensions_that_shadow_typed_fields() {
        let request: types::CreateResponse = serde_json::from_value(json!({
            "model": "gpt-5.4",
            "input": "hello"
        }))
        .unwrap();
        let extensions = Map::from_iter([("model".to_string(), json!("other-model"))]);
        assert!(matches!(
            request_body(&request, extensions),
            Err(Error::ExtensionFieldConflict(field)) if field == "model"
        ));
    }

    #[test]
    fn builds_endpoint_paths_without_losing_base_url_prefix() {
        let openai = Url::parse("https://api.openai.com/v1").unwrap();
        let deepseek = Url::parse("https://api.deepseek.com").unwrap();
        assert_eq!(
            append_path_segments(&openai, &["responses"])
                .unwrap()
                .as_str(),
            "https://api.openai.com/v1/responses"
        );
        assert_eq!(
            append_path_segments(&deepseek, &["responses", "resp_1", "input_items"])
                .unwrap()
                .as_str(),
            "https://api.deepseek.com/responses/resp_1/input_items"
        );
    }

    #[test]
    fn encodes_typed_query_values_using_openapi_form_style() {
        let params = types::ListInputItemsParams {
            after: Some("item 2".to_string()),
            include: vec![
                types::IncludeEnum::MessageInputImageImageUrl,
                types::IncludeEnum::ReasoningEncryptedContent,
            ],
            limit: Some(12),
            order: Some(types::ListInputItemsParamsOrder::Asc),
        };
        let mut endpoint =
            Url::parse("https://api.openai.com/v1/responses/resp_1/input_items").unwrap();
        append_query(&mut endpoint, &params).unwrap();
        let pairs = endpoint.query_pairs().collect::<Vec<_>>();
        assert!(pairs.contains(&("after".into(), "item 2".into())));
        assert!(pairs.contains(&("limit".into(), "12".into())));
        assert!(pairs.contains(&("order".into(), "asc".into())));
        let include_values = pairs
            .iter()
            .filter(|(key, _)| key == "include")
            .map(|(_, value)| value.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(
            include_values,
            [
                "message.input_image.image_url",
                "reasoning.encrypted_content"
            ]
        );
    }

    #[test]
    fn encodes_metadata_query_as_bracketed_key_value_pairs() {
        let mut endpoint = Url::parse("https://api.openai.com/v1/chat/completions").unwrap();
        append_query(
            &mut endpoint,
            &json!({"metadata": {"key1": "value1", "key2": "value2"}}),
        )
        .unwrap();
        let pairs = endpoint.query_pairs().collect::<Vec<_>>();
        assert!(pairs.contains(&("metadata[key1]".into(), "value1".into())));
        assert!(pairs.contains(&("metadata[key2]".into(), "value2".into())));
    }

    #[test]
    fn encodes_files_list_query_parameters() {
        let params = types::files::ListFilesParams {
            after: Some("file cursor".to_owned()),
            limit: Some(25),
            order: Some(types::files::ListFilesParamsOrder::Asc),
            purpose: Some("user_data".to_owned()),
        };
        let mut endpoint = Url::parse("https://api.openai.com/v1/files").unwrap();
        append_query(&mut endpoint, &params).unwrap();
        let pairs = endpoint.query_pairs().collect::<Vec<_>>();
        assert!(pairs.contains(&("after".into(), "file cursor".into())));
        assert!(pairs.contains(&("limit".into(), "25".into())));
        assert!(pairs.contains(&("order".into(), "asc".into())));
        assert!(pairs.contains(&("purpose".into(), "user_data".into())));
    }

    #[test]
    fn encodes_conversation_item_query_parameters() {
        let params = types::conversations::ListConversationItemsParams {
            after: Some("item cursor".to_owned()),
            conversation_id: "conv_1".to_owned(),
            include: vec![types::conversations::IncludeEnum::ReasoningEncryptedContent],
            limit: Some(15),
            order: Some(types::conversations::ListConversationItemsParamsOrder::Asc),
        };
        let mut endpoint =
            Url::parse("https://api.openai.com/v1/conversations/conv_1/items").unwrap();
        append_query_omitting(&mut endpoint, &params, &["conversation_id"]).unwrap();
        let pairs = endpoint.query_pairs().collect::<Vec<_>>();
        assert!(pairs.contains(&("after".into(), "item cursor".into())));
        assert!(pairs.contains(&("limit".into(), "15".into())));
        assert!(pairs.contains(&("order".into(), "asc".into())));
        assert!(pairs.contains(&("include".into(), "reasoning.encrypted_content".into())));
        assert!(!pairs.iter().any(|(key, _)| key == "conversation_id"));
    }

    #[test]
    fn encodes_container_list_query_parameters() {
        let params = types::containers::ListContainersParams {
            after: Some("container cursor".to_owned()),
            limit: Some(12),
            name: Some("analysis".to_owned()),
            order: Some(types::containers::ListContainersParamsOrder::Desc),
        };
        let mut endpoint = Url::parse("https://api.openai.com/v1/containers").unwrap();
        append_query(&mut endpoint, &params).unwrap();
        let pairs = endpoint.query_pairs().collect::<Vec<_>>();
        assert!(pairs.contains(&("after".into(), "container cursor".into())));
        assert!(pairs.contains(&("limit".into(), "12".into())));
        assert!(pairs.contains(&("name".into(), "analysis".into())));
        assert!(pairs.contains(&("order".into(), "desc".into())));

        let params = types::containers::ListContainerFilesParams {
            after: Some("file cursor".to_owned()),
            container_id: "c_1".to_owned(),
            limit: Some(7),
            order: Some(types::containers::ListContainerFilesParamsOrder::Asc),
        };
        let mut endpoint = Url::parse("https://api.openai.com/v1/containers/c_1/files").unwrap();
        append_query_omitting(&mut endpoint, &params, &["container_id"]).unwrap();
        let pairs = endpoint.query_pairs().collect::<Vec<_>>();
        assert!(pairs.contains(&("after".into(), "file cursor".into())));
        assert!(pairs.contains(&("limit".into(), "7".into())));
        assert!(pairs.contains(&("order".into(), "asc".into())));
        assert!(!pairs.iter().any(|(key, _)| key == "container_id"));
    }

    #[test]
    fn generated_container_requests_serialize() {
        let request: types::containers::CreateContainerBody = serde_json::from_value(json!({
            "name": "analysis",
            "file_ids": ["file_1"]
        }))
        .unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap()["file_ids"][0],
            "file_1"
        );

        let attach: types::containers::CreateContainerFileBody = serde_json::from_value(json!({
            "file_id": "file_1"
        }))
        .unwrap();
        assert_eq!(serde_json::to_value(attach).unwrap()["file_id"], "file_1");
    }

    #[test]
    fn generated_batch_request_and_cursor_serialize() {
        let request: types::batches::CreateBatchRequest = serde_json::from_value(json!({
            "input_file_id": "file_1",
            "endpoint": "/v1/responses",
            "completion_window": "24h"
        }))
        .unwrap();
        let body = serde_json::to_value(request).unwrap();
        assert_eq!(body["input_file_id"], "file_1");
        assert_eq!(body["completion_window"], "24h");

        let params = types::batches::ListBatchesParams {
            limit: Some(10),
            ..Default::default()
        };
        let next = next_batch_params(&params, true, Some("batch_2"))
            .unwrap()
            .unwrap();
        assert_eq!(next.after.as_deref(), Some("batch_2"));
        assert_eq!(next.limit, Some(10));
        assert!(matches!(
            next_batch_params(&params, true, None),
            Err(Error::InvalidPaginationCursor)
        ));
        assert!(matches!(
            next_batch_params(&next, true, Some("batch_2")),
            Err(Error::InvalidPaginationCursor)
        ));
    }

    #[test]
    fn generated_webhook_requests_and_cursor_serialize() {
        let create: types::webhooks::PublicCreateEndpointBody = serde_json::from_value(json!({
            "url": "https://example.com/webhooks",
            "name": "responses",
            "event_types": ["response.completed"]
        }))
        .unwrap();
        let body = serde_json::to_value(create).unwrap();
        assert_eq!(body["url"], "https://example.com/webhooks");
        assert_eq!(body["event_types"][0], "response.completed");

        let params = types::webhooks::ListWebhookEndpointsParams {
            limit: Some(7.try_into().unwrap()),
            ..Default::default()
        };
        let next = next_webhook_endpoint_params(&params, true, Some("whe_2"))
            .unwrap()
            .unwrap();
        assert_eq!(next.after.as_deref(), Some("whe_2"));
        assert_eq!(next.limit, params.limit);
        assert!(matches!(
            next_webhook_endpoint_params(&params, true, None),
            Err(Error::InvalidPaginationCursor)
        ));
        assert!(matches!(
            next_webhook_endpoint_params(&next, true, Some("whe_2")),
            Err(Error::InvalidPaginationCursor)
        ));
    }

    #[test]
    fn advances_container_and_container_file_cursors() {
        let containers = types::containers::ListContainersParams {
            name: Some("analysis".to_owned()),
            ..Default::default()
        };
        let next = next_container_params(&containers, true, "container_2")
            .unwrap()
            .unwrap();
        assert_eq!(next.after.as_deref(), Some("container_2"));
        assert_eq!(next.name.as_deref(), Some("analysis"));
        assert!(
            next_container_params(&containers, false, "container_2")
                .unwrap()
                .is_none()
        );

        let repeated = types::containers::ListContainersParams {
            after: Some("container_2".to_owned()),
            ..Default::default()
        };
        assert!(matches!(
            next_container_params(&repeated, true, "container_2"),
            Err(Error::InvalidPaginationCursor)
        ));

        let files = types::containers::ListContainerFilesParams {
            after: None,
            container_id: "c_1".to_owned(),
            limit: Some(8),
            order: None,
        };
        let next = next_container_file_params(&files, true, "file_2")
            .unwrap()
            .unwrap();
        assert_eq!(next.after.as_deref(), Some("file_2"));
        assert_eq!(next.limit, Some(8));
        assert!(matches!(
            next_container_file_params(&files, true, ""),
            Err(Error::InvalidPaginationCursor)
        ));
    }

    #[test]
    fn generated_conversation_types_serialize_and_deserialize() {
        let request: types::conversations::CreateConversationBody = serde_json::from_value(json!({
            "metadata": {"topic": "sdk"}
        }))
        .unwrap();
        assert_eq!(
            serde_json::to_value(request).unwrap()["metadata"]["topic"],
            "sdk"
        );

        let conversation: types::conversations::ConversationResource =
            serde_json::from_value(json!({
                "id": "conv_123",
                "object": "conversation",
                "created_at": 1,
                "metadata": {"topic": "sdk"}
            }))
            .unwrap();
        assert_eq!(conversation.id, "conv_123");
    }

    #[test]
    fn advances_conversation_cursor_and_rejects_empty_or_repeated_values() {
        let params = types::conversations::ListConversationItemsParams {
            after: None,
            conversation_id: "conv_1".to_owned(),
            include: vec![types::conversations::IncludeEnum::ReasoningEncryptedContent],
            limit: Some(9),
            order: None,
        };
        let next = next_conversation_items_params(&params, true, "item_2")
            .unwrap()
            .unwrap();
        assert_eq!(next.after.as_deref(), Some("item_2"));
        assert_eq!(next.limit, Some(9));
        assert_eq!(next.include, params.include);
        assert!(
            next_conversation_items_params(&params, false, "item_2")
                .unwrap()
                .is_none()
        );

        let repeated = types::conversations::ListConversationItemsParams {
            after: Some("item_2".to_owned()),
            conversation_id: "conv_1".to_owned(),
            include: Vec::new(),
            limit: None,
            order: None,
        };
        assert!(matches!(
            next_conversation_items_params(&repeated, true, "item_2"),
            Err(Error::InvalidPaginationCursor)
        ));
        assert!(matches!(
            next_conversation_items_params(&params, true, ""),
            Err(Error::InvalidPaginationCursor)
        ));
    }

    #[tokio::test]
    async fn file_download_stream_yields_chunks_and_request_id() {
        let mut download = FileContentStream {
            inner: Box::pin(stream::iter([
                Ok::<_, reqwest::Error>(Bytes::from_static(b"part-1")),
                Ok(Bytes::from_static(b"part-2")),
            ])),
            request_id: Some("req_file".to_owned()),
            idle_timeout: None,
            terminated: false,
        };
        assert_eq!(download.request_id(), Some("req_file"));
        assert_eq!(download.next_chunk().await.unwrap().unwrap(), "part-1");
        assert_eq!(download.next_chunk().await.unwrap().unwrap(), "part-2");
        assert!(download.next_chunk().await.is_none());
    }

    #[cfg(feature = "multipart")]
    #[test]
    fn creates_replayable_multipart_body_and_rejects_header_injection() {
        let (content_type, body) = super::multipart_body(
            "input.jsonl".to_owned(),
            Bytes::from_static(b"{\"input\":\"hello\"}"),
            vec![("purpose".to_owned(), "batch".to_owned())],
        )
        .unwrap();
        let boundary = content_type
            .strip_prefix("multipart/form-data; boundary=")
            .unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();
        assert!(body.starts_with(&format!("--{boundary}\r\n")));
        assert!(body.contains("filename=\"input.jsonl\""));
        assert!(body.contains("name=\"purpose\"\r\n\r\nbatch"));
        assert!(body.ends_with(&format!("--{boundary}--\r\n")));
        assert!(matches!(
            super::multipart_body("unsafe\r\nname".to_owned(), Bytes::new(), vec![]),
            Err(Error::InvalidMultipartHeader)
        ));

        let (_, part_body) = super::multipart_body_part(
            "data",
            None,
            Bytes::from_static(b"chunk-bytes"),
            Vec::new(),
        )
        .unwrap();
        let part_body = String::from_utf8(part_body.to_vec()).unwrap();
        assert!(part_body.contains("Content-Disposition: form-data; name=\"data\"\r\n"));
        assert!(part_body.contains("Content-Type: application/octet-stream\r\n\r\nchunk-bytes"));
        assert!(!part_body.contains("filename="));

        let (_, audio_body) = super::multipart_body_files(
            vec![
                super::MultipartFilePart {
                    field_name: "recording".to_owned(),
                    filename: Some("consent.wav".to_owned()),
                    mime_type: "audio/wav".to_owned(),
                    contents: Bytes::from_static(b"consent-audio"),
                },
                super::MultipartFilePart {
                    field_name: "audio_sample".to_owned(),
                    filename: Some("sample.mp3".to_owned()),
                    mime_type: "audio/mpeg".to_owned(),
                    contents: Bytes::from_static(b"voice-sample"),
                },
            ],
            vec![("consent".to_owned(), "cons_1".to_owned())],
        )
        .unwrap();
        let audio_body = String::from_utf8(audio_body.to_vec()).unwrap();
        assert!(
            audio_body.contains(
                "name=\"recording\"; filename=\"consent.wav\"\r\nContent-Type: audio/wav"
            )
        );
        assert!(audio_body.contains(
            "name=\"audio_sample\"; filename=\"sample.mp3\"\r\nContent-Type: audio/mpeg"
        ));
        assert!(audio_body.contains("name=\"consent\"\r\n\r\ncons_1"));

        let (_, image_body) = super::multipart_body_files(
            vec![
                super::MultipartFilePart {
                    field_name: "image[]".to_owned(),
                    filename: Some("one.png".to_owned()),
                    mime_type: "image/png".to_owned(),
                    contents: Bytes::from_static(b"image-one"),
                },
                super::MultipartFilePart {
                    field_name: "image[]".to_owned(),
                    filename: Some("two.webp".to_owned()),
                    mime_type: "image/webp".to_owned(),
                    contents: Bytes::from_static(b"image-two"),
                },
                super::MultipartFilePart {
                    field_name: "mask".to_owned(),
                    filename: Some("mask.png".to_owned()),
                    mime_type: "image/png".to_owned(),
                    contents: Bytes::from_static(b"mask"),
                },
            ],
            vec![
                ("prompt".to_owned(), "make a watercolor".to_owned()),
                ("stream".to_owned(), "false".to_owned()),
            ],
        )
        .unwrap();
        let image_body = String::from_utf8(image_body.to_vec()).unwrap();
        assert_eq!(image_body.matches("name=\"image[]\"").count(), 2);
        assert!(image_body.contains("name=\"mask\"; filename=\"mask.png\""));
        assert!(image_body.contains("name=\"prompt\"\r\n\r\nmake a watercolor"));
        assert!(image_body.contains("name=\"stream\"\r\n\r\nfalse"));
    }

    #[cfg(feature = "multipart")]
    #[tokio::test]
    async fn one_shot_multipart_body_preserves_chunks_and_propagates_stream_errors() {
        use futures_util::StreamExt;

        let chunks = stream::iter(vec![
            Ok(Bytes::from_static(b"first-")),
            Ok(Bytes::from_static(b"second")),
        ]);
        let (content_type, body) = super::multipart_stream_body(
            "data",
            Some("large.jsonl"),
            "application/octet-stream",
            chunks,
            vec![("purpose".to_owned(), "fine-tune".to_owned())],
        )
        .unwrap();
        let chunks = body.collect::<Vec<_>>().await;
        let body = chunks
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .flat_map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();
        let body = String::from_utf8(body).unwrap();
        let boundary = content_type
            .strip_prefix("multipart/form-data; boundary=")
            .unwrap();
        assert!(body.starts_with(&format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"data\"; filename=\"large.jsonl\"\r\n"
        )));
        assert!(body.contains("first-second\r\n"));
        assert!(body.contains("name=\"purpose\"\r\n\r\nfine-tune\r\n"));
        assert!(body.ends_with(&format!("--{boundary}--\r\n")));

        let chunks = stream::iter(vec![Err(std::io::Error::other("fixture stream failure"))]);
        let (_, body) = super::multipart_stream_body(
            "data",
            None,
            "application/octet-stream",
            chunks,
            Vec::new(),
        )
        .unwrap();
        let chunks = body.collect::<Vec<_>>().await;
        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].is_ok());
        assert_eq!(
            chunks[1].as_ref().unwrap_err().to_string(),
            "fixture stream failure"
        );
    }

    #[test]
    fn generated_upload_requests_serialize_in_order() {
        let create: types::uploads::CreateUploadRequest = serde_json::from_value(json!({
            "bytes": 12,
            "filename": "training.jsonl",
            "mime_type": "text/jsonl",
            "purpose": "fine-tune"
        }))
        .unwrap();
        let create = serde_json::to_value(create).unwrap();
        assert_eq!(create["bytes"], 12);
        assert_eq!(create["purpose"], "fine-tune");

        let complete: types::uploads::CompleteUploadRequest = serde_json::from_value(json!({
            "part_ids": ["part_1", "part_2"]
        }))
        .unwrap();
        assert_eq!(complete.part_ids, ["part_1", "part_2"]);
    }

    #[cfg(feature = "multipart")]
    #[test]
    fn audio_requests_and_multipart_fields_serialize() {
        let transcription: types::audio::CreateTranscriptionRequest =
            serde_json::from_value(json!({
                "file": "schema-placeholder.wav",
                "model": "gpt-4o-transcribe",
                "include": ["logprobs"],
                "keywords": ["Rust", "coroutines"],
                "response_format": "json"
            }))
            .unwrap();
        let fields = super::multipart_fields(&transcription, &["file"]).unwrap();
        assert!(!fields.iter().any(|(name, _)| name == "file"));
        assert_eq!(
            fields.iter().filter(|(name, _)| name == "keywords").count(),
            2
        );
        assert!(
            fields
                .iter()
                .any(|(name, value)| name == "include" && value == "logprobs")
        );

        let speech = types::audio::CreateSpeechRequest {
            input: "hello".parse().unwrap(),
            instructions: None,
            model: "gpt-4o-mini-tts".to_owned(),
            response_format: None,
            speed: None,
            stream_format: Some(types::audio::CreateSpeechRequestStreamFormat::Sse),
            voice: types::audio::Voice::VoiceIdsShared(types::audio::VoiceIdsShared(
                "nova".to_owned(),
            )),
        };
        let value = serde_json::to_value(speech).unwrap();
        assert_eq!(value["input"], "hello");
        assert_eq!(value["stream_format"], "sse");
        assert_eq!(value["voice"], "nova");
    }

    #[test]
    fn advances_voice_consent_cursor_and_rejects_bad_cursor() {
        let params = types::audio::ListVoiceConsentsParams {
            limit: Some(10),
            ..Default::default()
        };
        let next = super::next_voice_consent_params(&params, true, Some("consent_2"))
            .unwrap()
            .unwrap();
        assert_eq!(next.after.as_deref(), Some("consent_2"));
        assert_eq!(next.limit, params.limit);
        assert!(matches!(
            super::next_voice_consent_params(&params, true, None),
            Err(Error::InvalidPaginationCursor)
        ));
        assert!(matches!(
            super::next_voice_consent_params(&next, true, Some("consent_2")),
            Err(Error::InvalidPaginationCursor)
        ));
    }

    #[test]
    fn advances_page_cursor_and_rejects_repeated_or_empty_cursors() {
        let params = types::ListInputItemsParams::default();
        let next = next_input_page_params(&params, true, "item_2")
            .unwrap()
            .unwrap();
        assert_eq!(next.after.as_deref(), Some("item_2"));
        assert!(
            next_input_page_params(&params, false, "item_2")
                .unwrap()
                .is_none()
        );

        let repeated = types::ListInputItemsParams {
            after: Some("item_2".to_owned()),
            ..Default::default()
        };
        assert!(matches!(
            next_input_page_params(&repeated, true, "item_2"),
            Err(Error::InvalidPaginationCursor)
        ));
        assert!(matches!(
            next_input_page_params(&params, true, ""),
            Err(Error::InvalidPaginationCursor)
        ));
    }

    #[test]
    fn parses_retry_after_seconds_and_http_date() {
        assert_eq!(
            parse_retry_after("120", UNIX_EPOCH),
            Some(Duration::from_secs(120))
        );
        assert_eq!(
            parse_retry_after("Sun, 06 Nov 1994 08:49:37 GMT", UNIX_EPOCH),
            Some(Duration::from_secs(784_111_777))
        );
        assert_eq!(
            parse_retry_after("Sunday, 06-Nov-94 08:49:37 GMT", UNIX_EPOCH),
            Some(Duration::from_secs(784_111_777))
        );
        assert_eq!(
            parse_retry_after("Sun Nov  6 08:49:37 1994", UNIX_EPOCH),
            Some(Duration::from_secs(784_111_777))
        );
        assert_eq!(
            parse_retry_after(
                "Sun, 06 Nov 1994 08:49:37 GMT",
                UNIX_EPOCH + Duration::from_secs(900_000_000)
            ),
            Some(Duration::ZERO)
        );
        let now_in_2076 = make_http_date(1, 1, 2076, "00:00:00").unwrap();
        assert_eq!(
            parse_retry_after("Sunday, 06-Nov-19 08:49:37 GMT", now_in_2076),
            Some(Duration::ZERO),
            "a date more than 50 years in the past keeps its stated past century"
        );
        assert_eq!(
            parse_http_date(
                "Sunday, 06-Nov-77 08:49:37 GMT",
                make_http_date(1, 1, 2026, "00:00:00").unwrap()
            ),
            UNIX_EPOCH.checked_add(Duration::from_secs(247_654_177)),
            "a date more than 50 years in the future maps to the latest matching past year"
        );
        assert_eq!(parse_retry_after("tomorrow", UNIX_EPOCH), None);
        assert_eq!(
            parse_retry_after("Sun, 31 Feb 1994 08:49:37 GMT", UNIX_EPOCH),
            None
        );
    }

    #[test]
    fn retry_policy_is_bounded_and_requires_idempotency_for_writes() {
        let policy = RetryPolicy::new(9, Duration::from_secs(2), Duration::from_secs(3));
        assert_eq!(policy.max_retries, 3);
        assert_eq!(
            jittered_backoff(&policy, 3, u64::MAX),
            Duration::from_secs(3)
        );
        assert!(retry_allowed(&Method::GET, &HeaderMap::new()));
        assert!(!retry_allowed(&Method::POST, &HeaderMap::new()));
        let mut headers = HeaderMap::new();
        headers.insert("idempotency-key", HeaderValue::from_static("idem_1"));
        assert!(retry_allowed(&Method::POST, &headers));
        assert!(!is_retryable_status(StatusCode::BAD_REQUEST));
        assert!(is_retryable_status(StatusCode::TOO_MANY_REQUESTS));
        assert!(is_retryable_status(StatusCode::INTERNAL_SERVER_ERROR));
        assert!(days_from_civil(1994, 11, 6).is_some());
    }

    #[tokio::test]
    async fn retry_request_makes_only_the_initial_attempt_plus_the_retry_limit() {
        let mut attempts = 0;
        let delays = std::cell::RefCell::new(Vec::new());
        let result = retry_request(
            3,
            || {
                attempts += 1;
                let attempt = attempts;
                std::future::ready(if attempt <= 3 {
                    Err(attempt)
                } else {
                    Ok(attempt)
                })
            },
            |result| match result {
                Err(_) => RetryDecision::Retry(None),
                Ok(_) => RetryDecision::Stop,
            },
            |retry_number, retry_after| {
                delays.borrow_mut().push((retry_number, retry_after));
                Duration::ZERO
            },
        )
        .await;

        assert_eq!(result, Ok(4));
        assert_eq!(attempts, 4);
        assert_eq!(*delays.borrow(), vec![(1, None), (2, None), (3, None)]);
    }

    #[tokio::test]
    async fn retry_request_stops_immediately_for_non_retryable_results() {
        let mut attempts = 0;
        let result: Result<(), &str> = retry_request(
            3,
            || {
                attempts += 1;
                std::future::ready(Err("permanent"))
            },
            |result| match result {
                Err("transient") => RetryDecision::Retry(None),
                _ => RetryDecision::Stop,
            },
            |_, _| Duration::ZERO,
        )
        .await;

        assert_eq!(result, Err("permanent"));
        assert_eq!(attempts, 1);
    }

    #[tokio::test]
    async fn retry_request_with_retries_disabled_makes_one_attempt() {
        let mut attempts = 0;
        let result: Result<(), usize> = retry_request(
            0,
            || {
                attempts += 1;
                std::future::ready(Err(attempts))
            },
            |_| RetryDecision::Retry(None),
            |_, _| Duration::ZERO,
        )
        .await;

        assert_eq!(result, Err(1));
        assert_eq!(attempts, 1);
    }

    #[test]
    fn resumes_response_stream_after_sequence_number() {
        let params = types::RetrieveResponseStreamParams {
            include: vec![],
            include_obfuscation: Some(false),
            starting_after: Some(7),
        };
        let endpoint = response_stream_url(
            &Url::parse("https://api.openai.com/v1").unwrap(),
            "resp_1",
            &params,
        )
        .unwrap();
        let pairs = endpoint.query_pairs().collect::<Vec<_>>();
        assert!(pairs.contains(&("starting_after".into(), "7".into())));
        assert!(pairs.contains(&("include_obfuscation".into(), "false".into())));
        assert!(pairs.contains(&("stream".into(), "true".into())));
    }

    #[test]
    fn encodes_response_ids_as_single_path_segments() {
        let endpoint = append_path_segments(
            &Url::parse("https://api.openai.com/v1").unwrap(),
            &["responses", "id/with?reserved"],
        )
        .unwrap();
        assert_eq!(endpoint.path(), "/v1/responses/id%2Fwith%3Freserved");
    }

    #[test]
    fn encodes_nested_object_query_values() {
        let mut endpoint = Url::parse("https://api.openai.com/v1/chat/completions").unwrap();
        append_query(
            &mut endpoint,
            &json!({"metadata": {"nested": {"value": true}}}),
        )
        .unwrap();
        assert!(
            endpoint
                .query_pairs()
                .any(|(key, value)| key == "metadata[nested][value]" && value == "true")
        );
    }

    #[test]
    fn generated_new_endpoint_types_deserialize() {
        let delete: types::DeleteResponse = serde_json::from_value(json!({
            "id": "resp_1",
            "object": "response.deleted",
            "deleted": true
        }))
        .unwrap();
        assert!(delete.deleted);

        let counts: types::TokenCountsResource = serde_json::from_value(json!({
            "object": "response.input_tokens",
            "input_tokens": 12
        }))
        .unwrap();
        assert_eq!(counts.input_tokens, 12);

        let list: types::ResponseItemList = serde_json::from_value(json!({
            "object": "list",
            "data": [],
            "first_id": "item_1",
            "last_id": "item_1",
            "has_more": true
        }))
        .unwrap();
        assert!(list.has_more);
    }

    #[test]
    fn generated_models_family_types_deserialize() {
        let models: types::models::ListModelsResponse = serde_json::from_value(json!({
            "object": "list",
            "data": [{
                "id": "gpt-6-astra",
                "object": "model",
                "created": 1686935002,
                "owned_by": "openai",
                "shutdown_date": "2026-10-23",
                "future_field": true
            }]
        }))
        .unwrap();
        assert_eq!(models.data[0].id, "gpt-6-astra");
        assert_eq!(models.data[0].created, Some(1686935002));
        assert_eq!(models.data[0].shutdown_date.as_deref(), Some("2026-10-23"));

        let compatible_models: types::models::ListModelsResponse = serde_json::from_value(json!({
            "object": "list",
            "data": [{
                "id": "deepseek-flash",
                "object": "model",
                "owned_by": "deepseek",
                "name": "DeepSeek-V4.1-Flash",
                "context_window": 1048576,
                "max_output_tokens": 393216,
                "input_modalities": ["text", "image"],
                "output_modalities": ["text"]
            }]
        }))
        .unwrap();
        assert_eq!(compatible_models.data[0].id, "deepseek-flash");
        assert_eq!(compatible_models.data[0].created, None);

        let compatible_file: types::files::OpenAiFile = serde_json::from_value(json!({
            "id": "file-api-example",
            "object": "file",
            "bytes": 43,
            "created_at": 1700000000,
            "filename": "openai-rs-smoke.gif",
            "purpose": "user_data",
            "expires_at": 1700003600
        }))
        .unwrap();
        assert_eq!(compatible_file.id, "file-api-example");
        assert_eq!(compatible_file.status, None);

        let deleted: types::models::DeleteModelResponse = serde_json::from_value(json!({
            "id": "ft:gpt-6-astra:example",
            "object": "model",
            "deleted": true
        }))
        .unwrap();
        assert!(deleted.deleted);
    }

    #[test]
    fn generated_embeddings_types_deserialize_and_serialize() {
        let request: types::embeddings::CreateEmbeddingRequest = serde_json::from_value(json!({
            "model": "text-embedding-3-small",
            "input": ["first", "second"]
        }))
        .unwrap();
        let body = serde_json::to_value(request).unwrap();
        assert_eq!(body["model"], "text-embedding-3-small");
        assert_eq!(body["input"][1], "second");

        let response: types::embeddings::CreateEmbeddingResponse = serde_json::from_value(json!({
            "object": "list",
            "model": "text-embedding-3-small",
            "data": [{"object": "embedding", "index": 0, "embedding": [0.1, 0.2]}],
            "usage": {"prompt_tokens": 2, "total_tokens": 2}
        }))
        .unwrap();
        assert_eq!(response.data[0].index, 0);
        assert_eq!(response.usage.total_tokens, 2);
    }

    #[test]
    fn generated_moderation_request_serializes_text_input() {
        let request: types::moderations::CreateModerationRequest =
            serde_json::from_value(json!({"input": "short neutral text"})).unwrap();
        let body = serde_json::to_value(request).unwrap();
        assert_eq!(body["input"], "short neutral text");
    }

    #[test]
    fn generated_content_provenance_request_serializes_file_name() {
        let request = types::content_provenance::CreateContentProvenanceBody {
            file: "image.png".to_owned(),
        };
        assert_eq!(serde_json::to_value(request).unwrap()["file"], "image.png");
    }

    #[test]
    fn generated_image_generation_and_json_edit_requests_serialize() {
        let generation: types::images::CreateImageRequest = serde_json::from_value(json!({
            "model": "gpt-image-1.5",
            "prompt": "A small blue kite in a clear sky"
        }))
        .unwrap();
        assert_eq!(
            serde_json::to_value(generation).unwrap()["prompt"],
            "A small blue kite in a clear sky"
        );

        let edit = types::images::EditImageBodyJsonParam {
            background: None,
            images: vec![types::images::ImageRefParam {
                file_id: None,
                image_url: Some("https://example.com/source.png".parse().unwrap()),
            }],
            input_fidelity: None,
            mask: None,
            model: None,
            moderation: None,
            n: None,
            output_compression: None,
            output_format: None,
            partial_images: None,
            prompt: "Add a watercolor effect".parse().unwrap(),
            quality: None,
            size: None,
            stream: None,
            user: None,
        };
        let body = serde_json::to_value(edit).unwrap();
        assert_eq!(
            body["images"][0]["image_url"],
            "https://example.com/source.png"
        );
        assert_eq!(body["prompt"], "Add a watercolor effect");
    }

    #[test]
    fn generated_chat_completion_types_deserialize_and_serialize() {
        let request: types::chat_completions::CreateChatCompletionRequest =
            serde_json::from_value(json!({
                "model": "deepseek-flash",
                "messages": [{"role": "user", "content": "hello"}]
            }))
            .unwrap();
        let body = serde_json::to_value(request).unwrap();
        assert_eq!(body["model"], "deepseek-flash");
        assert_eq!(body["messages"][0]["content"], "hello");

        let response: types::chat_completions::CreateChatCompletionResponse =
            serde_json::from_value(json!({
                "id": "chatcmpl_1",
                "object": "chat.completion",
                "created": 1,
                "model": "deepseek-flash",
                "choices": [{
                    "index": 0,
                    "message": {"role": "assistant", "content": "hello"},
                    "finish_reason": "stop",
                    "logprobs": null
                }]
            }))
            .unwrap();
        assert_eq!(
            response.choices[0].message.content.as_deref(),
            Some("hello")
        );
    }

    #[test]
    fn requires_api_key_and_rejects_invalid_base_urls() {
        assert!(matches!(
            ClientBuilder::default().build(),
            Err(Error::MissingApiKey)
        ));
        assert!(matches!(
            ClientBuilder::default()
                .api_key("test")
                .base_url("https://example.com/v1?query=1")
                .build(),
            Err(Error::InvalidBaseUrl(_))
        ));
    }

    #[test]
    fn allows_extra_request_headers_but_reserves_protocol_headers() {
        let headers = RequestOptions::default().header(
            "x-tenant".parse().unwrap(),
            HeaderValue::from_static("demo"),
        );
        assert!(validate_request_headers(&headers.headers).is_ok());

        for name in [AUTHORIZATION, CONTENT_TYPE, ACCEPT] {
            let mut headers = HeaderMap::new();
            headers.insert(name.clone(), HeaderValue::from_static("override"));
            assert!(matches!(
                validate_request_headers(&headers),
                Err(Error::ReservedHeader(header)) if header == name.as_str()
            ));
        }

        let error = ClientBuilder::default()
            .api_key("test")
            .default_header(ACCEPT, HeaderValue::from_static("text/plain"))
            .build();
        assert!(matches!(error, Err(Error::ReservedHeader(header)) if header == "accept"));

        let create = CreateOptions::default().extension("cache_control", json!("ephemeral"));
        assert_eq!(create.extensions["cache_control"], "ephemeral");
    }
}
