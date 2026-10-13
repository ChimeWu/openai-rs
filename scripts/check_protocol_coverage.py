#!/usr/bin/env python3
"""Check that generated protocol modules account for the entire pinned snapshot."""

from __future__ import annotations

import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "spec/openai-openapi.json"
VERSION = json.loads((ROOT / "spec/VERSION.json").read_text())
OUT = ROOT / "crates/openai-api-types/src/generated"
MANIFEST = OUT / "coverage.json"
METHODS = {"get", "post", "put", "patch", "delete", "head", "options"}


def main() -> None:
    spec = json.loads(SPEC.read_text())
    manifest = json.loads(MANIFEST.read_text())
    expected_operations = {
        operation["operationId"]
        for path in spec["paths"].values()
        for method, operation in path.items()
        if method.lower() in METHODS and isinstance(operation, dict)
    }
    covered_operations = set(manifest.get("operations", {}))
    expected_schemas = set(spec["components"]["schemas"])
    covered_schemas = set(manifest.get("schemas", []))
    if manifest.get("openapi_commit") != VERSION["commit"]:
        raise SystemExit("protocol coverage manifest uses a different OpenAPI commit")
    if manifest.get("openapi_sha256") != VERSION["sha256"]:
        raise SystemExit("protocol coverage manifest uses a different OpenAPI hash")
    if expected_operations != covered_operations:
        raise SystemExit(
            f"operation coverage mismatch: missing={sorted(expected_operations-covered_operations)}, "
            f"extra={sorted(covered_operations-expected_operations)}"
        )
    if expected_schemas != covered_schemas:
        raise SystemExit(
            f"component schema coverage mismatch: missing={sorted(expected_schemas-covered_schemas)}, "
            f"extra={sorted(covered_schemas-expected_schemas)}"
        )
    modules = (OUT / "mod.rs").read_text()
    features = tomllib.loads((ROOT / "crates/openai-api-types/Cargo.toml").read_text())["features"]
    if features.get("default") != []:
        raise SystemExit("protocol crate default features must remain empty")
    if "common" not in features:
        raise SystemExit("shared protocol types must have a common feature")
    if not set(("components", "common", *set(manifest["operations"].values()))).issubset(
        features.get("full", [])
    ):
        raise SystemExit("full feature omits generated protocol features")
    for operation_id, module in manifest["operations"].items():
        if not (OUT / f"{module}.rs").is_file():
            raise SystemExit(f"operation {operation_id} maps to missing family module {module}")
        if not re.search(rf'#\[cfg\(feature = "{re.escape(module)}"\)\]', modules):
            raise SystemExit(f"family module {module} has no matching Cargo feature gate")
        if "common" not in features.get(module, []):
            raise SystemExit(f"family feature {module} does not enable shared schemas")
    print(f"protocol coverage is complete ({len(covered_operations)} operations, {len(covered_schemas)} component schemas)")


if __name__ == "__main__":
    try:
        main()
    except (KeyError, TypeError, ValueError) as error:
        print(f"protocol coverage check failed: {error}", file=sys.stderr)
        raise SystemExit(1) from error
