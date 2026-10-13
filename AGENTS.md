# 仓库协作指南

## 项目目标

- 开始实现或调整项目范围前，先阅读 [`docs/PROJECT_PLAN.md`](docs/PROJECT_PLAN.md)。
- 构建干净、可独立使用的 Rust OpenAI SDK。`/home/chimewu/Projects/codex` 中的 Codex 源码仅作为行为参考，不作为依赖。
- 不添加 Codex 工作区 crate 依赖；公共 API 不暴露 Codex 专用类型、请求头、遥测或策略行为。
- 协议类型从 OpenAI 发布的 OpenAPI 规范生成。确保生成过程可复现，并将规范更新作为源代码变更审查。

## 验证方式

- 不使用本地 mock HTTP 服务验证 SDK 行为。网络和服务商集成通过真实 DeepSeek OpenAI 兼容 API 验证。
- 凭据只从进程继承的 `DEEPSEEK_TOKEN` 环境变量读取。不得打印、记录、检查、复制到文件、提交凭据，也不得将其值写入命令输出或测试诊断信息。不要搜索文件系统以查找凭据。
- 在线 Responses API smoke 验证使用 DeepSeek OpenAI 兼容地址 `https://api.deepseek.com` 和 `deepseek-flash` 模型。如果测试依赖服务商当前支持情况或模型细节，先查阅 [DeepSeek 官方文档](https://api-docs.deepseek.com/api/create-response/)。
- DeepSeek Responses 流通过语义终止事件结束，例如 `response.completed`、`response.incomplete` 或 `response.failed`；不要假设它会发送 Chat Completions 使用的 `data: [DONE]` 标记。参见 [DeepSeek Responses API 流式文档](https://api-docs.deepseek.com/guides/responses_api/)。
- 在线 API 检查与离线检查分开执行。在线检查应显式选择，只发送不含敏感用户数据的短请求，并报告覆盖场景、结果及已知兼容性限制。不得静默跳过用户要求的在线检查，也不得用 mock 替代；凭据不可用时，将在线检查标记为未验证。
- 离线测试可以覆盖纯序列化、反序列化、基于字节样例的 SSE 解析、分块边界、畸形事件和错误映射。不要启动本地 HTTP mock 服务。
- 在实现记录和完成报告中明确区分真实服务商调用与本地检查。不能仅凭编译或离线测试就声称在线集成通过。

## 变更与报告

- 保留无关的用户改动。编辑前检查 `git status`；只有用户要求暂存时，才暂存请求范围内的文件。
- 确保文档和示例与实际公共 API 及已验证行为一致。
- 分别报告实现情况、离线验证、真实服务商验证和环境限制。
