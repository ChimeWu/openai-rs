#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
VERSION="$ROOT/spec/VERSION.json"
COMMIT=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["commit"])' "$VERSION")
URL="https://raw.githubusercontent.com/openai/openai-openapi/$COMMIT/openapi.json"

curl --fail --location --silent --show-error "$URL" --output "$ROOT/spec/openai-openapi.json"
python3 - "$VERSION" "$ROOT/spec/openai-openapi.json" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

version_path, spec_path = map(Path, sys.argv[1:])
version = json.loads(version_path.read_text())
version["sha256"] = hashlib.sha256(spec_path.read_bytes()).hexdigest()
version_path.write_text(json.dumps(version, indent=2) + "\n")
PY
python3 "$ROOT/scripts/extract_responses_schema.py" \
    "$ROOT/spec/openai-openapi.json" "$ROOT/spec/responses.schema.json"
python3 "$ROOT/scripts/generate_api_scope.py" \
    --spec "$ROOT/spec/openai-openapi.json" \
    --version "$ROOT/spec/VERSION.json" \
    --output "$ROOT/spec/API_SCOPE.json"
