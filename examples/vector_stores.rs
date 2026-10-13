use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;

    let page = client
        .vector_stores()
        .list(&types::vector_stores::ListVectorStoresParams::default())
        .await?;
    for vector_store in &page.data.data {
        println!("{} {}", vector_store.id, vector_store.name);
    }

    Ok(())
}
