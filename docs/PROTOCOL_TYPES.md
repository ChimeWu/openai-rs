# OpenAPI protocol types

The workspace package `openai-api-types` (`openai_api_types`) provides generated
protocol types independently of the `openai-rs` client. Its family modules and
Cargo features cover every HTTP operation in the pinned OpenAI OpenAPI snapshot,
including operations that `API_SCOPE.json` excludes from SDK runtime support.
Schemas reused by several families are generated once in `common`; each family
feature enables `common` and re-exports the shared types it uses. The
`components` feature exposes every schema in the pinned document, including
schemas not referenced by an operation. `full` enables all family features and
`components`. The default feature set is empty.

The root SDK enables the family features corresponding to its existing client
API and re-exports them from `openai_rs::types`, so both paths refer to the same
Rust types. Enable the SDK's `full-protocol-types` feature to expose every
protocol family and component schema from that path. This does not expand the
SDK's HTTP, SSE, or WebSocket implementation coverage.

## Reproducing and checking generation

The source is `spec/openai-openapi.json`, pinned to commit
`77618e641a840768f1e58197ca53ddedffea6df9` with its SHA-256 in
`spec/VERSION.json`. Generation uses `cargo-typify 0.8.0`:

```sh
scripts/generate-types.sh --write
scripts/generate-types.sh --check
```

The generated `coverage.json` maps all 360 HTTP operation IDs to API family
modules and lists all 2,049 component schemas. The coverage checker compares
those sets directly with the pinned document; it does not consult client
inclusion decisions. Generation fails on unresolved schema references and
module-name collisions. The generated files are committed so consumers do not
need the generator or network access.

The generator applies the documented provider-compatibility optional-field
overrides in `spec/COMPATIBILITY_OVERRIDES.json` uniformly to family modules and
the `components` module. These adjust only the requiredness of `Model.created`
and `OpenAIFile.status`; the source OpenAPI snapshot remains unchanged.

CI checks the default empty feature set, every individual family feature,
cross-family combinations, and `full`. It also checks the SDK re-export and
offline Serde examples. These checks establish schema coverage and Rust type
compatibility; they do not establish runtime interoperability with OpenAI or any
OpenAI-compatible provider.
