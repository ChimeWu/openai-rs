#![cfg(feature = "live-api")]

use openai_rs::{CreateOptions, OpenAIClient, RequestOptions, types};
use serde_json::json;
use std::time::Duration;

fn request(prompt: &str) -> types::CreateResponse {
    serde_json::from_value(json!({
        "model": "deepseek-flash",
        "input": prompt,
        "max_output_tokens": 128
    }))
    .expect("construct request using generated Responses schema types")
}

fn client() -> OpenAIClient {
    let token =
        std::env::var("DEEPSEEK_TOKEN").expect("DEEPSEEK_TOKEN must be set for live API tests");
    OpenAIClient::builder()
        .api_key(token)
        .base_url("https://api.deepseek.com")
        .timeout(Duration::from_secs(90))
        .build()
        .expect("build DeepSeek API client")
}

#[tokio::test]
async fn deepseek_responses_json_and_sse_round_trip() {
    let client = client();

    let options = CreateOptions::default().request(RequestOptions::default().header(
        "x-sdk-smoke".parse().unwrap(),
        "request-header".parse().unwrap(),
    ));
    let response = client
        .responses()
        .create_with_options(
            &request("Reply with exactly the two words: live check."),
            options,
        )
        .await
        .expect("DeepSeek non-streaming Responses request succeeds");
    assert!(matches!(
        response.status,
        Some(types::ResponseStatus::Completed | types::ResponseStatus::Incomplete)
    ));
    assert!(response.raw()["usage"].is_object());
    assert!(response.raw()["output"].is_array());

    let options = CreateOptions::default().request(RequestOptions::default().header(
        "x-sdk-smoke".parse().unwrap(),
        "request-header".parse().unwrap(),
    ));
    let mut stream = client
        .responses()
        .create_stream_with_options(
            &request("Reply with exactly the two words: stream check."),
            options,
        )
        .await
        .expect("DeepSeek streaming Responses request succeeds");
    let mut saw_text = false;
    let mut saw_terminal = false;
    while let Some(event) = stream.next_event().await {
        let event = event.expect("Responses SSE event decodes");
        assert_ne!(event.event_type(), "response.failed");
        saw_text |= event.event_type() == "response.output_text.delta";
        saw_terminal |= matches!(
            event.event_type(),
            "response.completed" | "response.incomplete"
        );
    }

    assert!(saw_text, "DeepSeek stream should contain a text delta");
    assert!(
        saw_terminal,
        "DeepSeek stream should end in a semantic terminal event"
    );
}
