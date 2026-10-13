#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
FEATURES=$(python3 - "$ROOT/crates/openai-api-types/src/generated/coverage.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1]))
print("\n".join(sorted(set(manifest["operations"].values()))))
PY
)

cargo check --locked -p openai-api-types --no-default-features
cargo check --locked -p openai-api-types --no-default-features --features components
for feature in $FEATURES; do
    cargo check --locked -p openai-api-types --no-default-features --features "$feature"
done
cargo check --locked -p openai-api-types --no-default-features --features responses,chat_completions
cargo check --locked -p openai-api-types --no-default-features --features live,realtime
cargo check --locked -p openai-api-types --no-default-features --features full
