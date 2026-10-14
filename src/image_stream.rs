use crate::Error;
use crate::types::images;
use async_stream::stream;
use bytes::Bytes;
use eventsource_stream::Eventsource;
use futures_core::Stream;
use futures_util::StreamExt;
use serde_json::Value;
use std::fmt::Display;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub(crate) enum ImageEventKind {
    Generation,
    Edit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImageEventPayload {
    Generation(images::ImageGenStreamEvent),
    Edit(images::ImageEditStreamEvent),
    Unknown(Value),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImageStreamEvent {
    pub event_type: String,
    pub payload: ImageEventPayload,
}

pub struct ImageEventStream {
    inner: Pin<Box<dyn Stream<Item = Result<ImageStreamEvent, Error>> + Send>>,
    request_id: Option<String>,
}

impl ImageEventStream {
    pub(crate) fn new(
        response: reqwest::Response,
        request_id: Option<String>,
        kind: ImageEventKind,
        idle_timeout: Option<Duration>,
    ) -> Self {
        Self {
            inner: Box::pin(parse_events(response.bytes_stream(), kind, idle_timeout)),
            request_id,
        }
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

impl Stream for ImageEventStream {
    type Item = Result<ImageStreamEvent, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().inner.as_mut().poll_next(cx)
    }
}

impl std::fmt::Debug for ImageEventStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageEventStream")
            .field("request_id", &self.request_id)
            .finish_non_exhaustive()
    }
}

fn parse_events<S, E>(
    source: S,
    kind: ImageEventKind,
    idle_timeout: Option<Duration>,
) -> impl Stream<Item = Result<ImageStreamEvent, Error>> + Send
where
    S: Stream<Item = Result<Bytes, E>> + Send + Unpin + 'static,
    E: Display + Send + 'static,
{
    let mut source = source.eventsource();
    stream! {
        let mut terminal = false;
        loop {
            let next = if let Some(timeout) = idle_timeout {
                match tokio::time::timeout(timeout, source.next()).await {
                    Ok(next) => next,
                    Err(_) => {
                        yield Err(Error::StreamIdleTimeout);
                        return;
                    }
                }
            } else {
                source.next().await
            };
            let Some(frame) = next else { break };
            let frame = match frame {
                Ok(frame) => frame,
                Err(error) => {
                    yield Err(Error::Sse(error.to_string()));
                    return;
                }
            };
            if frame.data.is_empty() {
                continue;
            }
            let value: Value = match serde_json::from_str(&frame.data) {
                Ok(value) => value,
                Err(error) => {
                    yield Err(Error::Sse(error.to_string()));
                    return;
                }
            };
            let event_type = value.get("type")
                .and_then(Value::as_str)
                .or_else(|| (!frame.event.is_empty()).then_some(frame.event.as_str()))
                .unwrap_or("unknown")
                .to_owned();
            let payload = match (kind, event_type.as_str()) {
                (ImageEventKind::Generation, "image_generation.partial_image" | "image_generation.completed") => {
                    match serde_json::from_value(value.clone()) {
                        Ok(event) => ImageEventPayload::Generation(event),
                        Err(error) => {
                            yield Err(Error::Json(error));
                            return;
                        }
                    }
                }
                (ImageEventKind::Edit, "image_edit.partial_image" | "image_edit.completed") => {
                    match serde_json::from_value(value.clone()) {
                        Ok(event) => ImageEventPayload::Edit(event),
                        Err(error) => {
                            yield Err(Error::Json(error));
                            return;
                        }
                    }
                }
                _ => ImageEventPayload::Unknown(value),
            };
            terminal = matches!(event_type.as_str(), "image_generation.completed" | "image_edit.completed");
            yield Ok(ImageStreamEvent { event_type, payload });
            if terminal {
                break;
            }
        }
        if !terminal {
            yield Err(Error::UnexpectedStreamEnd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ImageEventKind, ImageEventPayload, parse_events};
    use crate::Error;
    use bytes::Bytes;
    use futures_util::{StreamExt, stream};

    fn chunks(
        text: &'static str,
        sizes: Vec<usize>,
    ) -> impl futures_core::Stream<Item = Result<Bytes, std::convert::Infallible>> + Unpin {
        let mut result = Vec::new();
        let mut offset = 0;
        for size in sizes {
            let end = (offset + size).min(text.len());
            if end > offset {
                result.push(Ok(Bytes::copy_from_slice(&text.as_bytes()[offset..end])));
                offset = end;
            }
        }
        if offset < text.len() {
            result.push(Ok(Bytes::copy_from_slice(&text.as_bytes()[offset..])));
        }
        stream::iter(result)
    }

    #[tokio::test]
    async fn parses_partial_unknown_and_completed_events_across_chunks() {
        let input = "event: image_generation.partial_image\ndata: {\"type\":\"image_generation.partial_image\",\"b64_json\":\"abc\",\"created_at\":1,\"size\":\"1024x1024\",\"quality\":\"high\",\"background\":\"opaque\",\"output_format\":\"png\",\"partial_image_index\":0}\n\nevent: future.image_event\ndata: {\"type\":\"future.image_event\",\"field\":7}\n\nevent: image_generation.completed\ndata: {\"type\":\"image_generation.completed\",\"b64_json\":\"done\",\"created_at\":1,\"size\":\"1024x1024\",\"quality\":\"high\",\"background\":\"opaque\",\"output_format\":\"png\",\"usage\":{\"input_tokens\":0,\"input_tokens_details\":{\"image_tokens\":0,\"text_tokens\":0},\"output_tokens\":0,\"total_tokens\":0}}\n\n";
        let mut events = Box::pin(parse_events(
            chunks(input, vec![1, 9, 3, 17, 24, 4]),
            ImageEventKind::Generation,
            None,
        ));
        assert!(matches!(
            events.next().await.unwrap().unwrap().payload,
            ImageEventPayload::Generation(_)
        ));
        assert!(matches!(
            events.next().await.unwrap().unwrap().payload,
            ImageEventPayload::Unknown(_)
        ));
        assert_eq!(
            events.next().await.unwrap().unwrap().event_type,
            "image_generation.completed"
        );
        assert!(events.next().await.is_none());
    }

    #[tokio::test]
    async fn eof_before_completed_is_an_error() {
        let input = "data: {\"type\":\"image_generation.partial_image\",\"b64_json\":\"x\",\"created_at\":1,\"size\":\"1024x1024\",\"quality\":\"high\",\"background\":\"opaque\",\"output_format\":\"png\",\"partial_image_index\":0}\n\n";
        let mut events = Box::pin(parse_events(
            chunks(input, vec![input.len()]),
            ImageEventKind::Generation,
            None,
        ));
        assert!(events.next().await.unwrap().is_ok());
        assert!(matches!(
            events.next().await.unwrap(),
            Err(Error::UnexpectedStreamEnd)
        ));
    }

    #[tokio::test]
    async fn parses_typed_edit_events() {
        let input = "data: {\"type\":\"image_edit.partial_image\",\"b64_json\":\"partial\",\"created_at\":1,\"size\":\"1024x1024\",\"quality\":\"high\",\"background\":\"opaque\",\"output_format\":\"png\",\"partial_image_index\":0}\n\n";
        let mut events = Box::pin(parse_events(
            chunks(input, vec![input.len()]),
            ImageEventKind::Edit,
            None,
        ));
        assert!(matches!(
            events.next().await.unwrap().unwrap().payload,
            ImageEventPayload::Edit(_)
        ));
    }
}
