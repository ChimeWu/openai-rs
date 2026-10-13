use openai_rs::OpenAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let event_types = client.webhooks().list_event_types().await?;
    for event_type in &event_types.data.data {
        println!("{event_type}");
    }
    Ok(())
}
