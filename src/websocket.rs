use crate::{Error, OpenAIClient, types};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::tungstenite::http::header::AUTHORIZATION;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;
type SocketSink = futures_util::stream::SplitSink<Socket, Message>;
type SocketSource = Pin<
    Box<
        dyn futures_core::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>>
            + Send,
    >,
>;
type EventRoutes = Arc<Mutex<RouteRegistry>>;

const CHANNEL_CAPACITY: usize = 32;
const MAX_NAMED_STREAMS: usize = 32;
const MAX_QUEUED_RESPONSES_PER_LANE: usize = 16;

#[derive(Default)]
struct RouteRegistry {
    lanes: HashMap<Option<String>, VecDeque<mpsc::Sender<Result<Value, String>>>>,
    named_ids: HashSet<String>,
}

/// An inbound typed event or an event unknown to the pinned schema.
#[derive(Clone, Debug, PartialEq)]
pub enum WebSocketEvent<T> {
    Event(T),
    Unknown(Value),
    Binary(Bytes),
}

/// A bidirectional OpenAI WebSocket connection for one API family's events.
pub struct WebSocketConnection<T> {
    outgoing: mpsc::Sender<Message>,
    source: SocketSource,
    request_id: Option<String>,
    idle_timeout: Option<Duration>,
    closed: Arc<AtomicBool>,
    close_signal: watch::Sender<bool>,
    _event: PhantomData<fn() -> T>,
}

impl<T> WebSocketConnection<T> {
    pub(crate) async fn connect(
        client: &OpenAIClient,
        mut endpoint: reqwest::Url,
    ) -> Result<Self, Error> {
        websocket_scheme(&mut endpoint)?;
        let timeout = match (client.connect_timeout, client.request_timeout) {
            (Some(connect), Some(request)) => Some(connect.min(request)),
            (Some(connect), None) => Some(connect),
            (None, request) => request,
        };
        let max_retries = client.websocket_retry_count();
        let mut attempt = 0;
        let (socket, handshake) = loop {
            let request = websocket_request(client, &endpoint)?;
            let connect = connect_async(request);
            let result = if let Some(timeout) = timeout {
                tokio::time::timeout(timeout, connect)
                    .await
                    .map_err(|_| Error::RequestTimeout)?
            } else {
                connect.await
            };
            match result {
                Ok(connection) => break connection,
                Err(error) if attempt_retry(attempt, max_retries, client, &error).await => {
                    attempt += 1;
                }
                Err(error) => return Err(Error::WebSocketHandshake(error.to_string())),
            }
        };
        let request_id = websocket_request_id(handshake.headers());
        let (sink, source) = socket.split();
        let (outgoing, outgoing_rx) = mpsc::channel(CHANNEL_CAPACITY);
        let closed = Arc::new(AtomicBool::new(false));
        let (close_signal, close_rx) = watch::channel(false);
        tokio::spawn(write_loop(sink, outgoing_rx, close_rx, closed.clone()));
        Ok(Self {
            outgoing,
            source: Box::pin(source),
            request_id,
            idle_timeout: client.stream_idle_timeout,
            closed,
            close_signal,
            _event: PhantomData,
        })
    }

    /// Returns the request ID supplied with the WebSocket handshake response, if any.
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    /// Separates concurrent sending and typed event reception.
    pub fn split(self) -> (WebSocketSender, WebSocketReceiver<T>)
    where
        T: DeserializeOwned + Send + 'static,
    {
        let sender = WebSocketSender {
            outgoing: self.outgoing.clone(),
            closed: self.closed.clone(),
            close_signal: self.close_signal.clone(),
        };
        let receiver = WebSocketReceiver {
            source: self.source,
            outgoing: self.outgoing,
            closed: self.closed,
            close_signal: self.close_signal,
            request_id: self.request_id,
            idle_timeout: self.idle_timeout,
            idle_sleep: self
                .idle_timeout
                .map(|timeout| Box::pin(tokio::time::sleep(timeout))),
            terminated: false,
            _event: PhantomData,
        };
        (sender, receiver)
    }
}

/// The sending half of a split Realtime or Live WebSocket.
#[derive(Clone)]
pub struct WebSocketSender {
    outgoing: mpsc::Sender<Message>,
    closed: Arc<AtomicBool>,
    close_signal: watch::Sender<bool>,
}

impl WebSocketSender {
    /// Sends a JSON event without changing its wire shape.
    pub async fn send_json<T: Serialize>(&self, event: &T) -> Result<(), Error> {
        if self.closed.load(Ordering::Acquire) {
            return Err(Error::WebSocketClosed);
        }
        let text = serde_json::to_string(event)?;
        self.outgoing
            .send(Message::Text(text.into()))
            .await
            .map_err(|_| Error::WebSocketClosed)
    }

    /// Sends a binary media frame.
    pub async fn send_binary(&self, bytes: Bytes) -> Result<(), Error> {
        if self.closed.load(Ordering::Acquire) {
            return Err(Error::WebSocketClosed);
        }
        self.outgoing
            .send(Message::Binary(bytes))
            .await
            .map_err(|_| Error::WebSocketClosed)
    }

    /// Closes the connection and makes future sends fail with `WebSocketClosed`.
    pub async fn close(&self) -> Result<(), Error> {
        if self.closed.swap(true, Ordering::AcqRel) {
            return Err(Error::WebSocketClosed);
        }
        self.close_signal.send_replace(true);
        Ok(())
    }
}

/// The typed receiving half of a split Realtime or Live WebSocket.
pub struct WebSocketReceiver<T> {
    source: SocketSource,
    outgoing: mpsc::Sender<Message>,
    closed: Arc<AtomicBool>,
    close_signal: watch::Sender<bool>,
    request_id: Option<String>,
    idle_timeout: Option<Duration>,
    idle_sleep: Option<Pin<Box<tokio::time::Sleep>>>,
    terminated: bool,
    _event: PhantomData<fn() -> T>,
}

impl<T> WebSocketReceiver<T> {
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl<T: DeserializeOwned> futures_core::Stream for WebSocketReceiver<T> {
    type Item = Result<WebSocketEvent<T>, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.terminated {
            return Poll::Ready(None);
        }
        if let (Some(_timeout), Some(sleep)) = (self.idle_timeout, &mut self.idle_sleep)
            && sleep.as_mut().poll(cx).is_ready()
        {
            self.terminate();
            return Poll::Ready(Some(Err(Error::StreamIdleTimeout)));
        }
        loop {
            match self.source.as_mut().poll_next(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) | Poll::Ready(Some(Ok(Message::Close(_)))) => {
                    self.terminate();
                    return Poll::Ready(None);
                }
                Poll::Ready(Some(Err(error))) => {
                    self.terminate();
                    return Poll::Ready(Some(Err(Error::WebSocket(error.to_string()))));
                }
                Poll::Ready(Some(Ok(Message::Text(text)))) => {
                    let value: Value = match serde_json::from_str(text.as_str()) {
                        Ok(value) => value,
                        Err(error) => {
                            self.terminate();
                            return Poll::Ready(Some(Err(Error::Json(error))));
                        }
                    };
                    if value["type"] == "error" {
                        self.terminate();
                        return Poll::Ready(Some(Err(Error::WebSocketServerEvent {
                            body: value,
                            request_id: self.request_id.clone(),
                        })));
                    }
                    if let (Some(timeout), Some(sleep)) = (self.idle_timeout, &mut self.idle_sleep)
                    {
                        sleep.as_mut().reset(tokio::time::Instant::now() + timeout);
                    }
                    return Poll::Ready(Some(match serde_json::from_value(value.clone()) {
                        Ok(event) => Ok(WebSocketEvent::Event(event)),
                        Err(_) => Ok(WebSocketEvent::Unknown(value)),
                    }));
                }
                Poll::Ready(Some(Ok(Message::Binary(bytes)))) => {
                    if let (Some(timeout), Some(sleep)) = (self.idle_timeout, &mut self.idle_sleep)
                    {
                        sleep.as_mut().reset(tokio::time::Instant::now() + timeout);
                    }
                    return Poll::Ready(Some(Ok(WebSocketEvent::Binary(bytes))));
                }
                Poll::Ready(Some(Ok(Message::Ping(payload)))) => {
                    if self.outgoing.try_send(Message::Pong(payload)).is_err() {
                        self.terminate();
                        return Poll::Ready(Some(Err(Error::WebSocketClosed)));
                    }
                }
                Poll::Ready(Some(Ok(Message::Pong(_) | Message::Frame(_)))) => continue,
            }
        }
    }
}

impl<T> WebSocketReceiver<T> {
    fn terminate(&mut self) {
        self.terminated = true;
        self.closed.store(true, Ordering::Release);
        self.close_signal.send_replace(true);
    }
}

impl<T> Drop for WebSocketReceiver<T> {
    fn drop(&mut self) {
        if !self.terminated {
            self.terminate();
            let _ = self.outgoing.try_send(Message::Close(None));
        }
    }
}

fn websocket_request(
    client: &OpenAIClient,
    endpoint: &reqwest::Url,
) -> Result<tokio_tungstenite::tungstenite::http::Request<()>, Error> {
    let mut request = endpoint
        .as_str()
        .into_client_request()
        .map_err(|error| Error::WebSocket(error.to_string()))?;
    let authorization = HeaderValue::from_str(&format!("Bearer {}", client.api_key))
        .map_err(|error| Error::WebSocket(error.to_string()))?;
    request.headers_mut().insert(AUTHORIZATION, authorization);
    request.headers_mut().extend(client.default_headers.clone());
    Ok(request)
}

async fn attempt_retry(
    attempt: u8,
    max_retries: u8,
    client: &OpenAIClient,
    error: &tokio_tungstenite::tungstenite::Error,
) -> bool {
    if attempt >= max_retries {
        return false;
    }
    let Some(retry_after) = handshake_retry_after(error) else {
        return false;
    };
    tokio::time::sleep(client.websocket_retry_delay(attempt + 1, retry_after)).await;
    true
}

fn handshake_retry_after(
    error: &tokio_tungstenite::tungstenite::Error,
) -> Option<Option<Duration>> {
    match error {
        tokio_tungstenite::tungstenite::Error::Io(_) => Some(None),
        tokio_tungstenite::tungstenite::Error::Http(response) => {
            let status = reqwest::StatusCode::from_u16(response.status().as_u16()).ok()?;
            crate::client::is_retryable_status(status)
                .then(|| crate::client::retry_after_delay(response.headers()))
        }
        _ => None,
    }
}

/// An event delivered on a Responses WebSocket stream lane.
#[derive(Clone, Debug, PartialEq)]
pub struct ResponseWebSocketEvent {
    pub stream_id: Option<String>,
    /// Schema typed when known; otherwise its complete raw JSON is retained.
    pub event: crate::ResponseStreamEvent,
}

/// A bounded event receiver for one Responses WebSocket lane.
pub struct ResponseWebSocketStream {
    stream_id: Option<String>,
    request_id: Option<String>,
    receiver: mpsc::Receiver<Result<Value, String>>,
    idle_timeout: Option<Duration>,
    idle_sleep: Option<Pin<Box<tokio::time::Sleep>>>,
    terminated: bool,
}

impl ResponseWebSocketStream {
    pub fn stream_id(&self) -> Option<&str> {
        self.stream_id.as_deref()
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl futures_core::Stream for ResponseWebSocketStream {
    type Item = Result<ResponseWebSocketEvent, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.terminated {
            return Poll::Ready(None);
        }
        if let Some(sleep) = &mut self.idle_sleep
            && sleep.as_mut().poll(cx).is_ready()
        {
            self.terminated = true;
            return Poll::Ready(Some(Err(Error::StreamIdleTimeout)));
        }
        match self.receiver.poll_recv(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => {
                self.terminated = true;
                Poll::Ready(Some(Err(Error::WebSocketClosed)))
            }
            Poll::Ready(Some(Err(error))) => {
                self.terminated = true;
                Poll::Ready(Some(Err(route_failure(error))))
            }
            Poll::Ready(Some(Ok(value))) => {
                if value.get("type").and_then(Value::as_str) == Some("error") {
                    self.terminated = true;
                    return Poll::Ready(Some(Err(Error::WebSocketServerEvent {
                        body: value,
                        request_id: self.request_id.clone(),
                    })));
                }
                let event = match crate::ResponseStreamEvent::decode(&value.to_string(), None) {
                    Ok(event) => event,
                    Err(error) => {
                        self.terminated = true;
                        return Poll::Ready(Some(Err(error)));
                    }
                };
                if event.event_type() == "response.failed" {
                    self.terminated = true;
                    return Poll::Ready(Some(Err(Error::ResponseFailed {
                        body: value,
                        request_id: self.request_id.clone(),
                    })));
                }
                let terminal = event.is_terminal();
                if terminal {
                    self.terminated = true;
                } else if let (Some(timeout), Some(sleep)) =
                    (self.idle_timeout, &mut self.idle_sleep)
                {
                    sleep.as_mut().reset(tokio::time::Instant::now() + timeout);
                }
                Poll::Ready(Some(Ok(ResponseWebSocketEvent {
                    stream_id: self.stream_id.clone(),
                    event,
                })))
            }
        }
    }
}

impl std::fmt::Debug for ResponseWebSocketStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseWebSocketStream")
            .field("stream_id", &self.stream_id)
            .finish_non_exhaustive()
    }
}

/// A Responses WebSocket connection that routes interleaved events by `stream_id`.
pub struct ResponsesWebSocket {
    outgoing: mpsc::Sender<Message>,
    routes: EventRoutes,
    closed: Arc<AtomicBool>,
    close_signal: watch::Sender<bool>,
    request_id: Option<String>,
    idle_timeout: Option<Duration>,
}

impl ResponsesWebSocket {
    pub(crate) async fn connect(client: &OpenAIClient) -> Result<Self, Error> {
        let endpoint = append_path_segments(&client.base_url, &["responses"])?;
        let connection = WebSocketConnection::<Value>::connect(client, endpoint).await?;
        let idle_timeout = connection.idle_timeout;
        let request_id = connection.request_id.clone();
        let outgoing = connection.outgoing;
        let source = connection.source;
        let routes: EventRoutes = Arc::new(Mutex::new(RouteRegistry::default()));
        let closed = connection.closed;
        let close_signal = connection.close_signal;
        tokio::spawn(read_responses_loop(
            source,
            routes.clone(),
            outgoing.clone(),
            closed.clone(),
            close_signal.clone(),
            idle_timeout,
        ));
        Ok(Self {
            outgoing,
            routes,
            closed,
            close_signal,
            request_id,
            idle_timeout,
        })
    }

    /// Starts one response on the default lane or a named `stream_id` lane.
    pub async fn create_response(
        &self,
        stream_id: Option<String>,
        request: &types::CreateResponse,
    ) -> Result<ResponseWebSocketStream, Error> {
        if self.closed.load(Ordering::Acquire) {
            return Err(Error::WebSocketClosed);
        }
        let value = response_create_payload(request, stream_id.as_deref())?;
        let text = serde_json::to_string(&value)?;
        let (sender, receiver) = mpsc::channel(CHANNEL_CAPACITY);
        {
            let mut routes = self.routes.lock().expect("websocket route mutex poisoned");
            let mut inserted_named_id = false;
            if let Some(id) = &stream_id {
                validate_stream_id(id)?;
                if !routes.named_ids.contains(id) && routes.named_ids.len() >= MAX_NAMED_STREAMS {
                    return Err(Error::WebSocket(format!(
                        "a connection accepts at most {MAX_NAMED_STREAMS} distinct named stream IDs"
                    )));
                }
                inserted_named_id = routes.named_ids.insert(id.clone());
            }
            let lane = routes.lanes.entry(stream_id.clone()).or_default();
            if lane.len() >= MAX_QUEUED_RESPONSES_PER_LANE {
                return Err(Error::WebSocket(format!(
                    "a lane accepts at most {MAX_QUEUED_RESPONSES_PER_LANE} queued responses"
                )));
            }
            lane.push_back(sender);
            if let Err(error) = self.outgoing.try_send(Message::Text(text.into())) {
                let lane = routes
                    .lanes
                    .get_mut(&stream_id)
                    .expect("registered response lane");
                lane.pop_back();
                if lane.is_empty() {
                    routes.lanes.remove(&stream_id);
                }
                if inserted_named_id && let Some(id) = &stream_id {
                    routes.named_ids.remove(id);
                }
                return Err(match error {
                    mpsc::error::TrySendError::Closed(_) => Error::WebSocketClosed,
                    mpsc::error::TrySendError::Full(_) => {
                        Error::WebSocket("Responses WebSocket outgoing queue is full".to_owned())
                    }
                });
            }
        }
        Ok(ResponseWebSocketStream {
            stream_id,
            request_id: self.request_id.clone(),
            receiver,
            idle_timeout: self.idle_timeout,
            idle_sleep: self
                .idle_timeout
                .map(|timeout| Box::pin(tokio::time::sleep(timeout))),
            terminated: false,
        })
    }

    /// Closes the shared Responses socket. Existing lane receivers then report closure.
    pub async fn close(&self) -> Result<(), Error> {
        if self.closed.swap(true, Ordering::AcqRel) {
            return Err(Error::WebSocketClosed);
        }
        self.close_signal.send_replace(true);
        Ok(())
    }
}

impl Drop for ResponseWebSocketStream {
    fn drop(&mut self) {
        if !self.terminated {
            // Removing this sender lets the router stop reading a lane whose
            // consumer was dropped, so one abandoned stream cannot block others.
            self.receiver.close();
        }
    }
}

fn append_path_segments(base: &reqwest::Url, segments: &[&str]) -> Result<reqwest::Url, Error> {
    let mut url = base.clone();
    let original = url.to_string();
    {
        let mut path = url
            .path_segments_mut()
            .map_err(|_| Error::InvalidBaseUrl(original))?;
        path.pop_if_empty();
        path.extend(segments.iter().copied());
    }
    Ok(url)
}

fn websocket_scheme(url: &mut reqwest::Url) -> Result<(), Error> {
    let scheme = match url.scheme() {
        "https" => "wss",
        "http" => "ws",
        "ws" | "wss" => return Ok(()),
        other => {
            return Err(Error::InvalidBaseUrl(format!(
                "unsupported websocket URL scheme: {other}"
            )));
        }
    };
    url.set_scheme(scheme)
        .map_err(|_| Error::InvalidBaseUrl(url.to_string()))
}

fn websocket_request_id(
    headers: &tokio_tungstenite::tungstenite::http::HeaderMap,
) -> Option<String> {
    ["x-request-id", "openai-request-id"]
        .into_iter()
        .find_map(|name| headers.get(name).and_then(|value| value.to_str().ok()))
        .map(str::to_owned)
}

async fn write_loop(
    mut sink: SocketSink,
    mut receiver: mpsc::Receiver<Message>,
    mut close_signal: watch::Receiver<bool>,
    closed: Arc<AtomicBool>,
) {
    loop {
        tokio::select! {
            biased;
            changed = close_signal.changed() => {
                if changed.is_ok() && *close_signal.borrow() {
                    let _ = sink.send(Message::Close(None)).await;
                    break;
                }
            }
            message = receiver.recv() => {
                let Some(message) = message else { break };
                let close = matches!(message, Message::Close(_));
                if sink.send(message).await.is_err() || close {
                    break;
                }
            }
        }
    }
    let _ = sink.close().await;
    closed.store(true, Ordering::Release);
}

fn route_failure(reason: String) -> Error {
    match reason.as_str() {
        "Responses WebSocket closed" => Error::WebSocketClosed,
        "Responses WebSocket idle timeout" => Error::StreamIdleTimeout,
        _ => Error::WebSocket(reason),
    }
}

async fn read_responses_loop(
    mut source: SocketSource,
    routes: EventRoutes,
    outgoing: mpsc::Sender<Message>,
    closed: Arc<AtomicBool>,
    close_signal: watch::Sender<bool>,
    idle_timeout: Option<Duration>,
) {
    loop {
        let message = if let Some(timeout) = idle_timeout {
            match tokio::time::timeout(timeout, source.next()).await {
                Ok(message) => message,
                Err(_) => {
                    fail_responses(
                        &routes,
                        &closed,
                        &close_signal,
                        "Responses WebSocket idle timeout",
                    )
                    .await;
                    return;
                }
            }
        } else {
            source.next().await
        };
        let Some(message) = message else {
            fail_responses(
                &routes,
                &closed,
                &close_signal,
                "Responses WebSocket closed",
            )
            .await;
            return;
        };
        let message = match message {
            Ok(Message::Text(text)) => text.to_string(),
            Ok(Message::Ping(payload)) => {
                if outgoing.send(Message::Pong(payload)).await.is_err() {
                    fail_responses(
                        &routes,
                        &closed,
                        &close_signal,
                        "Responses WebSocket writer closed",
                    )
                    .await;
                    return;
                }
                continue;
            }
            Ok(Message::Pong(_) | Message::Frame(_)) => continue,
            Ok(Message::Close(_)) => {
                let _ = outgoing.send(Message::Close(None)).await;
                fail_responses(
                    &routes,
                    &closed,
                    &close_signal,
                    "Responses WebSocket closed",
                )
                .await;
                return;
            }
            Ok(Message::Binary(_)) => {
                fail_responses(
                    &routes,
                    &closed,
                    &close_signal,
                    "unexpected binary frame on Responses WebSocket",
                )
                .await;
                return;
            }
            Err(error) => {
                fail_responses(&routes, &closed, &close_signal, &error.to_string()).await;
                return;
            }
        };
        let value: Value = match serde_json::from_str(&message) {
            Ok(value) => value,
            Err(error) => {
                fail_responses(
                    &routes,
                    &closed,
                    &close_signal,
                    &format!("invalid Responses event JSON: {error}"),
                )
                .await;
                return;
            }
        };
        let stream_id = value
            .get("stream_id")
            .and_then(Value::as_str)
            .map(str::to_owned);
        let event_type = value
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_owned();
        let sender = routes
            .lock()
            .expect("websocket route mutex poisoned")
            .lanes
            .get(&stream_id)
            .and_then(VecDeque::front)
            .cloned();
        let Some(sender) = sender else {
            if event_type == "error" && stream_id.is_none() {
                fail_responses(
                    &routes,
                    &closed,
                    &close_signal,
                    &format!("connection-level Responses error event: {value}"),
                )
                .await;
                return;
            }
            // A dropped or already-finished consumer may leave late events on
            // the wire. Discard that lane without disturbing other streams.
            continue;
        };
        let _ = sender.send(Ok(value)).await;
        if matches!(
            event_type.as_str(),
            "response.completed" | "response.incomplete" | "response.failed" | "error"
        ) {
            let mut registry = routes.lock().expect("websocket route mutex poisoned");
            if let Some(lane) = registry.lanes.get_mut(&stream_id) {
                lane.pop_front();
                if lane.is_empty() {
                    registry.lanes.remove(&stream_id);
                }
            }
        }
    }
}

async fn fail_responses(
    routes: &EventRoutes,
    closed: &AtomicBool,
    close_signal: &watch::Sender<bool>,
    reason: &str,
) {
    closed.store(true, Ordering::Release);
    close_signal.send_replace(true);
    let senders = routes
        .lock()
        .expect("websocket route mutex poisoned")
        .lanes
        .drain()
        .flat_map(|(_, lane)| lane)
        .collect::<Vec<_>>();
    for sender in senders {
        let _ = sender.send(Err(reason.to_owned())).await;
    }
}

fn validate_stream_id(value: &str) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > 256
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
    {
        return Err(Error::WebSocket(
            "stream_id must contain 1 to 256 letters, numbers, underscores, hyphens, or periods"
                .to_owned(),
        ));
    }
    Ok(())
}

fn response_create_payload(
    request: &types::CreateResponse,
    stream_id: Option<&str>,
) -> Result<Value, Error> {
    let mut value = serde_json::to_value(request)?;
    let object = value.as_object_mut().ok_or_else(|| {
        Error::WebSocket("response request must serialize as a JSON object".to_owned())
    })?;
    object.insert(
        "type".to_owned(),
        Value::String("response.create".to_owned()),
    );
    if let Some(id) = stream_id {
        object.insert("stream_id".to_owned(), Value::String(id.to_owned()));
    }
    // WebSocket mode mirrors the Responses request body but excludes these
    // transport-specific HTTP parameters.
    object.remove("stream");
    object.remove("background");
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{
        CHANNEL_CAPACITY, MAX_NAMED_STREAMS, ResponseWebSocketStream, RouteRegistry,
        WebSocketEvent, WebSocketReceiver, WebSocketSender, handshake_retry_after,
        response_create_payload, validate_stream_id, websocket_scheme,
    };
    use crate::Error;
    use crate::types;
    use futures_util::StreamExt;
    use futures_util::stream;
    use serde_json::{Value, json};
    use std::marker::PhantomData;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::time::Duration;
    use tokio::sync::{mpsc, watch};
    use tokio_tungstenite::tungstenite::protocol::Message;

    #[test]
    fn websocket_handshake_retry_classifies_only_connection_and_retryable_http_errors() {
        use tokio_tungstenite::tungstenite::{Error as TungsteniteError, http::Response};

        let too_many_requests = Response::builder()
            .status(429)
            .header("retry-after", "5")
            .body(None::<Vec<u8>>)
            .unwrap();
        assert_eq!(
            handshake_retry_after(&TungsteniteError::Http(Box::new(too_many_requests))),
            Some(Some(Duration::from_secs(5)))
        );

        let unavailable = Response::builder()
            .status(503)
            .body(None::<Vec<u8>>)
            .unwrap();
        assert_eq!(
            handshake_retry_after(&TungsteniteError::Http(Box::new(unavailable))),
            Some(None)
        );

        let unauthorized = Response::builder()
            .status(401)
            .body(None::<Vec<u8>>)
            .unwrap();
        assert_eq!(
            handshake_retry_after(&TungsteniteError::Http(Box::new(unauthorized))),
            None
        );
        assert_eq!(
            handshake_retry_after(&TungsteniteError::Protocol(
                tokio_tungstenite::tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
            )),
            None
        );
    }

    fn response_stream(
        capacity: usize,
        idle_timeout: Option<Duration>,
    ) -> (ResponseWebSocketStream, mpsc::Sender<Result<Value, String>>) {
        let (sender, receiver) = mpsc::channel(capacity);
        let idle_sleep = idle_timeout.map(|timeout| Box::pin(tokio::time::sleep(timeout)));
        (
            ResponseWebSocketStream {
                stream_id: Some("unit-test".to_owned()),
                request_id: Some("req_fixture".to_owned()),
                receiver,
                idle_timeout,
                idle_sleep,
                terminated: false,
            },
            sender,
        )
    }

    fn websocket_halves<T>(
        messages: Vec<Message>,
    ) -> (
        WebSocketSender,
        WebSocketReceiver<T>,
        mpsc::Receiver<Message>,
        watch::Receiver<bool>,
    ) {
        let (outgoing, written) = mpsc::channel(CHANNEL_CAPACITY);
        let source = Box::pin(stream::iter(messages.into_iter().map(Ok)));
        let closed = Arc::new(AtomicBool::new(false));
        let (close_signal, close_rx) = watch::channel(false);
        let sender = WebSocketSender {
            outgoing: outgoing.clone(),
            closed: closed.clone(),
            close_signal: close_signal.clone(),
        };
        let receiver = WebSocketReceiver {
            source,
            outgoing,
            closed,
            close_signal,
            request_id: Some("req_socket".to_owned()),
            idle_timeout: None,
            idle_sleep: None,
            terminated: false,
            _event: PhantomData,
        };
        (sender, receiver, written, close_rx)
    }

    #[tokio::test]
    async fn typed_websocket_receiver_preserves_unknown_and_binary_frames() {
        let known = json!({
            "type":"response.output_text.delta",
            "event_id":"evt_1",
            "response_id":"resp_1",
            "item_id":"item_1",
            "output_index":0,
            "content_index":0,
            "delta":"hello"
        });
        let unknown = json!({"type":"server.new_event","extra":42});
        let (sender, mut receiver, mut written, _close_rx) =
            websocket_halves::<types::realtime::RealtimeServerEvent>(vec![
                Message::Text(known.to_string().into()),
                Message::Text(unknown.to_string().into()),
                Message::Binary(bytes::Bytes::from_static(b"audio")),
            ]);
        let outbound = json!({"type":"client.event"});
        let (send_result, receive_result) =
            tokio::join!(sender.send_json(&outbound), receiver.next());
        send_result.unwrap();
        assert!(matches!(written.recv().await, Some(Message::Text(_))));

        let event = receive_result.unwrap().unwrap();
        assert!(matches!(
            event,
            WebSocketEvent::Event(types::realtime::RealtimeServerEvent::ResponseTextDelta(_))
        ));
        assert_eq!(
            receiver.next().await.unwrap().unwrap(),
            WebSocketEvent::Unknown(unknown)
        );
        assert_eq!(
            receiver.next().await.unwrap().unwrap(),
            WebSocketEvent::Binary(bytes::Bytes::from_static(b"audio"))
        );
        assert!(receiver.next().await.is_none());
    }

    #[tokio::test]
    async fn websocket_protocol_error_is_reported_once() {
        let error = json!({"type":"error","error":{"message":"bad request"}});
        let (_sender, mut receiver, _written, _close_rx) =
            websocket_halves::<types::realtime::RealtimeServerEvent>(vec![Message::Text(
                error.to_string().into(),
            )]);
        assert!(matches!(
            receiver.next().await.unwrap(),
            Err(Error::WebSocketServerEvent { body, .. }) if body == error
        ));
        assert!(receiver.next().await.is_none());
    }

    #[tokio::test]
    async fn websocket_sender_close_and_receiver_drop_stop_future_sends() {
        let (sender, receiver, _written, close_rx) =
            websocket_halves::<types::live::LiveServerEvent>(vec![]);
        sender.close().await.unwrap();
        assert!(*close_rx.borrow());
        assert!(matches!(
            sender.send_json(&json!({"type":"later"})).await,
            Err(Error::WebSocketClosed)
        ));
        drop(receiver);

        let (sender, receiver, _written, close_rx) =
            websocket_halves::<types::live::LiveServerEvent>(vec![]);
        drop(receiver);
        assert!(matches!(
            sender.send_binary(bytes::Bytes::from_static(b"x")).await,
            Err(Error::WebSocketClosed)
        ));
        assert!(*close_rx.borrow());
    }

    fn response_value(status: &str) -> Value {
        let mut response = json!({
            "id": "resp_fixture",
            "object": "response",
            "created_at": 1,
            "status": status,
            "error": null,
            "incomplete_details": null,
            "instructions": null,
            "metadata": {},
            "model": "gpt-5.4",
            "output": [],
            "parallel_tool_calls": false,
            "tool_choice": "auto",
            "tools": [],
            "temperature": null,
            "top_p": null,
            "access_programs": null
        });
        if status == "incomplete" {
            response["incomplete_details"] = json!({"reason":"max_output_tokens"});
        }
        if status == "failed" {
            response["error"] = json!({"code":"server_error","message":"failed"});
        }
        response
    }

    #[test]
    fn maps_http_endpoint_schemes_to_websocket_schemes() {
        let mut https = reqwest::Url::parse("https://api.openai.com/v1/responses").unwrap();
        websocket_scheme(&mut https).unwrap();
        assert_eq!(https.scheme(), "wss");
        let mut http = reqwest::Url::parse("http://localhost/v1/live/sessions").unwrap();
        websocket_scheme(&mut http).unwrap();
        assert_eq!(http.scheme(), "ws");
        let mut ftp = reqwest::Url::parse("ftp://example.com").unwrap();
        assert!(matches!(
            websocket_scheme(&mut ftp),
            Err(Error::InvalidBaseUrl(_))
        ));
    }

    #[test]
    fn lane_id_constraints_match_the_responses_websocket_protocol() {
        assert_eq!(MAX_NAMED_STREAMS, 32);
        assert!(validate_stream_id("run-1.main_lane").is_ok());
        assert!(validate_stream_id("").is_err());
        assert!(validate_stream_id("not valid").is_err());
        assert!(validate_stream_id(&"x".repeat(257)).is_err());
    }

    #[test]
    fn response_create_uses_websocket_shape() {
        let request: crate::types::CreateResponse = serde_json::from_value(json!({
            "model": "gpt-4.1-mini",
            "input": "short",
            "stream": true,
            "background": true
        }))
        .unwrap();
        let payload = response_create_payload(&request, Some("lane-1")).unwrap();
        assert_eq!(payload["type"], "response.create");
        assert_eq!(payload["stream_id"], "lane-1");
        assert!(payload.get("stream").is_none());
        assert!(payload.get("background").is_none());
    }

    #[tokio::test]
    async fn response_lanes_route_queued_turns_in_fifo_order() {
        let (first_tx, mut first_rx) = mpsc::channel(2);
        let (second_tx, mut second_rx) = mpsc::channel(2);
        let mut routes = RouteRegistry::default();
        routes
            .lanes
            .entry(Some("lane-1".to_owned()))
            .or_default()
            .extend([first_tx, second_tx]);
        let lane = routes.lanes.get(&Some("lane-1".to_owned())).unwrap();
        lane.front()
            .unwrap()
            .send(Ok(json!({"type":"response.completed","id":"first"})))
            .await
            .unwrap();
        assert_eq!(first_rx.recv().await.unwrap().unwrap()["id"], "first");
        assert!(second_rx.try_recv().is_err());
        routes
            .lanes
            .get_mut(&Some("lane-1".to_owned()))
            .unwrap()
            .pop_front();
        routes
            .lanes
            .get(&Some("lane-1".to_owned()))
            .unwrap()
            .front()
            .unwrap()
            .send(Ok(json!({"type":"response.completed","id":"second"})))
            .await
            .unwrap();
        assert_eq!(second_rx.recv().await.unwrap().unwrap()["id"], "second");
    }

    #[tokio::test]
    async fn response_stream_preserves_unknown_events_and_stops_at_semantic_terminal() {
        let (mut stream, sender) = response_stream(2, None);
        sender
            .send(Ok(json!({"type":"response.future_event","extra":{"n":3}})))
            .await
            .unwrap();
        sender
            .send(Ok(json!({
                "type":"response.incomplete",
                "sequence_number":9,
                "response":response_value("incomplete")
            })))
            .await
            .unwrap();
        let event = stream.next().await.unwrap().unwrap();
        assert_eq!(event.event.event_type(), "response.future_event");
        assert_eq!(event.event.data()["extra"]["n"], 3);
        assert!(event.event.typed().is_none());
        let terminal = stream.next().await.unwrap().unwrap();
        assert_eq!(terminal.event.event_type(), "response.incomplete");
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn response_stream_maps_failed_and_error_events_to_errors() {
        let (mut failed, failed_tx) = response_stream(1, None);
        let failure = json!({
            "type":"response.failed",
            "sequence_number":1,
            "response":response_value("failed")
        });
        failed_tx.send(Ok(failure.clone())).await.unwrap();
        assert!(matches!(
            failed.next().await.unwrap(),
            Err(Error::ResponseFailed { body, request_id })
                if body == failure && request_id.as_deref() == Some("req_fixture")
        ));

        let (mut server_error, error_tx) = response_stream(1, None);
        let error = json!({
            "type":"error",
            "status":400,
            "error":{
                "type":"invalid_request_error",
                "code":"invalid_stream_id",
                "message":"The stream identifier is invalid.",
                "param":"stream_id"
            }
        });
        error_tx.send(Ok(error.clone())).await.unwrap();
        assert!(matches!(
            server_error.next().await.unwrap(),
            Err(Error::WebSocketServerEvent { body, request_id })
                if body == error && request_id.as_deref() == Some("req_fixture")
        ));
    }

    #[tokio::test]
    async fn response_stream_reports_close_before_terminal() {
        let (mut stream, sender) = response_stream(1, None);
        drop(sender);
        assert!(matches!(
            stream.next().await.unwrap(),
            Err(Error::WebSocketClosed)
        ));
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn response_stream_reports_idle_timeout() {
        let (mut stream, _sender) = response_stream(1, Some(Duration::from_millis(5)));
        assert!(matches!(
            stream.next().await.unwrap(),
            Err(Error::StreamIdleTimeout)
        ));
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn response_stream_drop_closes_its_bounded_receiver() {
        let (stream, sender) = response_stream(CHANNEL_CAPACITY, None);
        drop(stream);
        assert!(sender.is_closed());
        assert!(matches!(
            sender.try_send(Ok(json!({"type":"response.output_text.delta"}))),
            Err(mpsc::error::TrySendError::Closed(_))
        ));
    }

    #[tokio::test]
    async fn response_stream_channel_applies_bounded_backpressure() {
        let (_stream, sender) = response_stream(CHANNEL_CAPACITY, None);
        for sequence_number in 0..CHANNEL_CAPACITY {
            sender
                .try_send(Ok(
                    json!({"type":"response.output_text.delta","sequence_number":sequence_number}),
                ))
                .unwrap();
        }
        assert!(matches!(
            sender.try_send(Ok(json!({"type":"response.output_text.delta"}))),
            Err(mpsc::error::TrySendError::Full(_))
        ));
    }
}
