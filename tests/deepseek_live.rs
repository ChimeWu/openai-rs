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
async fn deepseek_models_list_decodes_documented_compatible_shape() {
    let models = client()
        .models()
        .list()
        .await
        .expect("DeepSeek Models API returns a compatible model list");
    assert!(
        models
            .data
            .data
            .iter()
            .any(|model| model.id == "deepseek-flash")
    );
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

#[tokio::test]
async fn deepseek_chat_completions_json_and_sse_round_trip() {
    let client = client();
    let request: types::chat_completions::CreateChatCompletionRequest =
        serde_json::from_value(json!({
            "model": "deepseek-flash",
            "messages": [{"role": "user", "content": "Reply with exactly the two words: chat check."}],
            "reasoning_effort": "none",
            "max_tokens": 32
        }))
        .expect("construct request using generated Chat Completions schema types");

    let response = client
        .chat_completions()
        .create(&request)
        .await
        .expect("DeepSeek non-streaming Chat Completions request succeeds");
    assert!(response.raw()["choices"].is_array());
    assert!(response.raw()["choices"][0]["message"]["content"].is_string());
    assert!(response.raw()["usage"].is_object());

    let mut stream = client
        .chat_completions()
        .create_stream(&request)
        .await
        .expect("DeepSeek streaming Chat Completions request succeeds");
    let mut saw_text = false;
    let mut saw_finish = false;
    while let Some(chunk) = stream.next_chunk().await {
        let chunk = chunk.expect("Chat Completions SSE chunk decodes");
        saw_text |= chunk.data()["choices"][0]["delta"]["content"]
            .as_str()
            .is_some_and(|text| !text.is_empty());
        saw_finish |= !chunk.data()["choices"][0]["finish_reason"].is_null();
    }

    assert!(saw_text, "DeepSeek stream should contain a text delta");
    assert!(saw_finish, "DeepSeek stream should contain a finish reason");
}

#[cfg(feature = "multipart")]
#[tokio::test]
async fn deepseek_files_accept_one_shot_multipart_upload_and_cleanup() {
    use bytes::Bytes;
    use futures_util::stream;

    let client = client();
    let request = types::files::CreateFileRequest {
        expires_after: Some(types::files::FileExpirationAfter {
            anchor: types::files::FileExpirationAfterAnchor::CreatedAt,
            seconds: 3600,
        }),
        file: "openai-rs-smoke.gif".to_owned(),
        purpose: types::files::CreateFileRequestPurpose::UserData,
    };
    // A one-pixel GIF fixture; no user or account data is uploaded.
    let contents = Bytes::from_static(&[
        0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xff, 0xff, 0xff, 0x21, 0xf9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x02, 0x44, 0x01, 0x00, 0x3b,
    ]);
    let chunks = stream::iter(vec![Ok::<_, std::io::Error>(contents.clone())]);
    let uploaded = client
        .files()
        .create_stream(&request, chunks)
        .await
        .expect("DeepSeek accepts one-shot multipart file upload");
    let file_id = uploaded.id.clone();

    // Always attempt deletion after upload, even if retrieval or its assertions fail.
    let retrieved = client.files().retrieve(&file_id).await;
    let deleted = client
        .files()
        .delete(&file_id)
        .await
        .expect("DeepSeek deletes the smoke file");
    let retrieved = retrieved.expect("DeepSeek retrieves the uploaded file metadata");

    assert_eq!(uploaded.id, file_id);
    assert_eq!(uploaded.bytes, Some(contents.len() as i64));
    assert_eq!(retrieved.id, file_id);
    assert!(deleted.deleted);
}
