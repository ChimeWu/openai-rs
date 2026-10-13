use openai_rs::OpenAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let models = client.models().list().await?;

    for model in &models.data.data {
        println!("{} ({})", model.id, model.owned_by);
    }
    Ok(())
}
