use crate::Error;
use crate::types;
use async_stream::stream;
use bytes::Bytes;
use eventsource_stream::Eventsource;
use futures_core::Stream;
use futures_util::StreamExt;
use std::fmt::Display;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

/// A Chat Completions SSE stream. It completes when the server sends `[DONE]`.
pub struct ChatCompletionStream {
    inner: Pin<Box<dyn Stream<Item = Result<ChatCompletionStreamChunk, Error>> + Send>>,
}

impl ChatCompletionStream {
    pub(crate) fn new(response: reqwest::Response, idle_timeout: Option<Duration>) -> Self {
        Self {
            inner: Box::pin(parse_chat_sse(response.bytes_stream(), idle_timeout)),
        }
    }

    pub async fn next_chunk(&mut self) -> Option<Result<ChatCompletionStreamChunk, Error>> {
        self.inner.next().await
    }
}

/// A typed Chat Completions chunk with its original JSON payload.
#[derive(Clone, Debug, PartialEq)]
pub struct ChatCompletionStreamChunk {
    data: serde_json::Value,
    typed: types::chat_completions::CreateChatCompletionStreamResponse,
}

impl ChatCompletionStreamChunk {
    pub fn data(&self) -> &serde_json::Value {
        &self.data
    }

    pub fn typed(&self) -> &types::chat_completions::CreateChatCompletionStreamResponse {
        &self.typed
    }
}

fn parse_chat_sse<S, E>(
    source: S,
    idle_timeout: Option<Duration>,
) -> impl Stream<Item = Result<ChatCompletionStreamChunk, Error>> + Send
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
            if frame.data.trim() == "[DONE]" {
                terminal = true;
                break;
            }
            if frame.data.is_empty() {
                continue;
            }

            let data: serde_json::Value = match serde_json::from_str(&frame.data) {
                Ok(data) => data,
                Err(error) => {
                    yield Err(Error::Json(error));
                    return;
                }
            };
            let typed = match serde_json::from_value(data.clone()) {
                Ok(typed) => typed,
                Err(error) => {
                    yield Err(Error::Json(error));
                    return;
                }
            };
            yield Ok(ChatCompletionStreamChunk { data, typed });
        }

        if !terminal {
            yield Err(Error::UnexpectedStreamEnd);
        }
    }
}

impl Stream for ChatCompletionStream {
    type Item = Result<ChatCompletionStreamChunk, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().inner.as_mut().poll_next(cx)
    }
}

impl std::fmt::Debug for ChatCompletionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChatCompletionStream")
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::parse_chat_sse;
    use crate::Error;
    use bytes::Bytes;
    use futures_util::StreamExt;
    use std::convert::Infallible;
    use std::time::Duration;

    #[tokio::test]
    async fn parses_chat_chunks_and_done_across_arbitrary_boundaries() {
        let chunk = serde_json::json!({
            "id": "chatcmpl_1",
            "object": "chat.completion.chunk",
            "created": 1,
            "model": "gpt-6-astra",
            "choices": [{
                "index": 0,
                "delta": {"content": "hello"},
                "finish_reason": null
            }]
        });
        let input = format!("data: {chunk}\n\ndata: [DONE]\n\n");
        let pieces = input
            .as_bytes()
            .chunks(3)
            .map(|piece| Ok::<_, Infallible>(Bytes::copy_from_slice(piece)))
            .collect::<Vec<_>>();
        let items = parse_chat_sse(futures_util::stream::iter(pieces), None)
            .collect::<Vec<_>>()
            .await;

        assert_eq!(items.len(), 1);
        let item = items[0].as_ref().unwrap();
        assert_eq!(item.data()["choices"][0]["delta"]["content"], "hello");
        assert_eq!(item.typed().id, "chatcmpl_1");
    }

    #[tokio::test]
    async fn reports_missing_done_marker_and_idle_timeout() {
        let premature = b"data: {\"id\":\"chatcmpl_1\",\"object\":\"chat.completion.chunk\",\"created\":1,\"model\":\"gpt-6-astra\",\"choices\":[]}\n\n";
        let items = parse_chat_sse(
            futures_util::stream::iter(vec![Ok::<_, Infallible>(Bytes::from_static(premature))]),
            None,
        )
        .collect::<Vec<_>>()
        .await;
        assert!(matches!(
            items.last(),
            Some(Err(Error::UnexpectedStreamEnd))
        ));

        let pending = futures_util::stream::pending::<Result<Bytes, Infallible>>();
        let items = parse_chat_sse(pending, Some(Duration::ZERO))
            .collect::<Vec<_>>()
            .await;
        assert!(matches!(items.as_slice(), [Err(Error::StreamIdleTimeout)]));
    }
}
