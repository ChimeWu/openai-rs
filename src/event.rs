use crate::Error;
use crate::generated::KNOWN_RESPONSE_EVENT_TYPES;
use crate::types;
use serde_json::Value;

/// A decoded Responses SSE event with its original JSON payload.
///
/// `typed()` is populated for event types in the OpenAI schema. Unknown event
/// types remain available through `event_type()` and `data()`.
#[derive(Clone, Debug, PartialEq)]
pub struct ResponseStreamEvent {
    event_type: String,
    data: Value,
    typed: Option<types::ResponseStreamEvent>,
}

impl ResponseStreamEvent {
    pub(crate) fn decode(data: &str, sse_event_type: Option<&str>) -> Result<Self, Error> {
        let value: Value = serde_json::from_str(data)?;
        let event_type = value
            .get("type")
            .and_then(Value::as_str)
            .or(sse_event_type)
            .ok_or_else(|| Error::Sse("event is missing its type".to_string()))?
            .to_string();

        let typed = if KNOWN_RESPONSE_EVENT_TYPES.contains(&event_type.as_str()) {
            Some(serde_json::from_value(value.clone()).map_err(Error::Json)?)
        } else {
            None
        };

        Ok(Self {
            event_type,
            data: value,
            typed,
        })
    }

    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    pub fn data(&self) -> &Value {
        &self.data
    }

    pub fn typed(&self) -> Option<&types::ResponseStreamEvent> {
        self.typed.as_ref()
    }

    pub fn into_typed(self) -> Option<types::ResponseStreamEvent> {
        self.typed
    }

    pub fn is_terminal(&self) -> bool {
        is_terminal_event_type(&self.event_type)
    }
}

fn is_terminal_event_type(event_type: &str) -> bool {
    matches!(
        event_type,
        "response.completed" | "response.incomplete" | "response.failed"
    )
}

#[cfg(test)]
mod tests {
    use super::ResponseStreamEvent;
    use crate::types;
    use serde_json::json;

    #[test]
    fn retains_unknown_event_type_and_payload() {
        let event = ResponseStreamEvent::decode(
            r#"{"type":"response.future_event","new_field":{"v":3}}"#,
            Some("message"),
        )
        .unwrap();
        assert_eq!(event.event_type(), "response.future_event");
        assert_eq!(event.data()["new_field"], json!({"v": 3}));
        assert!(event.typed().is_none());
    }

    #[test]
    fn deserializes_known_event_into_schema_generated_union() {
        let event = ResponseStreamEvent::decode(
            r#"{"type":"response.output_text.delta","sequence_number":1,"item_id":"msg_1","output_index":0,"content_index":0,"delta":"hello","logprobs":[]}"#,
            None,
        )
        .unwrap();
        assert_eq!(event.event_type(), "response.output_text.delta");
        assert!(event.typed().is_some());
    }

    #[test]
    fn accepts_compatible_usage_without_cache_write_count() {
        let details: types::ResponseUsageInputTokensDetails =
            serde_json::from_value(json!({ "cached_tokens": 3 })).unwrap();
        assert_eq!(details.cached_tokens, 3);
        assert_eq!(details.cache_write_tokens, None);
    }

    #[test]
    fn classifies_all_responses_terminal_event_types() {
        for event_type in [
            "response.completed",
            "response.incomplete",
            "response.failed",
        ] {
            assert!(super::is_terminal_event_type(event_type));
        }
        assert!(!super::is_terminal_event_type("response.output_text.delta"));
    }
}
