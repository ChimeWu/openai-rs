#!/usr/bin/env python3
"""Generate an explicit include/exclude inventory for the pinned OpenAPI operations."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
from typing import Any

HTTP_METHODS = {"get", "post", "put", "patch", "delete", "head", "options"}


def family_for(path: str, tags: list[str]) -> str:
    if tags:
        return tags[0]
    if path.startswith("/containers"):
        return "Containers"
    if path.startswith("/chatkit"):
        return "ChatKit"
    if path.startswith("/webhook_"):
        return "Webhooks"
    if path.startswith("/content_provenance"):
        return "Content provenance"
    if path.startswith("/safety/"):
        return "Safety"
    return "Uncategorized"


def classify(path: str, operation: dict[str, Any], family: str) -> tuple[str, str]:
    operation_id = str(operation.get("operationId", ""))
    lowered = f"{path} {operation_id} {family}".lower()

    if "?beta=" in path or operation_id.lower().startswith("beta_"):
        return "exclude", "Beta operation; this SDK includes stable public APIs only."
    if path.startswith("/chatkit/") or family.lower() == "chatkit":
        return "exclude", "ChatKit currently requires the OpenAI-Beta chatkit_beta=v1 header."
    if path.startswith("/decisions/") or family.lower() == "decisions":
        return "exclude", "Decisions is listed under Beta APIs in the official API reference."
    if "/alpha/" in path or path.startswith("/fine_tuning/alpha/"):
        return "exclude", "Alpha API operation; this SDK includes stable public APIs only."
    if path.startswith(("/organization/", "/projects/")):
        return "exclude", "Organization or project administration API; requires administrative scope."
    if path.startswith("/safety/") or "safetycase" in operation_id.lower():
        return "exclude", "Safety administration API; outside ordinary project API-key scope."
    if family.lower() in {"assistants"} or path.startswith("/threads"):
        return "exclude", "Assistants/Threads API was sunset on 2026-08-26."
    if family.lower() == "evals" or path.startswith("/evals"):
        return "exclude", "Evals API is scheduled to shut down on 2026-11-30."
    if family.lower() == "videos" or path.startswith("/videos"):
        return "exclude", "Videos API shut down on 2026-09-24."
    if path == "/completions" or family.lower() == "completions":
        return "exclude", "Legacy Completions API; excluded in favor of Chat Completions and Responses."
    if family.lower() == "agents" or "/agents" in lowered:
        return "exclude", "Agents platform API; outside this SDK's ordinary public model API scope."
    if family.lower() in {"skills", "vaults"}:
        return "exclude", "Agent workspace or credential-management API; outside the public model API scope."
    if operation.get("deprecated"):
        return "exclude", "Deprecated by the pinned OpenAPI snapshot."

    return "include", f"Stable public {family} API operation for ordinary project API keys."


def generate(spec: dict[str, Any], version: dict[str, Any], source_sha256: str) -> dict[str, Any]:
    operations = []
    seen: set[tuple[str, str]] = set()
    for path, path_item in spec["paths"].items():
        for method, operation in path_item.items():
            if method.lower() not in HTTP_METHODS or not isinstance(operation, dict):
                continue
            operation_id = operation.get("operationId")
            if not isinstance(operation_id, str) or not operation_id:
                raise ValueError(f"operation missing operationId: {method.upper()} {path}")
            key = (method.upper(), path)
            if key in seen:
                raise ValueError(f"duplicate HTTP operation: {method.upper()} {path}")
            seen.add(key)
            tags = operation.get("tags", [])
            family = family_for(path, tags)
            decision, reason = classify(path, operation, family)
            operations.append(
                {
                    "operation_id": operation_id,
                    "method": method.upper(),
                    "path": path,
                    "family": family,
                    "decision": decision,
                    "reason": reason,
                }
            )
    operations.sort(key=lambda item: (item["family"].casefold(), item["path"], item["method"], item["operation_id"]))
    return {
        "openapi_commit": version["commit"],
        "openapi_file_sha256": source_sha256,
        "operation_count": len(operations),
        "included_count": sum(item["decision"] == "include" for item in operations),
        "excluded_count": sum(item["decision"] == "exclude" for item in operations),
        "operations": operations,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="fail if the checked-in manifest is stale")
    parser.add_argument("--spec", type=Path, default=Path("spec/openai-openapi.json"))
    parser.add_argument("--version", type=Path, default=Path("spec/VERSION.json"))
    parser.add_argument("--output", type=Path, default=Path("spec/API_SCOPE.json"))
    args = parser.parse_args()

    version = json.loads(args.version.read_text())
    spec_sha256 = hashlib.sha256(args.spec.read_bytes()).hexdigest()
    if spec_sha256 != version["sha256"]:
        raise SystemExit(
            f"OpenAPI snapshot hash mismatch: expected {version['sha256']}, got {spec_sha256}"
        )
    manifest = generate(
        json.loads(args.spec.read_text()),
        version,
        spec_sha256,
    )
    rendered = json.dumps(manifest, indent=2, ensure_ascii=False) + "\n"
    if args.check:
        if not args.output.exists() or args.output.read_text() != rendered:
            raise SystemExit(f"API scope manifest is stale: {args.output}")
        print(f"API scope manifest is current ({manifest['included_count']} included, {manifest['excluded_count']} excluded)")
    else:
        args.output.write_text(rendered)
        print(f"wrote {manifest['operation_count']} operations to {args.output}")


if __name__ == "__main__":
    main()
