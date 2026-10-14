use futures_util::StreamExt;
use openai_rs::{OpenAIClient, WebSocketEvent};
use serde_json::{Value, json};
use std::error::Error;
use std::io;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder()
        .api_key(api_key)
        .stream_idle_timeout(Duration::from_secs(60))
        .build()?;

    let connection = client
        .realtime()
        .connect_websocket("gpt-realtime-2.1")
        .await?;
    let (sender, mut receiver) = connection.split();

    sender
        .send_json(&json!({
            "type": "session.update",
            "session": {
                "type": "realtime",
                "instructions": "Answer briefly in plain text.",
                "output_modalities": ["text"]
            }
        }))
        .await?;

    loop {
        let event = next_json_event(&mut receiver).await?;
        match event["type"].as_str() {
            Some("session.updated") => break,
            Some("error") => return Err(server_error(&event).into()),
            _ => println!("{}", event["type"].as_str().unwrap_or("unknown event")),
        }
    }

    sender
        .send_json(&json!({
            "type": "conversation.item.create",
            "item": {
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": "Say hello in one sentence."}]
            }
        }))
        .await?;
    sender
        .send_json(&json!({"type": "response.create"}))
        .await?;

    loop {
        let event = next_json_event(&mut receiver).await?;
        match event["type"].as_str() {
            Some("response.done") => break,
            Some("error") => return Err(server_error(&event).into()),
            _ => println!("{}", event["type"].as_str().unwrap_or("unknown event")),
        }
    }

    sender.close().await?;
    Ok(())
}

async fn next_json_event(
    socket: &mut openai_rs::WebSocketReceiver<openai_rs::types::realtime::RealtimeServerEvent>,
) -> Result<Value, Box<dyn Error>> {
    match socket
        .next()
        .await
        .ok_or_else(|| io::Error::other("Realtime WebSocket closed before the expected event"))??
    {
        WebSocketEvent::Event(event) => Ok(serde_json::to_value(event)?),
        WebSocketEvent::Unknown(event) => Ok(event),
        WebSocketEvent::Binary(_) => Err(io::Error::other("expected a JSON Realtime event").into()),
    }
}

fn server_error(event: &Value) -> io::Error {
    io::Error::other(format!("Realtime server error: {event}"))
}
