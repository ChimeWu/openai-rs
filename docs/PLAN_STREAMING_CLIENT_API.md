# 统一流式客户端 API 设计

## 目标

统一 SDK 中 SSE、字节流和 WebSocket 接收端的 Rust 消费方式，同时保留各 API 家族自己的协议语义和事件类型。流式 API 应支持自然的异步迭代、明确的终止与错误语义、可取消的资源生命周期，以及常见文本场景的便捷消费。

本计划定义公共 API 和验收标准，不表示实现已经完成，也不表示对应服务商互操作已经验证。

## 公共 API

### 统一接收接口

- 保留资源方法命名：SSE 使用 `create_stream` 或 `generate_stream`，WebSocket 使用 `connect_websocket`。
- 所有接收流均实现 `futures_core::Stream<Item = Result<T, Error>>`。消费者通过 `futures_util::StreamExt::next` 读取，不再需要专用的 `next_event`、`next_chunk` 或 `next_message` 方法。
- 覆盖 Responses 与 Chat Completions SSE、Audio 与 Images SSE、Audio 与 File 字节流，以及 Responses、Realtime 与 Live WebSocket 接收端。
- 各 API 家族保留自己的事件类型；不建立把不相关协议事件合并在一起的全局枚举。

### 文本便捷接口

- Responses 与 Chat Completions 流提供 `text_deltas()` 适配器，用于只消费增量文本。
- 提供消费整个流的 `collect_text()` 便捷方法。
- Responses 聚合结果包含按事件顺序拼接的文本和最终 `Response`。
- Chat Completions 聚合结果按 choice 保存文本，并保留每个 choice 的 finish reason 及最终 usage（如果服务端提供）。

### WebSocket 收发

- Realtime 与 Live 的 `WebSocketConnection::split(self)` 返回 `WebSocketSender` 和 `WebSocketReceiver`，允许发送与接收并发进行。
- Sender 提供 JSON 消息、二进制消息发送和显式关闭操作；receiver 实现标准 `Stream`，并使用各 API 家族的类型化事件。
- 未知的 JSON 事件保留原始 JSON，二进制帧继续以 `Bytes` 表示。
- Responses WebSocket 保留专用 `ResponsesWebSocket` API。其 `create_response(stream_id, request)` 返回对应的 `ResponseWebSocketStream`，继续按 `stream_id` 路由响应事件。

## 生命周期与错误语义

- 收到协议正常终止事件后，流返回 `None`。
- 传输、解析、空闲超时或协议错误以一个 `Err` 返回；之后流结束，不重复报告同一错误。
- Responses 的失败终止事件转换为流错误。
- 丢弃 SSE 或字节流会取消底层读取。丢弃某个 Responses WebSocket 响应流会停止向该 lane 分发；关闭共享连接后，其余 lane 以连接已关闭错误结束。
- 丢弃通用 WebSocket receiver 会关闭底层连接，之后 sender 的发送操作返回连接已关闭错误。显式关闭则执行正常关闭流程。
- 保留有界队列，避免慢消费者导致无界内存增长。连接建立阶段可以按既有策略重试；连接建立后不自动重连或重放已经开始的流。

## 验收标准

- 离线检查确认所有流式接收面都实现 `Stream`，正常终止返回 `None`，错误只返回一次后结束；覆盖空闲超时、丢弃和取消行为。
- WebSocket 检查覆盖 split 后并发收发、显式关闭与 drop、已知和未知事件、二进制帧、Responses `stream_id` 路由及有界背压。
- 文本聚合检查确认 Responses 保留最终 `Response`；Chat 按 choice 聚合文本，并保留 finish reason 和 usage。
- 使用离线字节样例、分块边界和 channel 检查协议解析及生命周期；不启动本地 HTTP 或 WebSocket mock 服务。
- 更新 README、示例和迁移说明，使其对应新的公共 API 和已验证行为。
- 直接替换旧的专用读取方法，作为 `openai-rs` `0.3.0` 的破坏性 API 变更发布。

## 设计边界

- 类型化事件流是主要接口，文本适配器用于常见的文本生成场景。
- 各 API 家族保持独立事件类型，并采用统一的 `Stream` 与错误生命周期约定。
- 本轮为 Realtime 与 Live 提供并发收发、类型化事件和安全关闭能力；不在 SDK 中实现更高层的实时会话状态机。

## 实施记录

- 2026-10-14：SDK 版本升级为 `0.3.0`。Responses、Chat Completions、Audio、Images、文件/容器/Live 字节下载、Responses WebSocket lane、Realtime WebSocket 和 Live WebSocket 接收端均通过 `Stream` 消费；旧的 `next_event`、`next_chunk`、`next_message` 方法已移除。Realtime 与 Live 返回按 API 家族类型化的事件，保留未知 JSON 和二进制帧；WebSocket split sender/receiver 可并发使用，并在 drop 时触发连接关闭。
- 文本 API 已实现：Responses 聚合文本与最终 `Response`；Chat 按 choice 聚合文本、finish reason 和可选 usage。README、全部流式示例和 [`MIGRATION_0.3.0.md`](MIGRATION_0.3.0.md) 已迁移。
- 离线验证通过：`cargo check --all-targets --all-features --locked`；`cargo test --offline --locked --features websocket,multipart,full-protocol-types`（80 项单测、2 项协议集成测试）；`cargo fmt --all -- --check`、`scripts/check_client_coverage.py`（111 个纳入 operation）和 `git diff --check`。没有启动本地 HTTP 或 WebSocket mock 服务。
- 未验证真实服务商互操作。一次 `cargo test --offline --all-features --locked` 意外启用了 `live-api` DeepSeek 测试；DNS 解析失败，四项在线测试均未建立连接或发送请求。之后选择不含 `live-api` 的离线测试集完成本地验证。版本号已更新；尚未创建提交或发布 tag。
