use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let request: types::images::CreateImageRequest = serde_json::from_value(serde_json::json!({
        "model": "gpt-image-1.5",
        "prompt": "A small red kite flying above a quiet beach",
        "stream": true,
        "partial_images": 2
    }))?;

    let mut events = client.images().generate_stream(&request).await?;
    while let Some(event) = events.next_event().await {
        let event = event?;
        println!("{}", event.event_type);
    }
    Ok(())
}
