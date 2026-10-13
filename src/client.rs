use crate::Error;
use crate::ResponseStream;
use crate::types;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use reqwest::{Method, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::ops::Deref;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Clone)]
pub struct OpenAIClient {
    http: reqwest::Client,
    api_key: String,
    base_url: Url,
}

impl OpenAIClient {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Returns the Responses API resource bound to this client.
    pub fn responses(&self) -> ResponsesApi<'_> {
        ResponsesApi { client: self }
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
        let request = self
            .http
            .request(method, endpoint)
            .headers(headers.clone())
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, accept);
        let request = if let Some(body) = body {
            request.json(&body)
        } else {
            request
        };
        Ok(request.send().await?)
    }
}

/// Responses API methods for an [`OpenAIClient`].
pub struct ResponsesApi<'a> {
    client: &'a OpenAIClient,
}

impl ResponsesApi<'_> {
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
        decode_response(response).await
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
        decode_response(response).await
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
        decode_response(response).await
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
        decode_response(response).await
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
        decode_response(response).await
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
        decode_response(response).await
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
        decode_response(response).await
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
        Ok(ResponseStream::new(response))
    }
}

/// Per-call HTTP headers. Protocol headers managed by the SDK are rejected.
#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    pub headers: HeaderMap,
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
    timeout: Option<Duration>,
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

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
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

        let mut http = reqwest::Client::builder().default_headers(self.default_headers);
        if let Some(timeout) = self.timeout {
            http = http.timeout(timeout);
        }

        Ok(OpenAIClient {
            http: http.build()?,
            api_key,
            base_url,
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
    let params = serde_json::to_value(params)?;
    let Some(params) = params.as_object() else {
        return Ok(());
    };
    let mut query = endpoint.query_pairs_mut();
    for (key, value) in params {
        match value {
            Value::Null => {}
            Value::Array(values) => {
                for value in values {
                    append_query_value(&mut query, key, value)?;
                }
            }
            value => append_query_value(&mut query, key, value)?,
        }
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
        _ => {
            return Err(Error::InvalidQueryParameter(key.to_string()));
        }
    };
    query.append_pair(key, &value);
    Ok(())
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

async fn read_http_error(response: reqwest::Response) -> Error {
    let status = response.status();
    let request_id = request_id(response.headers());
    let body = response
        .bytes()
        .await
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .unwrap_or_else(|error| format!("failed to read error body: {error}"));
    Error::HttpStatus {
        status,
        request_id,
        body,
    }
}

fn request_id(headers: &HeaderMap) -> Option<String> {
    ["x-request-id", "openai-request-id"]
        .into_iter()
        .find_map(|name| headers.get(name).and_then(|value| value.to_str().ok()))
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::{
        ClientBuilder, CreateOptions, RequestOptions, append_path_segments, append_query,
        request_body, response_stream_url, validate_request_headers,
    };
    use crate::Error;
    use crate::types;
    use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
    use serde_json::{Map, json};
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
    fn rejects_non_scalar_query_values() {
        let mut endpoint = Url::parse("https://api.openai.com/v1/responses").unwrap();
        assert!(matches!(
            append_query(&mut endpoint, &json!({"invalid": {"nested": true}})),
            Err(Error::InvalidQueryParameter(name)) if name == "invalid"
        ));
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
