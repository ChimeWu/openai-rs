use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let request: types::chat_completions::CreateChatCompletionRequest =
        serde_json::from_value(serde_json::json!({
            "model": "gpt-6-astra",
            "messages": [{"role": "user", "content": "用一句话介绍 Rust。"}]
        }))?;

    let response = client.chat_completions().create(&request).await?;
    println!(
        "{}",
        response.choices[0]
            .message
            .content
            .as_deref()
            .unwrap_or_default()
    );
    Ok(())
}
