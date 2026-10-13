# 从 Codex 抽取独立的 Rust OpenAI SDK

## 项目目标

在当前仓库构建一个可通过 Git 依赖使用的独立 Rust crate。以本地 Codex 源码 `/home/chimewu/Projects/codex` 中的 Responses API 实现为行为参考，以 OpenAI 官方 OpenAPI 规范为协议类型来源。首版提供 Tokio 异步 HTTP 与 SSE 流式 Responses API，不依赖任何 Codex 内部 crate 或类型。

## 首版范围

- 单 crate，包名 `openai-rs`，Rust 导入名 `openai_rs`。
- 只支持 Responses API 的 HTTP 请求与 SSE 流式响应。
- 使用 API key 认证；支持默认 OpenAI API 地址、可配置的兼容服务地址及请求级额外 headers。
- 暂不包含 Responses WebSocket、Chat Completions、Embeddings、Models 和凭据刷新。
- 交付目标为可独立构建、可通过 Git 依赖消费；首版暂不要求 crates.io 发布。

## 实施方案

### 协议类型

- 采用 OpenAI 官方发布的 OpenAPI 3.1 规范，覆盖 Responses 请求、响应、流事件和相关错误类型。
- 固定生成所用的规范版本；生成 Rust 类型并提交到仓库，消费者构建不需要联网下载规范或运行生成器。
- 提供可复现的生成与更新脚本，并在 CI 中检查生成代码与固定规范的一致性。
- 生成类型提供 JSON 扩展字段入口；未知流事件保留原始类型和 payload，尽量兼容服务端新增字段。
- 代码生成只负责协议类型；客户端、认证、SSE 解码和易用的构建接口由手写代码维护。

### 客户端与流处理

- 实现 `/v1/responses` 请求、API key 认证、可配置服务地址、请求级扩展 headers、HTTP 错误解析及 SSE 事件流。
- 使用 Tokio、Serde/Serde JSON 和异步 HTTP 客户端；由调用方提供 Tokio runtime。
- 开发期间通过真实 DeepSeek API 验证端到端请求与流式响应：从进程环境读取 `DEEPSEEK_TOKEN`，使用 OpenAI 兼容地址 `https://api.deepseek.com` 和 `deepseek-flash` 模型。DeepSeek 官方文档说明其 Responses API 使用 OpenAI Responses 格式；请求字段和事件覆盖仍以官方兼容说明及实际返回为准。
- 从 Codex 借鉴通用 Responses/SSE 行为；移除 Codex 专用会话头、遥测、网络策略及其他内部集成逻辑。
- 对借鉴或改编的 Codex 源码核对许可证及归属要求，不引入 `codex-api`、`codex-client`、`codex-protocol` 等内部依赖。

## 验收标准

- 不使用本地 mock 服务作为 SDK 验证手段。
- 使用 DeepSeek 在线 smoke 验证真实 HTTP 请求、认证、Responses 响应反序列化和 SSE 流式事件；只发送无敏感信息的短请求，并记录模型、请求场景、结果和是否产生费用，不记录 token。覆盖 `response.completed`、`response.incomplete`、`response.failed` 等终止事件；按 DeepSeek 文档处理其 Responses SSE 结束语义，不假设存在 `[DONE]`。
- 离线验证请求序列化、生成类型解析、未知字段/事件处理及 SSE 分块边界；覆盖正常结束、畸形事件等可由纯单元测试验证的行为，不启动本地 mock HTTP 服务。
- 独立消费者通过 Git 依赖构建并运行文档示例；依赖树不包含 Codex 内部 crate。
- 从固定规范可重复生成类型；规范更新产生可审查的代码差异。
- 将离线检查与 DeepSeek 在线验证分别报告。在线验证需要可用的 `DEEPSEEK_TOKEN`；凭据不可用时明确记录未验证，不以 mock 服务替代。

## 官方规范来源

- [OpenAI OpenAPI specification repository](https://github.com/openai/openai-openapi)
- [OpenAI OpenAPI YAML](https://raw.githubusercontent.com/openai/openai-openapi/main/openapi.yaml)
- [Responses API reference](https://developers.openai.com/api/reference/resources/responses)
- [DeepSeek Responses API](https://api-docs.deepseek.com/api/create-response/)
- [DeepSeek OpenAI-compatible API quick start](https://api-docs.deepseek.com/guides/codex)
- [DeepSeek Responses API streaming](https://api-docs.deepseek.com/guides/responses_api/)

## 状态

当前处于规划阶段，尚未开始 SDK 实现、代码生成器选型或验证。
