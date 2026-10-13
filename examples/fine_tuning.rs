use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;

    let page = client
        .fine_tuning()
        .list_jobs(&types::fine_tuning::ListPaginatedFineTuningJobsParams::default())
        .await?;
    for job in &page.data.data {
        println!("{} {}", job.id, job.status);
    }

    Ok(())
}
