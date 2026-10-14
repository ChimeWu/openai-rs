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

    /// Maps chat chunks to their text deltas, ignoring chunks without text.
    pub fn text_deltas(self) -> impl Stream<Item = Result<String, Error>> + Send {
        self.flat_map(|item| {
            let deltas = match item {
                Err(error) => vec![Err(error)],
                Ok(chunk) => chunk.data["choices"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|choice| choice["delta"]["content"].as_str())
                    .filter(|text| !text.is_empty())
                    .map(|text| Ok(text.to_owned()))
                    .collect(),
            };
            futures_util::stream::iter(deltas)
        })
    }

    /// Consumes the stream and aggregates text, finish reasons, and final usage.
    pub async fn collect_text(mut self) -> Result<ChatCompletionText, Error> {
        use std::collections::BTreeMap;
        let mut choices: BTreeMap<i64, ChatCompletionChoiceText> = BTreeMap::new();
        let mut usage = None;
        while let Some(item) = self.next().await {
            let chunk = item?;
            if let Some(value) = chunk.typed.usage.clone() {
                usage = Some(value);
            }
            let raw_choices = chunk.data["choices"].as_array();
            for choice in &chunk.typed.choices {
                let output =
                    choices
                        .entry(choice.index)
                        .or_insert_with(|| ChatCompletionChoiceText {
                            index: choice.index,
                            ..Default::default()
                        });
                if let Some(delta) = raw_choices
                    .and_then(|values| values.iter().find(|value| value["index"] == choice.index))
                    .and_then(|value| value["delta"]["content"].as_str())
                {
                    output.text.push_str(delta);
                }
                if choice.finish_reason.is_some() {
                    output.finish_reason = choice.finish_reason.clone();
                }
            }
        }
        Ok(ChatCompletionText {
            choices: choices.into_values().collect(),
            usage,
        })
    }
}

/// Text and termination metadata for one Chat Completions choice.
#[derive(Clone, Debug, PartialEq)]
pub struct ChatCompletionChoiceText {
    pub index: i64,
    pub text: String,
    pub finish_reason:
        Option<types::chat_completions::CreateChatCompletionStreamResponseChoicesItemFinishReason>,
}

/// Aggregated Chat Completions text, finish reasons, and optional usage.
#[derive(Clone, Debug, PartialEq)]
pub struct ChatCompletionText {
    pub choices: Vec<ChatCompletionChoiceText>,
    pub usage: Option<crate::types::common::CompletionUsage>,
}

impl Default for ChatCompletionChoiceText {
    fn default() -> Self {
        Self {
            index: 0,
            text: String::new(),
            finish_reason: None,
        }
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
    use super::{ChatCompletionStream, parse_chat_sse};
    use crate::{Error, types};
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

    #[tokio::test]
    async fn aggregates_chat_text_by_choice_and_retains_finish_reason_and_usage() {
        let first = serde_json::json!({
            "id":"chat_1","object":"chat.completion.chunk","created":1,"model":"model",
            "choices":[{"index":2,"delta":{"content":"hi"},"finish_reason":null}]
        });
        let final_chunk = serde_json::json!({
            "id":"chat_1","object":"chat.completion.chunk","created":1,"model":"model",
            "choices":[{"index":2,"delta":{},"finish_reason":"stop"}],
            "usage":{"completion_tokens":1,"prompt_tokens":2,"total_tokens":3}
        });
        let input = format!("data: {first}\n\ndata: {final_chunk}\n\ndata: [DONE]\n\n");
        let pieces = input
            .as_bytes()
            .chunks(5)
            .map(|piece| Ok::<_, Infallible>(Bytes::copy_from_slice(piece)))
            .collect::<Vec<_>>();
        let stream = ChatCompletionStream {
            inner: Box::pin(parse_chat_sse(futures_util::stream::iter(pieces), None)),
        };
        let text = stream.collect_text().await.unwrap();
        assert_eq!(text.choices.len(), 1);
        assert_eq!(text.choices[0].index, 2);
        assert_eq!(text.choices[0].text, "hi");
        assert_eq!(text.choices[0].finish_reason, Some(types::chat_completions::CreateChatCompletionStreamResponseChoicesItemFinishReason::Stop));
        assert_eq!(text.usage.unwrap().total_tokens, 3);

        let pieces = input
            .as_bytes()
            .chunks(7)
            .map(|piece| Ok::<_, Infallible>(Bytes::copy_from_slice(piece)))
            .collect::<Vec<_>>();
        let stream = ChatCompletionStream {
            inner: Box::pin(parse_chat_sse(futures_util::stream::iter(pieces), None)),
        };
        assert_eq!(
            stream
                .text_deltas()
                .map(Result::unwrap)
                .collect::<Vec<_>>()
                .await,
            vec!["hi".to_owned()]
        );
    }
}
