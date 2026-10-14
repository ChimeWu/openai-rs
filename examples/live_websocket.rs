use futures_util::StreamExt;
use openai_rs::{OpenAIClient, WebSocketEvent, types};
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

    let connection = client.live().connect_websocket().await?;
    let (sender, mut receiver) = connection.split();
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
    sender.send_json(&start).await?;

    loop {
        let event = next_json_event(&mut receiver).await?;
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
    sender.send_json(&close).await?;

    loop {
        let event = next_json_event(&mut receiver).await?;
        match event["type"].as_str() {
            Some("session.closed") => {
                println!("Live session closed");
                break;
            }
            Some("error") => return Err(server_error(&event).into()),
            _ => println!("{}", event["type"].as_str().unwrap_or("unknown event")),
        }
    }

    sender.close().await?;
    Ok(())
}

async fn next_json_event(
    socket: &mut openai_rs::WebSocketReceiver<types::live::LiveServerEvent>,
) -> Result<Value, Box<dyn Error>> {
    match socket
        .next()
        .await
        .ok_or_else(|| io::Error::other("Live WebSocket closed before the expected event"))??
    {
        WebSocketEvent::Event(event) => Ok(serde_json::to_value(event)?),
        WebSocketEvent::Unknown(event) => Ok(event),
        WebSocketEvent::Binary(_) => Err(io::Error::other("expected a JSON Live event").into()),
    }
}

fn server_error(event: &Value) -> io::Error {
    io::Error::other(format!("Live server error: {event}"))
}
