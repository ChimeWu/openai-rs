use bytes::Bytes;
use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let bytes = Bytes::from_static(b"{\"input\":\"hello\"}\n");
    let byte_length = bytes.len();
    let contents = futures_util::stream::iter(vec![Ok(bytes)]);
    let request: types::uploads::CreateUploadRequest = serde_json::from_value(serde_json::json!({
        "bytes": byte_length,
        "filename": "training.jsonl",
        "mime_type": "text/jsonl",
        "purpose": "fine-tune"
    }))?;

    let upload = client.uploads().create(&request).await?;
    let part = client
        .uploads()
        .add_part_stream(&upload.data.id, contents)
        .await?;
    let complete = types::uploads::CompleteUploadRequest {
        md5: None,
        part_ids: vec![part.data.id],
    };
    let file_upload = client
        .uploads()
        .complete(&upload.data.id, &complete)
        .await?;
    println!("{} {}", file_upload.data.id, file_upload.data.status);
    Ok(())
}
