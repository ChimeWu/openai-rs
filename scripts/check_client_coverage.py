#!/usr/bin/env python3
"""Verify that every included OpenAPI operation maps to a public client method."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCOPE_PATH = ROOT / "spec/API_SCOPE.json"
COVERAGE_PATH = ROOT / "spec/CLIENT_COVERAGE.json"
CLIENT_PATH = ROOT / "src/client.rs"

FAMILY_RESOURCES = {
    "Audio": "audio",
    "Batch": "batches",
    "Chat": "chat_completions",
    "Containers": "containers",
    "Content provenance": "content_provenance",
    "Conversations": "conversations",
    "Embeddings": "embeddings",
    "Files": "files",
    "Fine-tuning": "fine_tuning",
    "Images": "images",
    "Live": "live",
    "Models": "models",
    "Moderations": "moderations",
    "Realtime": "realtime",
    "Responses": "responses",
    "Uploads": "uploads",
    "Vector stores": "vector_stores",
    "Webhooks": "webhooks",
}

RESOURCE_IMPLS = {
    "audio": "AudioApi",
    "batches": "BatchesApi",
    "chat_completions": "ChatCompletionsApi",
    "containers": "ContainersApi",
    "content_provenance": "ContentProvenanceApi",
    "conversations": "ConversationsApi",
    "embeddings": "EmbeddingsApi",
    "files": "FilesApi",
    "fine_tuning": "FineTuningApi",
    "images": "ImagesApi",
    "live": "LiveApi",
    "models": "ModelsApi",
    "moderations": "ModerationsApi",
    "realtime": "RealtimeApi",
    "responses": "ResponsesApi",
    "uploads": "UploadsApi",
    "vector_stores": "VectorStoresApi",
    "webhooks": "WebhooksApi",
}


def read_coverage() -> tuple[dict, dict[str, set[str]]]:
    scope = json.loads(SCOPE_PATH.read_text())
    coverage = json.loads(COVERAGE_PATH.read_text())
    if coverage.get("openapi_commit") != scope.get("openapi_commit"):
        raise ValueError("CLIENT_COVERAGE.json uses a different OpenAPI commit")

    included = {
        item["operation_id"]: item
        for item in scope["operations"]
        if item["decision"] == "include"
    }
    entries = coverage.get("operations", [])
    covered = {item.get("operation_id") for item in entries}
    if len(covered) != len(entries):
        raise ValueError("CLIENT_COVERAGE.json contains duplicate operation IDs")
    if covered != set(included):
        missing = sorted(set(included) - covered)
        stale = sorted(covered - set(included))
        raise ValueError(f"coverage mismatch; missing={missing}, stale={stale}")

    source = CLIENT_PATH.read_text()
    implementations: dict[str, set[str]] = {}
    pattern = re.compile(r"^impl\s+(\w+Api)\b[^\n{]*\{\n(.*?)^\}", re.M | re.S)
    for match in pattern.finditer(source):
        implementations[match.group(1)] = set(
            re.findall(r"^\s*pub\s+async\s+fn\s+(\w+)\s*\(", match.group(2), re.M)
        )

    method_pattern = re.compile(r"^(\w+)\.(\w+)$")
    for entry in entries:
        operation_id = entry["operation_id"]
        operation = included[operation_id]
        if entry.get("method") != operation["method"] or entry.get("path") != operation["path"]:
            raise ValueError(f"stale route mapping for {operation_id}")
        family = operation["family"]
        expected_resource = FAMILY_RESOURCES.get(family)
        match = method_pattern.fullmatch(entry.get("client_method", ""))
        if not expected_resource or not match:
            raise ValueError(f"invalid client method mapping for {operation_id}")
        resource, method = match.groups()
        if resource != expected_resource:
            raise ValueError(
                f"{operation_id} belongs to {family}, but maps to client.{resource}()"
            )
        implementation = RESOURCE_IMPLS[resource]
        if method not in implementations.get(implementation, set()):
            raise ValueError(
                f"{operation_id} ({entry['method']} {entry['path']}) maps to missing "
                f"client.{resource}().{method}()"
            )
    return scope, implementations


def main() -> None:
    try:
        scope, _ = read_coverage()
    except (KeyError, TypeError, ValueError) as error:
        print(f"client coverage check failed: {error}", file=sys.stderr)
        raise SystemExit(1) from error
    print(
        "client coverage is current "
        f"({scope['included_count']} included operations mapped to public methods)"
    )


if __name__ == "__main__":
    main()
