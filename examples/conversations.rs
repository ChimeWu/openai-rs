use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let request: types::conversations::CreateConversationBody =
        serde_json::from_value(serde_json::json!({"metadata": {"purpose": "example"}}))?;
    let conversation = client.conversations().create(&request).await?;
    println!("{}", conversation.data.id);
    Ok(())
}
