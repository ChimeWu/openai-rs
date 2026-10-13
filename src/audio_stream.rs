use crate::Error;
use crate::types::audio;
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
pub(crate) enum AudioEventKind {
    Speech,
    #[cfg(feature = "multipart")]
    Transcription,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioEventPayload {
    Speech(audio::CreateSpeechResponseStreamEvent),
    Transcription(audio::CreateTranscriptionResponseStreamEvent),
    Unknown(Value),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioStreamEvent {
    pub event_type: String,
    pub payload: AudioEventPayload,
}

pub struct AudioEventStream {
    inner: Pin<Box<dyn Stream<Item = Result<AudioStreamEvent, Error>> + Send>>,
    request_id: Option<String>,
}

impl AudioEventStream {
    pub(crate) fn new(
        response: reqwest::Response,
        request_id: Option<String>,
        kind: AudioEventKind,
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

    pub async fn next_event(&mut self) -> Option<Result<AudioStreamEvent, Error>> {
        self.inner.next().await
    }
}

impl Stream for AudioEventStream {
    type Item = Result<AudioStreamEvent, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().inner.as_mut().poll_next(cx)
    }
}

impl std::fmt::Debug for AudioEventStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioEventStream")
            .field("request_id", &self.request_id)
            .finish_non_exhaustive()
    }
}

fn parse_events<S, E>(
    source: S,
    kind: AudioEventKind,
    idle_timeout: Option<Duration>,
) -> impl Stream<Item = Result<AudioStreamEvent, Error>> + Send
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
                (AudioEventKind::Speech, "speech.audio.delta" | "speech.audio.done") => {
                    match serde_json::from_value(value.clone()) {
                        Ok(event) => AudioEventPayload::Speech(event),
                        Err(error) => {
                            yield Err(Error::Json(error));
                            return;
                        }
                    }
                }
                #[cfg(feature = "multipart")]
                (AudioEventKind::Transcription, "transcript.text.delta" | "transcript.text.done" | "transcript.text.segment") => {
                    match serde_json::from_value(value.clone()) {
                        Ok(event) => AudioEventPayload::Transcription(event),
                        Err(error) => {
                            yield Err(Error::Json(error));
                            return;
                        }
                    }
                }
                _ => AudioEventPayload::Unknown(value),
            };
            terminal = matches!(event_type.as_str(), "speech.audio.done" | "transcript.text.done");
            yield Ok(AudioStreamEvent { event_type, payload });
            if terminal {
                break;
            }
        }
        if !terminal {
            yield Err(Error::UnexpectedStreamEnd);
        }
    }
}

pub struct AudioContentStream {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send>>,
    request_id: Option<String>,
    content_type: Option<String>,
    idle_timeout: Option<Duration>,
    terminated: bool,
}

impl AudioContentStream {
    pub(crate) fn new(
        response: reqwest::Response,
        request_id: Option<String>,
        content_type: Option<String>,
        idle_timeout: Option<Duration>,
    ) -> Self {
        let source = response
            .bytes_stream()
            .map(|item| item.map_err(Error::Http));
        Self {
            inner: Box::pin(source),
            request_id,
            content_type,
            idle_timeout,
            terminated: false,
        }
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    pub async fn next_chunk(&mut self) -> Option<Result<Bytes, Error>> {
        if self.terminated {
            return None;
        }
        if let Some(timeout) = self.idle_timeout {
            match tokio::time::timeout(timeout, self.inner.next()).await {
                Ok(Some(Ok(chunk))) => Some(Ok(chunk)),
                Ok(Some(Err(error))) => {
                    self.terminated = true;
                    Some(Err(error))
                }
                Ok(None) => {
                    self.terminated = true;
                    None
                }
                Err(_) => {
                    self.terminated = true;
                    Some(Err(Error::StreamIdleTimeout))
                }
            }
        } else {
            let next = self.inner.next().await;
            self.terminated = next.is_none() || next.as_ref().is_some_and(Result::is_err);
            next
        }
    }
}

impl std::fmt::Debug for AudioContentStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioContentStream")
            .field("request_id", &self.request_id)
            .field("content_type", &self.content_type)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::{AudioEventKind, AudioEventPayload, parse_events};
    use crate::Error;
    use bytes::Bytes;
    use futures_util::StreamExt;
    use std::convert::Infallible;

    fn byte_chunks(input: &[u8], chunk_sizes: &[usize]) -> Vec<Result<Bytes, Infallible>> {
        let mut chunks = Vec::new();
        let mut offset = 0;
        let mut size_index = 0;
        while offset < input.len() {
            let size = chunk_sizes[size_index % chunk_sizes.len()].max(1);
            let end = (offset + size).min(input.len());
            chunks.push(Ok(Bytes::copy_from_slice(&input[offset..end])));
            offset = end;
            size_index += 1;
        }
        chunks
    }

    #[tokio::test]
    async fn parses_speech_events_and_semantic_terminal_across_chunks() {
        let input = concat!(
            "event: speech.audio.delta\ndata: {\"type\":\"speech.audio.delta\",\"audio\":\"AQID\"}\n\n",
            "event: speech.audio.done\ndata: {\"type\":\"speech.audio.done\",\"usage\":{\"input_tokens\":1,\"output_tokens\":2,\"total_tokens\":3}}\n\n",
            "data: {\"type\":\"ignored\"}\n\n"
        );
        let events = parse_events(
            futures_util::stream::iter(byte_chunks(input.as_bytes(), &[1, 4, 2, 9])),
            AudioEventKind::Speech,
            None,
        )
        .collect::<Vec<_>>()
        .await;
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].as_ref().unwrap().event_type, "speech.audio.delta");
        assert!(matches!(
            events[0].as_ref().unwrap().payload,
            AudioEventPayload::Speech(_)
        ));
        assert_eq!(events[1].as_ref().unwrap().event_type, "speech.audio.done");
    }

    #[tokio::test]
    async fn preserves_unknown_audio_event_and_requires_terminal_event() {
        let input = "event: audio.future\ndata: {\"type\":\"audio.future\",\"extra\":1}\n\n";
        let events = parse_events(
            futures_util::stream::iter(byte_chunks(input.as_bytes(), &[2, 3])),
            AudioEventKind::Speech,
            None,
        )
        .collect::<Vec<_>>()
        .await;
        assert_eq!(events.len(), 2);
        assert!(matches!(
            &events[0].as_ref().unwrap().payload,
            AudioEventPayload::Unknown(value) if value["extra"] == 1
        ));
        assert!(matches!(events[1], Err(Error::UnexpectedStreamEnd)));
    }

    #[cfg(feature = "multipart")]
    #[tokio::test]
    async fn parses_transcription_events_until_done() {
        let input = concat!(
            "data: {\"type\":\"transcript.text.delta\",\"delta\":\"hello\",\"logprobs\":[]}\n\n",
            "data: {\"type\":\"transcript.text.done\",\"text\":\"hello\",\"logprobs\":[],\"languages\":[],\"usage\":null}\n\n"
        );
        let events = parse_events(
            futures_util::stream::iter(byte_chunks(input.as_bytes(), &[3, 7, 1])),
            AudioEventKind::Transcription,
            None,
        )
        .collect::<Vec<_>>()
        .await;
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[0].as_ref().unwrap().event_type,
            "transcript.text.delta"
        );
        assert_eq!(
            events[1].as_ref().unwrap().event_type,
            "transcript.text.done"
        );
    }
}
