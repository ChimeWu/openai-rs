# openai-rs

一个可独立使用的 Rust OpenAI Responses API SDK。请求、响应和流事件类型从 OpenAI 官方 OpenAPI 规范生成；HTTP 客户端与 SSE 运行时代码独立实现，不依赖 Codex 内部 crate。

## 使用

```toml
[dependencies]
openai-rs = { git = "https://github.com/ChimeWu/openai-rs" }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust,no_run
use openai_rs::{CreateOptions, OpenAIClient, RequestOptions, types};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::builder().api_key(api_key).build()?;
    let request: types::CreateResponse = serde_json::from_value(serde_json::json!({
        "model": "gpt-5.4",
        "input": "用一句话介绍 Rust。"
    }))?;

    let options = CreateOptions::default().request(RequestOptions::default().header(
        "x-client-tag".parse()?,
        "example".parse()?,
    ));
    let response = client
        .responses()
        .create_with_options(&request, options)
        .await?;
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
    let mut events = client.responses().create_stream(request).await?;
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

Responses 操作通过 `client.responses()` 访问。创建请求可使用 `CreateOptions` 附加单次请求 headers 和 schema 尚未覆盖的顶层字段；其他端点可使用 `RequestOptions` 附加 headers。`Authorization`、`Content-Type` 和 `Accept` 由 SDK 管理，不能覆盖。

SDK 支持创建与流式创建、响应 JSON 检索与 SSE 恢复、删除、取消、输入项分页、token 计数和上下文 compact。列表端点返回单页结果；使用返回的 `last_id` 作为 `ListInputItemsParams.after` 可继续获取下一页。JSON 响应以 `ApiResponse<T>` 返回，`data` 保存生成的强类型结果，`raw()` 和 `request_id()` 分别提供原始 JSON 与请求 ID。

例如，检索已保存的 response 并继续读取下一页输入项：

```rust,no_run
use openai_rs::{OpenAIClient, types};

async fn inspect(client: &OpenAIClient, response_id: &str) -> Result<(), openai_rs::Error> {
    let response = client
        .responses()
        .retrieve(response_id, &types::RetrieveResponseParams::default())
        .await?;
    println!("response status: {:?}", response.status);

    let page = client
        .responses()
        .list_input_items(response_id, &types::ListInputItemsParams::default())
        .await?;
    if page.has_more {
        let next_page = types::ListInputItemsParams {
            after: Some(page.last_id.clone()),
            ..Default::default()
        };
        let _ = client
            .responses()
            .list_input_items(response_id, &next_page)
            .await?;
    }
    Ok(())
}
```

`count_input_tokens` 与 `compact` 分别接收生成的 `types::TokenCountsBody` 和 `types::CompactResponseMethodPublicBody`。`retrieve_stream` 接收 `types::RetrieveResponseStreamParams`，可设置 `starting_after` 从指定事件序号继续读取。

0.2.0 将 0.1.0 的 `client.create(...)` 和 `client.create_stream(...)` 改为 `client.responses().create(...)` 和 `client.responses().create_stream(...)`。单次请求自定义项统一通过 `CreateOptions` 和 `RequestOptions` 传递。

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
