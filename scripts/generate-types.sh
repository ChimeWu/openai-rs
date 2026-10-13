#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
MODE=${1:---check}
if [ "$MODE" != "--check" ] && [ "$MODE" != "--write" ]; then
    echo "usage: scripts/generate-types.sh [--check|--write]" >&2
    exit 2
fi

python3 "$ROOT/scripts/check_schema.py"
python3 "$ROOT/scripts/generate_api_scope.py" --check
GENERATOR_VERSION=$(cargo typify --version)
case "$GENERATOR_VERSION" in
    *"0.8.0"*) ;;
    *) echo "expected cargo-typify 0.8.0, found: $GENERATOR_VERSION" >&2; exit 1 ;;
esac

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT HUP INT TERM
python3 "$ROOT/scripts/generate_known_events.py" \
    "$ROOT/spec/responses.schema.json" "$TMP/event_types.rs"

if [ "$MODE" = "--write" ]; then
    cp "$TMP/event_types.rs" "$ROOT/src/generated/event_types.rs"
else
    if ! cmp -s "$TMP/event_types.rs" "$ROOT/src/generated/event_types.rs"; then
        echo "generated file is stale: src/generated/event_types.rs (run scripts/generate-types.sh --write)" >&2
        exit 1
    fi
fi
if [ "$MODE" = "--write" ]; then
    python3 "$ROOT/scripts/generate_protocol_types_crate.py" write
else
    python3 "$ROOT/scripts/generate_protocol_types_crate.py" check
    python3 "$ROOT/scripts/check_protocol_coverage.py"
    python3 "$ROOT/scripts/check_client_coverage.py"
fi
python3 "$ROOT/scripts/check_generated_builders.py"
echo "generated Rust protocol types are up to date"
