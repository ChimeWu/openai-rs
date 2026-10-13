use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let page = client
        .batches()
        .list(&types::batches::ListBatchesParams::default())
        .await?;
    for batch in &page.data.data {
        println!("{} {}", batch.id, batch.status);
    }
    Ok(())
}
