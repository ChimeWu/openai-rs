use openai_rs::{OpenAIClient, WebSocketMessage, types};
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

    let mut socket = client.live().connect_websocket().await?;
    let start: types::live::LiveSessionStartEvent = serde_json::from_value(json!({
        "type": "session.start",
        "event_id": "rust-example-start",
        "session": {
            "model": "gpt-live-1",
            "instructions": "Be concise.",
            "audio": {
                "format": {"type": "audio/pcm", "rate": 24000},
                "output": {"voice": "marin"}
            }
        }
    }))?;
    socket.send_json(&start).await?;

    loop {
        let event = next_json_event(&mut socket).await?;
        match event["type"].as_str() {
            Some("session.started") => {
                println!("Live session started");
                break;
            }
            Some("error") => return Err(server_error(&event).into()),
            _ => println!("{}", event["type"].as_str().unwrap_or("unknown event")),
        }
    }

    let close: types::live::LiveSessionCloseParam =
        serde_json::from_value(json!({"type": "session.close"}))?;
    socket.send_json(&close).await?;

    loop {
        let event = next_json_event(&mut socket).await?;
        match event["type"].as_str() {
            Some("session.closed") => {
                println!("Live session closed");
                break;
            }
            Some("error") => return Err(server_error(&event).into()),
            _ => println!("{}", event["type"].as_str().unwrap_or("unknown event")),
        }
    }

    socket.close().await?;
    Ok(())
}

async fn next_json_event(
    socket: &mut openai_rs::WebSocketConnection,
) -> Result<Value, Box<dyn Error>> {
    match socket
        .next_message()
        .await
        .ok_or_else(|| io::Error::other("Live WebSocket closed before the expected event"))??
    {
        WebSocketMessage::Json(event) => Ok(event),
        WebSocketMessage::Binary(_) => Err(io::Error::other("expected a JSON Live event").into()),
    }
}

fn server_error(event: &Value) -> io::Error {
    io::Error::other(format!("Live server error: {event}"))
}
