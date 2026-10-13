use crate::Error;
use crate::ResponseStreamEvent;
use async_stream::stream;
use bytes::Bytes;
use eventsource_stream::Eventsource;
use futures_core::Stream;
use futures_util::StreamExt;
use std::fmt::Display;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ResponseStream {
    inner: Pin<Box<dyn Stream<Item = Result<ResponseStreamEvent, Error>> + Send>>,
}

impl ResponseStream {
    pub(crate) fn new(response: reqwest::Response) -> Self {
        Self {
            inner: Box::pin(parse_sse(response.bytes_stream())),
        }
    }

    pub async fn next_event(&mut self) -> Option<Result<ResponseStreamEvent, Error>> {
        self.inner.next().await
    }
}

fn parse_sse<S, E>(source: S) -> impl Stream<Item = Result<ResponseStreamEvent, Error>> + Send
where
    S: Stream<Item = Result<Bytes, E>> + Send + Unpin + 'static,
    E: Display + Send + 'static,
{
    let mut source = source.eventsource();
    stream! {
            let mut terminal = false;
            while let Some(frame) = source.next().await {
                let frame = match frame {
                    Ok(frame) => frame,
                    Err(error) => {
                        yield Err(Error::Sse(error.to_string()));
                        return;
                    }
                };

                if frame.data.trim() == "[DONE]" {
                    terminal = true;
                    break;
                }
                if frame.data.is_empty() {
                    continue;
                }

                let event = match ResponseStreamEvent::decode(
                    &frame.data,
                    Some(frame.event.as_str()),
                ) {
                    Ok(event) => event,
                    Err(error) => {
                        yield Err(error);
                        return;
                    }
                };
                terminal = event.is_terminal();
                yield Ok(event);
                if terminal {
                    break;
                }
            }

            if !terminal {
                yield Err(Error::UnexpectedStreamEnd);
            }
    }
}

impl Stream for ResponseStream {
    type Item = Result<ResponseStreamEvent, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().inner.as_mut().poll_next(cx)
    }
}

impl std::fmt::Debug for ResponseStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseStream").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::parse_sse;
    use crate::Error;
    use bytes::Bytes;
    use futures_util::StreamExt;
    use std::convert::Infallible;

    fn completed_response() -> serde_json::Value {
        serde_json::json!({
            "id": "resp_fixture",
            "object": "response",
            "created_at": 1,
            "status": "completed",
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
        })
    }

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
    async fn parses_semantic_events_across_arbitrary_byte_boundaries() {
        let delta = serde_json::json!({
            "type": "response.output_text.delta",
            "sequence_number": 1,
            "item_id": "msg_1",
            "output_index": 0,
            "content_index": 0,
            "delta": "你好",
            "logprobs": []
        });
        assert!(
            serde_json::from_value::<crate::types::ResponseStreamEvent>(delta.clone()).is_ok(),
            "generated event must deserialize: {delta}"
        );
        let completed = serde_json::json!({
            "type": "response.completed",
            "sequence_number": 2,
            "response": completed_response()
        });
        let payload = format!(
            "event: response.output_text.delta\r\ndata: {delta}\r\n\r\nevent: response.completed\ndata: {completed}\n\n"
        );
        let chunks = byte_chunks(payload.as_bytes(), &[1, 2, 7, 3, 11]);
        let events = parse_sse(futures_util::stream::iter(chunks))
            .collect::<Vec<_>>()
            .await;

        assert_eq!(events.len(), 2);
        assert_eq!(
            events[0].as_ref().unwrap().event_type(),
            "response.output_text.delta"
        );
        assert_eq!(events[0].as_ref().unwrap().data()["delta"], "你好");
        assert!(events[0].as_ref().unwrap().typed().is_some());
        assert!(events[1].as_ref().unwrap().is_terminal());
    }

    #[tokio::test]
    async fn stops_after_each_schema_typed_terminal_event() {
        for event_type in [
            "response.completed",
            "response.incomplete",
            "response.failed",
        ] {
            let mut response = completed_response();
            response["status"] = serde_json::json!(match event_type {
                "response.completed" => "completed",
                "response.incomplete" => "incomplete",
                _ => "failed",
            });
            if event_type == "response.incomplete" {
                response["incomplete_details"] = serde_json::json!({"reason": "max_output_tokens"});
            }
            if event_type == "response.failed" {
                response["error"] =
                    serde_json::json!({"code": "server_error", "message": "failed"});
            }
            let event = serde_json::json!({
                "type": event_type,
                "sequence_number": 1,
                "response": response
            });
            let frame = format!("event: {event_type}\ndata: {event}\n\n");
            let events = parse_sse(futures_util::stream::iter(vec![Ok::<_, Infallible>(
                Bytes::from(frame),
            )]))
            .collect::<Vec<_>>()
            .await;

            assert_eq!(events.len(), 1, "{event_type} should finish the stream");
            let terminal = events[0].as_ref().unwrap();
            assert_eq!(terminal.event_type(), event_type);
            assert!(terminal.typed().is_some());
        }
    }

    #[tokio::test]
    async fn preserves_unknown_event_payload_and_reports_unterminated_stream() {
        let payload = b"event: response.future_event\ndata: {\"type\":\"response.future_event\",\"new_field\":7}\n\n";
        let events = parse_sse(futures_util::stream::iter(vec![Ok::<_, Infallible>(
            Bytes::from_static(payload),
        )]))
        .collect::<Vec<_>>()
        .await;

        assert_eq!(events.len(), 2);
        let event = events[0].as_ref().unwrap();
        assert_eq!(event.event_type(), "response.future_event");
        assert_eq!(event.data()["new_field"], 7);
        assert!(event.typed().is_none());
        assert!(matches!(events[1], Err(Error::UnexpectedStreamEnd)));
    }

    #[tokio::test]
    async fn rejects_malformed_known_event_and_accepts_done_terminator() {
        let malformed = b"data: {\"type\":\"response.output_text.delta\"}\n\n";
        let malformed_events = parse_sse(futures_util::stream::iter(vec![Ok::<_, Infallible>(
            Bytes::from_static(malformed),
        )]))
        .collect::<Vec<_>>()
        .await;
        assert!(matches!(malformed_events.as_slice(), [Err(Error::Json(_))]));

        let done = b"data: [DONE]\n\n";
        let done_events = parse_sse(futures_util::stream::iter(vec![Ok::<_, Infallible>(
            Bytes::from_static(done),
        )]))
        .collect::<Vec<_>>()
        .await;
        assert!(done_events.is_empty());
    }
}
