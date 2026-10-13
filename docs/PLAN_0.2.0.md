# openai-rs 0.2.0 开发计划

## 目标

将 SDK 从仅支持创建 Responses 扩展到固定 OpenAPI 规范中的完整非 beta HTTP Responses 操作集。0.2.0 可以调整 0.1.0 的公共 API；由于项目尚无实际用户，不保留旧扁平方法的兼容别名。

## 公共 API 与实现

- 以 `OpenAIClient::responses()` 返回的 Responses 资源接口作为唯一操作入口，形成 `client.responses().create(...)` 风格。
- 覆盖创建 JSON、创建 SSE、检索 JSON、检索 SSE、删除、取消、输入项列表、token 计数和 compact。
- 检索 SSE 支持 `starting_after` 恢复；输入项列表返回单页及 cursor，由调用方显式翻页，不实现自动分页迭代器。
- 使用通用 `ApiResponse<T>` 包装 JSON 响应，保留强类型数据、原始 JSON 和 request ID。删除响应也提供生成的强类型。
- 保留 client 默认 headers、请求级 headers 和创建请求额外顶层字段；通过一致的请求选项组织这些能力。
- 扩展 OpenAPI schema 提取及代码生成，覆盖上述操作的请求体、查询参数、响应体和复用定义。沿用当前固定的规范快照；仅在所需定义缺失时更新上游版本。

## 验收与验证

- 离线检查覆盖生成代码可复现、JSON 请求/查询编码和响应反序列化、错误上下文、cursor 参数、SSE 事件解析及恢复参数；检查依赖树不包含 Codex 内部 crate。
- 不启动本地 mock HTTP 服务。DeepSeek 在线 smoke 继续验证其支持的创建 JSON 和 SSE；DeepSeek Responses API 是无状态的，不能据此声称检索、删除等有状态操作通过在线验证。[DeepSeek Responses API](https://api-docs.deepseek.com/api/create-response/)
- 若用户明确提供 OpenAI 凭据，则用 OpenAI 真实 API 对可行的新端点执行独立 smoke；否则将这些端点的在线运行时验证标记为未验证。真实请求可能计费，不记录凭据。
- 更新 README 的新 API 示例和 0.1.0 到 0.2.0 迁移说明，升级 Cargo 包版本到 `0.2.0`。常规 CI 不要求服务商凭据。

## 范围边界与默认

- 暂不支持 Responses WebSocket、Chat Completions、Embeddings、Models、beta 路径和自动分页迭代器。
- OpenAI 固定 OpenAPI 规范作为类型和端点行为的基准；DeepSeek 只验证其文档及实际服务支持的兼容范围。[DeepSeek Responses guide](https://api-docs.deepseek.com/guides/responses_api/)
- 运行时在线覆盖与离线检查分别报告；没有真实服务商调用证据时，不把编译或离线测试表述为在线通过。

## 实施状态

计划范围已完成，验证记录如下：

- 公共 API 已迁移至 `client.responses()`；新增 JSON/SSE 检索、删除、取消、输入项单页列表、token 计数和 compact。响应统一返回 `ApiResponse<T>`，查询及请求/响应类型从固定 OpenAPI schema 生成。
- 已更新 `Cargo.toml` 与 `Cargo.lock` 至 `0.2.0`，并将 README 示例迁移到新 API，补充检索、分页和迁移说明。
- `cargo test --offline --locked`：18 项单测通过；`cargo check --all-targets --all-features --offline --locked`、`cargo fmt --all -- --check`、固定 `cargo-typify 0.8.0` 的生成一致性检查均通过。独立临时消费者编译了 README 用法和全部新增端点调用；依赖树无 Codex 内部 crate。
- 使用真实 DeepSeek `deepseek-flash` 成功完成创建 JSON 和 SSE smoke；未启动本地 mock 服务。DeepSeek 无状态，因此检索、删除、取消、输入项列表、token 计数和 compact 尚无真实服务运行时验证。没有用户明确提供 OpenAI 凭据，故这些端点标记为未验证。
