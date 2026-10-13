#!/usr/bin/env python3
"""Verify the vendored OpenAPI snapshot and extracted Responses schema."""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
VERSION = json.loads((ROOT / "spec/VERSION.json").read_text())
OPENAPI = ROOT / "spec/openai-openapi.json"
SCHEMA = ROOT / "spec/responses.schema.json"


def main() -> None:
    digest = hashlib.sha256(OPENAPI.read_bytes()).hexdigest()
    if digest != VERSION["sha256"]:
        raise SystemExit(
            f"OpenAPI snapshot hash mismatch: expected {VERSION['sha256']}, got {digest}"
        )

    with tempfile.TemporaryDirectory() as directory:
        output = Path(directory) / "responses.schema.json"
        subprocess.run(
            [
                sys.executable,
                str(ROOT / "scripts/extract_responses_schema.py"),
                str(OPENAPI),
                str(output),
            ],
            check=True,
        )
        if output.read_bytes() != SCHEMA.read_bytes():
            raise SystemExit("Responses schema snapshot is stale; run scripts/update-schema.sh")
    print(f"verified OpenAI OpenAPI commit {VERSION['commit']} and extracted Responses schema")


if __name__ == "__main__":
    main()
