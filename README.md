# openai-rs

一个可独立使用的 Rust OpenAI API SDK，包含 Responses、Chat Completions、Audio、Models、Embeddings、Moderations、Files/Uploads、Batches、Webhooks、Content Provenance、Images、Conversations、Containers、Vector Stores、Fine-tuning、Live 和 Realtime API。请求、响应和流事件类型从 OpenAI 官方 OpenAPI 规范生成；HTTP 客户端与 SSE 运行时代码独立实现，不依赖 Codex 内部 crate。

## 使用

```toml
[dependencies]
openai-rs = { git = "https://github.com/ChimeWu/openai-rs", features = ["multipart"] }
bytes = "1"
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

流式调用返回 `ResponseStream`，事件包含生成类型和原始 JSON；未知事件保留原始类型与 payload。可通过 `ResponseStream::request_id()` 读取响应头中的请求 ID；`response.failed` 映射为 `Error::ResponseFailed`，并可通过 `Error::request_id()` 取得同一请求 ID。正常完成和 incomplete 事件结束流；若终止事件前 EOF，则返回流错误：

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

客户端可分别配置连接超时、JSON 请求/流连接建立超时和 SSE idle timeout：`connect_timeout` 限制连接建立，`request_timeout` 限制收到 JSON 响应及建立 SSE 连接的等待时间，`stream_idle_timeout` 限制 SSE 两个事件之间的最长空闲时间。三者默认都不设置，因此长时间生成的 SSE 不会被总时限截断。

重试默认关闭。通过 `ClientBuilder::retry_policy(RetryPolicy::new(...))` 显式启用后，最多重试三次连接错误、408、429 和 5xx；GET/HEAD 可重试，写请求必须在本次请求 options 中提供 `idempotency-key`。JSON 请求体可重放。SSE 请求只会在流建立前重试；WebSocket 只会在握手成功前重试，不会重放已开始的会话。退避带 jitter 并受最大延迟限制；`Retry-After` 的秒数及三种 HTTP-date 格式会优先用于等待，HTTP 错误保留原始 `Retry-After` 值。

SDK 支持创建与流式创建、响应 JSON 检索与 SSE 恢复、删除、取消、输入项分页、token 计数和上下文 compact。列表端点可返回单页，也可通过 `list_input_items_pages` 逐页异步读取；流中的每一项都是完整 `ApiResponse<T>`，保留原始 JSON 与 request ID。自动分页遇到空或重复 cursor 时会返回 `InvalidPaginationCursor`。JSON 响应以 `ApiResponse<T>` 返回，`data` 保存生成的强类型结果，`raw()` 和 `request_id()` 分别提供原始 JSON 与请求 ID。

Chat Completions 的流式创建返回 `ChatCompletionStream`，逐项产出生成的 chunk 类型与原始 JSON，并在 `data: [DONE]` 时正常结束。若连接在终止标记前结束或超过配置的 idle timeout，会返回流错误。

Models API 通过 `client.models()` 访问，支持列出、检索和删除模型；对应响应结构由固定 OpenAPI snapshot 生成。为兼容 DeepSeek 官方 `GET /models` 响应中省略 `created` 时间戳的示例，该字段在 Rust 类型中是 `Option<i64>`；OpenAI 返回该字段时仍会保留。Chat Completions 仍受支持，新项目优先考虑 Responses API；Chat Completions 的请求、响应和流 chunk 类型也由固定规范生成。Chat 资源支持创建 JSON/SSE、检索、更新 metadata、删除、列表和消息分页；SSE chunk 保留原始 JSON，并在 `[DONE]` 正常结束。

Embeddings API 通过 `client.embeddings().create(&request)` 访问。生成类型支持字符串或批量文本输入、float 或 base64 embedding 响应。

Moderations API 通过 `client.moderations().create(&request)` 访问，支持文本、多段文本及生成类型中的多模态输入结构。

Audio API 提供文本转语音、音频转录/翻译及自定义语音同意记录管理。语音返回逐块读取的 `AudioContentStream`；设置 SSE 模式时返回保留未知事件的 `AudioEventStream`。转录和翻译文件上传需要 `multipart` feature。示例见 [`examples/audio.rs`](examples/audio.rs)。

Files API 支持列表、元数据检索/删除及逐块下载。使用 `multipart` feature 后，可通过 `client.files().create(&request, bytes::Bytes)` 上传可重放的内存内容，或用 `create_stream` 从一次性 byte stream 上传；一次性流不会自动重试。文件响应中的 OpenAI deprecated `status` 字段为 `Option`，以兼容 DeepSeek 官方 OpenAI-compatible Files 响应中的省略行为。`multipart` feature 默认关闭。下载返回 `FileContentStream`，逐次调用 `next_chunk()` 读取，不会将整个文件一次性载入内存。

大型文件可使用 Uploads API 的 `create`、`add_part`、`complete` 和 `cancel` 操作。上传 part 使用 `multipart` feature；`add_part` 接受可重放的 `bytes::Bytes`，`add_part_stream` 接受 `Result<Bytes, std::io::Error>` byte stream 且不会自动重试。完成请求中的 part ID 顺序决定最终文件内容顺序。示例见 [`examples/uploads.rs`](examples/uploads.rs)。

Content Provenance 检查通过 `client.content_provenance().check(&request, bytes)` 上传媒体文件；此操作需要 `multipart` feature。Images API 支持 JSON 图像生成/编辑、生成与编辑 SSE（完成事件后结束并保留未知事件），以及 `multipart` feature 下的 `edit_files` 图片文件编辑。文件编辑按重复 `image[]` part 上传，可选 `mask` 单独上传。示例见 [`examples/images.rs`](examples/images.rs)。

Conversations API 提供资源创建、检索、metadata 更新和删除，以及 conversation item 的增删查与分页列表。`list_items` 返回单页，`list_items_pages` 则逐页返回完整响应和 request ID，并检查空或重复 cursor。

Containers API 支持容器及其文件的创建、列表、读取、删除和内容下载。`list_pages` 与 `list_files_pages` 会逐页保留完整响应和 request ID，并检查空或重复 cursor。`upload_file` 使用可重放内存字节且需要 `multipart` feature；`download_file` 返回逐块读取的 `FileContentStream`。

Batches API 支持创建、列表、读取与取消批处理任务；`list_pages` 逐页读取并保留每页响应与 request ID。

Vector Stores API 支持向量库及文件/文件批次 CRUD、检索和搜索；列表、解析文件内容与搜索结果提供逐页流接口。查看 [`examples/vector_stores.rs`](examples/vector_stores.rs)。OpenAI 当前 API 参考中的部分 Vector Stores 示例仍包含 `OpenAI-Beta: assistants=v2`；如目标服务要求该 header，可通过 `RequestOptions` 添加。该兼容行为未在本地或真实服务商环境验证。

Fine-tuning API 支持列举、创建、读取、取消、暂停和恢复任务，查看任务事件与 checkpoint，并管理 checkpoint 权限；列表操作提供逐页流。Alpha Graders 未纳入。查看 [`examples/fine_tuning.rs`](examples/fine_tuning.rs)。

Live API 提供 WebRTC/SIP 会话创建、接受、分叉、挂断、转接、拒接及录音分块下载。启用 `websocket` feature 后可连接 Live 双向 WebSocket，按需收发 JSON 事件或二进制媒体帧；未知 JSON 事件保留原始内容。Live WebSocket 示例演示生成类型的 `session.start`、等待 `session.started`，再发送 `session.close` 并等待 `session.closed`，见 [`examples/live_websocket.rs`](examples/live_websocket.rs)。

Realtime API 提供 GA 会话/转录/翻译 client secret、WebRTC SDP call answer（含可选 multipart session configuration），以及 SIP 来电接受、拒绝、转接和挂断操作；`websocket` feature 也提供 GA Realtime 双向连接。Realtime WebSocket 示例演示 `session.update`、文本对话输入与 `response.done` 终止事件，见 [`examples/realtime_websocket.rs`](examples/realtime_websocket.rs)。

Responses WebSocket 同样需要 `websocket` feature。`client.responses().connect_websocket()` 返回共享连接，`create_response(Some(stream_id), &request)` 按 lane FIFO 排队并返回有界缓冲事件流；单连接最多使用 32 个命名 `stream_id`，服务端最多并行处理 16 个响应并排队其余请求。事件按 lane 路由，并在 completed/incomplete/failed 或服务端 error 事件后结束。启用 `RetryPolicy` 时仅会在握手成功前重试；连接建立后不会自动重连或重放请求。查看 [`examples/websockets.rs`](examples/websockets.rs)。

Webhooks API 支持 endpoint 创建、列表、读取、更新、删除、签名密钥轮换、测试投递和可用事件类型查询。密钥仅由创建或轮换响应类型返回。

```rust,no_run
use openai_rs::OpenAIClient;

async fn list_models(client: &OpenAIClient) -> Result<(), openai_rs::Error> {
    let models = client.models().list().await?;
    for model in &models.data.data {
        println!("{}", model.id);
    }
    Ok(())
}
```

```rust,no_run
use openai_rs::{OpenAIClient, types};

async fn chat(client: &OpenAIClient) -> Result<(), Box<dyn std::error::Error>> {
    let request: types::chat_completions::CreateChatCompletionRequest =
        serde_json::from_value(serde_json::json!({
            "model": "gpt-6-astra",
            "messages": [{"role": "user", "content": "用一句话介绍 Rust。"}]
        }))?;
    let response = client.chat_completions().create(&request).await?;
    println!("{}", response.choices[0].message.content.as_deref().unwrap_or_default());
    Ok(())
}
```

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
cargo test --features live-api,multipart --test deepseek_live
```

在线 smoke 会读取进程继承的 `DEEPSEEK_TOKEN`，向 DeepSeek Responses 与 Chat Completions JSON/SSE 端点发送短固定文本，并上传一个带 1 小时过期策略的合成像素 GIF，随后读取元数据并删除文件。DeepSeek 按实际使用返回计费，文件数据不含用户信息。不要将 token 写入文件或命令输出。

## 更新生成的协议类型

固定 OpenAI OpenAPI 提交、文件 SHA-256 和生成器版本记录在 `spec/VERSION.json`。`spec/API_SCOPE.json` 逐项记录此快照中的所有操作及纳入/排除理由；`spec/CLIENT_COVERAGE.json` 将每个纳入 operation 映射到资源 API 和公开方法。安装固定生成器后可检查范围清单、生成文件和客户端入口覆盖：

```sh
cargo install cargo-typify --version 0.8.0 --locked
scripts/generate-types.sh --check
```

更新 OpenAPI 提交后运行 `scripts/update-schema.sh` 和 `scripts/generate-types.sh --write`，并审查 schema snapshot、操作范围清单与生成 Rust 类型的变化。OpenAI 规范随仓库 MIT 许可证一并保存在 `spec/`。
