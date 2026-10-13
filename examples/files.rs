use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let params = types::files::ListFilesParams::default();
    let files = client.files().list(&params).await?;
    for file in &files.data.data {
        println!("{} {}", file.id, file.filename);
    }
    Ok(())
}
