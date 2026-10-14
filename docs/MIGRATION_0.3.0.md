# 迁移到 openai-rs 0.3.0

0.3.0 将 SSE、字节流和 WebSocket 接收端统一为 `futures_core::Stream`。旧的专用读取方法已移除；读取流时，在项目中添加 `futures-util` 并导入 `StreamExt`：

```toml
[dependencies]
openai-rs = { git = "https://github.com/ChimeWu/openai-rs", features = ["websocket", "multipart"] }
futures-util = "0.3"
```

## SSE 与字节流

将 `next_event()`、`next_chunk()` 和 `next_message()` 换成 `StreamExt::next()`。每项仍是 `Result<T, Error>`；正常协议终止或字节流 EOF 返回 `None`，流错误只报告一次，随后返回 `None`。丢弃 SSE 或字节流会取消底层读取。

```rust,no_run
use futures_util::StreamExt;
use openai_rs::{OpenAIClient, types};

async fn read_response(client: &OpenAIClient, request: &types::CreateResponse)
    -> Result<(), Box<dyn std::error::Error>>
{
    let mut events = client.responses().create_stream(request).await?;
    while let Some(event) = events.next().await {
        println!("{}", event?.event_type());
    }
    Ok(())
}
```

## 文本聚合

Responses 和 Chat Completions 流都提供 `text_deltas()` 与消费整个流的 `collect_text()`。Responses 聚合结果含文本和最终 `Response`。Chat 聚合结果的 `choices` 按 choice index 排序，每项含文本与 finish reason；最终 usage 在 `usage` 中，服务端未提供时为 `None`。

```rust,no_run
use openai_rs::{OpenAIClient, types};

async fn collect(client: &OpenAIClient, request: &types::CreateResponse)
    -> Result<(), Box<dyn std::error::Error>>
{
    let result = client.responses().create_stream(request).await?.collect_text().await?;
    println!("{} ({})", result.text, result.response.id);
    Ok(())
}
```

## Realtime 与 Live WebSocket

`connect_websocket()` 返回的连接通过 `split(self)` 分成 `WebSocketSender` 和类型化 `WebSocketReceiver<T>`。Sender 提供 `send_json()`、`send_binary()` 和 `close()`；receiver 是 `Stream<Item = Result<WebSocketEvent<T>, Error>>`。已知事件放在 `WebSocketEvent::Event`，schema 未知事件保留在 `WebSocketEvent::Unknown(Value)`，二进制帧放在 `WebSocketEvent::Binary(Bytes)`。

```rust,no_run
use futures_util::StreamExt;
use openai_rs::{OpenAIClient, WebSocketEvent};

async fn realtime(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    let connection = client.realtime().connect_websocket("gpt-realtime-2.1").await?;
    let (sender, mut receiver) = connection.split();
    sender.send_json(&serde_json::json!({"type":"response.create"})).await?;
    while let Some(event) = receiver.next().await {
        match event? {
            WebSocketEvent::Event(event) => println!("{event:?}"),
            WebSocketEvent::Unknown(value) => println!("unknown: {value}"),
            WebSocketEvent::Binary(bytes) => println!("{} audio bytes", bytes.len()),
        }
    }
    sender.close().await?;
    Ok(())
}
```

丢弃 receiver 会发起连接关闭，并使后续 sender 操作返回 `Error::WebSocketClosed`。Responses WebSocket 继续使用专用 `ResponsesWebSocket` 和 `create_response(stream_id, request)`；返回的 `ResponseWebSocketStream` 同样通过 `StreamExt::next()` 消费。
