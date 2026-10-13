# 独立 OpenAI OpenAPI 协议类型 crate 计划

## 目标

在当前仓库 Cargo workspace 中新增独立协议类型 crate，覆盖固定 OpenAI OpenAPI 快照描述的全部接口类型。协议层不根据用户用途、API 生命周期或客户端实现情况筛选接口；HTTP、SSE、WebSocket、认证和重试等运行时行为仍由客户端 crate 负责。

当前 `API_SCOPE.json` 的纳入/排除规则继续用于描述客户端运行时范围，不再限制协议类型生成。规范快照中的 Admin、Beta、Legacy、Agent 及其他类别均可在协议类型 crate 中表达，但类型存在不代表客户端支持或接口当前可调用。

## 包与公共接口

- 新增 workspace package `crates/openai-api-types`，Rust crate 名为 `openai_api_types`。
- 按 OpenAPI API 家族生成独立模块，并为每个家族提供 Cargo feature；共享 schema 由生成器识别并建立所需 feature 依赖。
- 默认不启用 API 家族 feature，提供 `full` feature 一次启用全部生成类型。
- 协议 crate 只承载协议数据类型及其序列化/反序列化能力，不依赖 Tokio、Reqwest 或 SDK 的传输实现。
- `openai-rs` 依赖新 crate，并通过 `openai_rs::types` 重导出协议类型，保持现有类型路径和类型身份。当前 SDK 客户端方法保持现有可用范围。

## 生成与范围管理

- 类型生成直接遍历固定 OpenAPI 快照中的全部 HTTP operations，生成路径/查询参数、请求体、响应、错误及其引用的 schema；同时覆盖规范中独立定义的事件类型和组件 schema。
- 按 API 家族组织生成模块，公共命名遵循稳定、可重复的规范化规则；生成时检查类型名冲突和未解析引用并失败退出。
- 保留固定的 OpenAPI commit、文件哈希及 `cargo-typify` 版本。生成差异必须由规范快照和生成脚本可复现，并纳入代码审查。
- `API_SCOPE.json` 与 `CLIENT_COVERAGE.json` 继续校验客户端实现范围。另由协议覆盖检查器验证快照中的全部操作、参数、请求/响应结构和 schema 引用进入生成结果，不把客户端清单当作类型生成过滤器。

## 验收

- 从固定快照重复生成的 Rust 文件完全一致；所有 OpenAPI operation 和组件 schema 均能映射到生成模块，无悬空引用、遗漏操作或命名冲突。
- 分别编译默认空 feature、各家族 feature、跨家族 feature 组合及 `full`；独立下游 crate 能通过 `default-features = false` 只启用所需类型。
- workspace 默认及 all-features 检查通过；现有 `openai_rs::types` 用法继续编译，且重导出类型与新 crate 类型是同一类型。
- 对代表性的请求、响应、错误和流事件执行离线 Serde 序列化/反序列化检查；不以类型覆盖宣称接口运行时互操作已验证。
- 更新 README、CI 和生成说明，区分“规范类型已覆盖”“客户端已有运行时支持”与“真实服务商互操作已验证”。

## 默认决策与边界

- “全部接口”以仓库固定 OpenAPI 快照为准，包括其中的 Admin、Beta、Legacy 和 Agent 等操作；快照外的接口需先更新并审查规范快照。
- Feature 以 API 家族为主要粒度，不为每个 operation 单独建立开关；默认 feature 集合为空，`full` 启用全部类型。
- 类型 crate 与客户端暂留在同一仓库 workspace；本计划不改变已有 SDK 客户端方法的运行时能力或承诺。
