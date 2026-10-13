use openai_rs::types;

#[test]
fn generated_request_and_response_builders_create_serializable_models() {
    let request: types::audio::CreateSpeechRequest = types::audio::CreateSpeechRequest::builder()
        .input("Hello from an OpenAI-compatible provider")
        .model("gpt-4o-mini-tts")
        .voice(types::audio::Voice::VoiceIdsShared(
            types::audio::VoiceIdsShared("marin".to_owned()),
        ))
        .try_into()
        .expect("required speech request fields are provided");
    let request_json = serde_json::to_value(request).expect("speech request serializes");
    assert_eq!(request_json["model"], "gpt-4o-mini-tts");

    let usage: types::embeddings::CreateEmbeddingResponseUsage =
        types::embeddings::CreateEmbeddingResponseUsage::builder()
            .prompt_tokens(0)
            .total_tokens(0)
            .try_into()
            .expect("required embedding usage fields are provided");

    let response: types::embeddings::CreateEmbeddingResponse =
        types::embeddings::CreateEmbeddingResponse::builder()
            .data(Vec::new())
            .model("example-embedding-model")
            .object(types::embeddings::CreateEmbeddingResponseObject::List)
            .usage(usage)
            .try_into()
            .expect("required embedding response fields are provided");
    let response_json = serde_json::to_value(response).expect("embedding response serializes");
    assert_eq!(response_json["object"], "list");
    assert_eq!(response_json["usage"]["total_tokens"], 0);
}
