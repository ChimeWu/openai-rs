# openai-api-types

`openai-api-types` is a standalone Serde type crate generated from the pinned
OpenAI OpenAPI snapshot in the `openai-rs` repository. It contains generated
request, path/query parameter, success response, error response, event, and
component schema types for every operation in that snapshot. Type coverage does
not imply that a client implements an operation or that a provider accepts it.

No API family is enabled by default. Select just the protocol families needed by
your application, or enable `full` to compile every family and the complete
component schema module:

```toml
[dependencies]
openai-api-types = { git = "https://github.com/ChimeWu/openai-rs", package = "openai-api-types", default-features = false, features = ["responses", "chat_completions"] }
```

```rust
use openai_api_types::{chat_completions, responses};

let _: responses::CreateResponse;
let _: chat_completions::CreateChatCompletionRequest;
```

Generated types support Serde serialization and deserialization. The crate
depends on Serde, `serde_json` for unconstrained JSON values, and `regress` for
regular-expression constraints emitted by `cargo-typify`; it has no Tokio,
Reqwest, or SDK transport dependency.

The source specification commit, hash, generator version, regeneration command,
and coverage checks are documented in [`../../docs/PROTOCOL_TYPES.md`](../../docs/PROTOCOL_TYPES.md).
