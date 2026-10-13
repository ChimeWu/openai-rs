#![cfg(feature = "responses")]

use openai_api_types::{common, responses};

#[test]
fn request_response_error_and_event_round_trip() {
    let request: responses::CreateResponse = serde_json::from_value(serde_json::json!({
        "model": "gpt-5",
        "input": "hello"
    }))
    .unwrap();
    let request_json = serde_json::to_value(request).unwrap();
    assert_eq!(request_json["model"], "gpt-5");
    assert_eq!(request_json["input"], "hello");

    let error: common::ErrorResponse = serde_json::from_value(serde_json::json!({
        "error": {
            "type": "invalid_request_error",
            "message": "bad request",
            "param": null,
            "code": null
        }
    }))
    .unwrap();
    assert_eq!(error.error.message, "bad request");

    let event: responses::ResponseTextDeltaEvent = serde_json::from_value(serde_json::json!({
        "type": "response.output_text.delta",
        "sequence_number": 1,
        "item_id": "msg_1",
        "output_index": 0,
        "content_index": 0,
        "delta": "hello",
        "logprobs": []
    }))
    .unwrap();
    assert_eq!(event.delta, "hello");
    assert_eq!(
        serde_json::to_value(event).unwrap()["type"],
        "response.output_text.delta"
    );
}
