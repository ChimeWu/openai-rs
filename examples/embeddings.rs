use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let request: types::embeddings::CreateEmbeddingRequest =
        serde_json::from_value(serde_json::json!({
            "model": "text-embedding-3-small",
            "input": "Rust SDK example"
        }))?;
    let response = client.embeddings().create(&request).await?;
    let dimensions = match &response.data.data[0].embedding {
        types::embeddings::EmbeddingEmbedding::Array(values) => values.len(),
        types::embeddings::EmbeddingEmbedding::String(value) => value.len(),
    };
    println!("dimensions: {dimensions}");
    Ok(())
}
