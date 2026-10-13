# openai-rs

一个可独立使用的 Rust OpenAI Responses API SDK。请求、响应和流事件类型从 OpenAI 官方 OpenAPI 规范生成；HTTP 客户端与 SSE 运行时代码独立实现，不依赖 Codex 内部 crate。

## 使用

```toml
[dependencies]
openai-rs = { git = "https://github.com/REPLACE_WITH_OWNER/openai-rs" }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

将示例中的 Git URL 替换为本项目仓库地址。

```rust,no_run
use openai_rs::{OpenAIClient, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let request: types::CreateResponse = serde_json::from_value(serde_json::json!({
        "model": "gpt-5.4",
        "input": "用一句话介绍 Rust。"
    }))?;

    let response = client.create(&request).await?;
    println!("{}", response.output_text.as_deref().unwrap_or(""));
    Ok(())
}
```

异步 API 使用调用方提供的 Tokio runtime。自定义兼容服务地址时，将 API 根地址传给 `base_url`：OpenAI 默认地址是 `https://api.openai.com/v1`，DeepSeek 地址是 `https://api.deepseek.com`。

流式调用返回 `ResponseStream`，事件包含生成类型和原始 JSON；未知事件保留原始类型与 payload：

```rust,no_run
use openai_rs::{OpenAIClient, types};

async fn stream(client: &OpenAIClient, request: &types::CreateResponse)
    -> Result<(), Box<dyn std::error::Error>>
{
    let mut events = client.create_stream(request).await?;
    while let Some(event) = events.next_event().await {
        let event = event?;
        if event.event_type() == "response.output_text.delta" {
            if let Some(delta) = event.data().get("delta").and_then(serde_json::Value::as_str) {
                print!("{delta}");
            }
        }
    }
    Ok(())
}
```

可用 `create_with_extensions` 或 `create_stream_with_extensions` 传递官方 schema 尚未覆盖的请求顶层字段。

## 验证

离线检查与真实 DeepSeek API smoke 分开运行。项目不启动本地 mock HTTP 服务。

```sh
cargo test
DEEPSEEK_TOKEN="$DEEPSEEK_TOKEN" cargo test --features live-api --test deepseek_live -- --nocapture
```

在线检查会发送简短的真实请求并按 DeepSeek 返回计费。不要把 token 写进仓库、日志或命令输出。

## 更新生成的协议类型

固定 OpenAI OpenAPI 提交、文件 SHA-256 和生成器版本记录在 `spec/VERSION.json`。安装固定生成器后可检查生成文件：

```sh
cargo install cargo-typify --version 0.8.0 --locked
scripts/generate-types.sh --check
```

更新 OpenAPI 提交后运行 `scripts/update-schema.sh` 和 `scripts/generate-types.sh --write`，并审查 schema snapshot 与生成 Rust 类型的变化。OpenAI 规范随仓库 MIT 许可证一并保存在 `spec/`。
