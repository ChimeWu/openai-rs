use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;

    let socket = client.responses().connect_websocket().await?;
    let request: types::CreateResponse = serde_json::from_value(serde_json::json!({
        "model": "gpt-4.1-mini",
        "input": "Return one short greeting."
    }))?;
    let mut response = socket
        .create_response(Some("example-lane".to_owned()), &request)
        .await?;
    while let Some(event) = response.next_event().await {
        println!("{}", event?.event.event_type());
    }
    socket.close().await?;
    Ok(())
}
