#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
MODE=${1:---check}
if [ "$MODE" != "--check" ] && [ "$MODE" != "--write" ]; then
    echo "usage: scripts/generate-types.sh [--check|--write]" >&2
    exit 2
fi

python3 "$ROOT/scripts/check_schema.py"
GENERATOR_VERSION=$(cargo typify --version)
case "$GENERATOR_VERSION" in
    *"0.8.0"*) ;;
    *) echo "expected cargo-typify 0.8.0, found: $GENERATOR_VERSION" >&2; exit 1 ;;
esac

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT HUP INT TERM
cargo typify --no-builder --additional-derive PartialEq \
    --output "$TMP/responses.rs" "$ROOT/spec/responses.schema.json"
python3 - "$TMP/responses.rs" <<'PY'
import sys
from pathlib import Path

path = Path(sys.argv[1])
lines = path.read_text().splitlines()
while lines and (lines[0].startswith("#![allow(") or not lines[0].strip()):
    lines.pop(0)
path.write_text("\n".join(lines) + "\n")
PY
python3 "$ROOT/scripts/generate_known_events.py" \
    "$ROOT/spec/responses.schema.json" "$TMP/event_types.rs"

if [ "$MODE" = "--write" ]; then
    cp "$TMP/responses.rs" "$ROOT/src/generated/responses.rs"
    cp "$TMP/event_types.rs" "$ROOT/src/generated/event_types.rs"
else
    for generated in responses.rs event_types.rs; do
        if ! cmp -s "$TMP/$generated" "$ROOT/src/generated/$generated"; then
            echo "generated file is stale: src/generated/$generated (run scripts/generate-types.sh --write)" >&2
            exit 1
        fi
    done
    echo "generated Rust types are up to date"
fi
