use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let params = types::containers::ListContainersParams::default();
    let containers = client.containers().list(&params).await?;
    for container in &containers.data.data {
        println!("{} {}", container.id, container.name);
    }
    Ok(())
}
