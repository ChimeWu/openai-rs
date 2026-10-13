use crate::Error;
use crate::ResponseStream;
use crate::types;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use reqwest::{Method, Url};
use serde_json::{Map, Value};
use std::ops::Deref;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";
const RESPONSES_PATH: &str = "responses";

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

    /// Creates a non-streaming response and preserves its original JSON body.
    pub async fn create(&self, request: &types::CreateResponse) -> Result<ResponseWithRaw, Error> {
        self.create_with_extensions_and_headers(request, Map::new(), HeaderMap::new())
            .await
    }

    /// Creates a response with additional top-level request fields.
    pub async fn create_with_extensions(
        &self,
        request: &types::CreateResponse,
        extensions: Map<String, Value>,
    ) -> Result<ResponseWithRaw, Error> {
        self.create_with_extensions_and_headers(request, extensions, HeaderMap::new())
            .await
    }

    /// Creates a response with additional HTTP headers for this request.
    pub async fn create_with_headers(
        &self,
        request: &types::CreateResponse,
        headers: HeaderMap,
    ) -> Result<ResponseWithRaw, Error> {
        self.create_with_extensions_and_headers(request, Map::new(), headers)
            .await
    }

    /// Creates a response with extra request fields and HTTP headers.
    pub async fn create_with_extensions_and_headers(
        &self,
        request: &types::CreateResponse,
        extensions: Map<String, Value>,
        headers: HeaderMap,
    ) -> Result<ResponseWithRaw, Error> {
        let mut request = request.clone();
        request.stream = Some(false);
        let body = request_body(&request, extensions)?;
        let response = self
            .send_json(Method::POST, body, ACCEPT_JSON, headers)
            .await?;
        decode_response(response).await
    }

    /// Creates a streaming response. The returned stream ends after a semantic
    /// Responses terminal event, or reports an error if the connection closes
    /// first.
    pub async fn create_stream(
        &self,
        request: &types::CreateResponse,
    ) -> Result<ResponseStream, Error> {
        self.create_stream_with_extensions_and_headers(request, Map::new(), HeaderMap::new())
            .await
    }

    /// Creates a streaming response with additional top-level request fields.
    pub async fn create_stream_with_extensions(
        &self,
        request: &types::CreateResponse,
        extensions: Map<String, Value>,
    ) -> Result<ResponseStream, Error> {
        self.create_stream_with_extensions_and_headers(request, extensions, HeaderMap::new())
            .await
    }

    /// Creates a streaming response with additional HTTP headers for this request.
    pub async fn create_stream_with_headers(
        &self,
        request: &types::CreateResponse,
        headers: HeaderMap,
    ) -> Result<ResponseStream, Error> {
        self.create_stream_with_extensions_and_headers(request, Map::new(), headers)
            .await
    }

    /// Creates a streaming response with extra request fields and HTTP headers.
    pub async fn create_stream_with_extensions_and_headers(
        &self,
        request: &types::CreateResponse,
        extensions: Map<String, Value>,
        headers: HeaderMap,
    ) -> Result<ResponseStream, Error> {
        let mut request = request.clone();
        request.stream = Some(true);
        let body = request_body(&request, extensions)?;
        let response = self
            .send_json(Method::POST, body, ACCEPT_EVENTS, headers)
            .await?;
        if !response.status().is_success() {
            return Err(read_http_error(response).await);
        }
        Ok(ResponseStream::new(response))
    }

    async fn send_json(
        &self,
        method: Method,
        body: Value,
        accept: &'static str,
        headers: HeaderMap,
    ) -> Result<reqwest::Response, Error> {
        validate_request_headers(&headers)?;
        let endpoint = responses_url(&self.base_url)?;
        Ok(self
            .http
            .request(method, endpoint)
            .headers(headers)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, accept)
            .json(&body)
            .send()
            .await?)
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

/// A typed Responses object paired with the unmodified JSON body and request ID.
#[derive(Clone, Debug, PartialEq)]
pub struct ResponseWithRaw {
    pub response: types::Response,
    raw: Value,
    request_id: Option<String>,
}

impl ResponseWithRaw {
    pub fn raw(&self) -> &Value {
        &self.raw
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl Deref for ResponseWithRaw {
    type Target = types::Response;

    fn deref(&self) -> &Self::Target {
        &self.response
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
    let mut endpoint = base_url.clone();
    let path = format!(
        "{}/{}",
        endpoint.path().trim_end_matches('/'),
        RESPONSES_PATH
    );
    endpoint.set_path(&path);
    Ok(endpoint)
}

async fn decode_response(response: reqwest::Response) -> Result<ResponseWithRaw, Error> {
    if !response.status().is_success() {
        return Err(read_http_error(response).await);
    }
    let request_id = request_id(response.headers());
    let bytes = response.bytes().await?;
    let raw: Value = serde_json::from_slice(&bytes)?;
    let response = serde_json::from_value(raw.clone())?;
    Ok(ResponseWithRaw {
        response,
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
    use super::{ClientBuilder, request_body, responses_url, validate_request_headers};
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
    fn appends_responses_path_to_openai_and_compatible_api_roots() {
        let openai = Url::parse("https://api.openai.com/v1").unwrap();
        let deepseek = Url::parse("https://api.deepseek.com").unwrap();
        assert_eq!(
            responses_url(&openai).unwrap().as_str(),
            "https://api.openai.com/v1/responses"
        );
        assert_eq!(
            responses_url(&deepseek).unwrap().as_str(),
            "https://api.deepseek.com/responses"
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
        let mut headers = HeaderMap::new();
        headers.insert("x-tenant", HeaderValue::from_static("demo"));
        assert!(validate_request_headers(&headers).is_ok());

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
    }
}
