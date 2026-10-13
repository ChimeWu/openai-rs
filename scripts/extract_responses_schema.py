#!/usr/bin/env python3
"""Extract the supported Responses HTTP operations from OpenAI OpenAPI JSON."""

from __future__ import annotations

import argparse
import copy
import json
from pathlib import Path
from typing import Any


def collect_refs(value: Any, refs: set[str]) -> None:
    if isinstance(value, dict):
        ref = value.get("$ref")
        if isinstance(ref, str) and ref.startswith("#/components/schemas/"):
            refs.add(ref.rsplit("/", 1)[1].replace("~1", "/").replace("~0", "~"))
        for nested in value.values():
            collect_refs(nested, refs)
    elif isinstance(value, list):
        for nested in value:
            collect_refs(nested, refs)


def rewrite_refs(value: Any) -> Any:
    if isinstance(value, dict):
        rewritten = {}
        for key, nested in value.items():
            # Defaults are descriptive metadata, not wire constraints. The upstream
            # specification has defaults on polymorphic $ref branches that cannot
            # be represented as valid defaults for the generated concrete type.
            if key == "default":
                continue
            if key == "$ref" and isinstance(nested, str):
                prefix = "#/components/schemas/"
                if nested.startswith(prefix):
                    name = nested[len(prefix) :]
                    nested = "#/definitions/" + name
            rewritten[key] = rewrite_refs(nested)
        return rewritten
    if isinstance(value, list):
        return [rewrite_refs(nested) for nested in value]
    return value


def normalize_nullable_aliases(selected: dict[str, Any]) -> None:
    # Split named nullable aliases into a wrapper and a separately named
    # concrete definition. This avoids typify 0.8 emitting the same Rust name
    # for both an Option wrapper and its concrete branch.
    for name, schema in list(selected.items()):
        if not isinstance(schema, dict):
            continue
        keyword = "anyOf" if isinstance(schema.get("anyOf"), list) else "oneOf"
        variants = schema.get(keyword)
        if not isinstance(variants, list):
            continue
        concrete = [
            variant
            for variant in variants
            if not (isinstance(variant, dict) and variant.get("type") == "null")
        ]
        nulls = [
            variant
            for variant in variants
            if isinstance(variant, dict) and variant.get("type") == "null"
        ]
        if len(concrete) != 1 or not nulls or not isinstance(concrete[0], dict):
            continue

        inner_name = f"{name}Value"
        suffix = 2
        while inner_name in selected:
            inner_name = f"{name}Value{suffix}"
            suffix += 1
        selected[inner_name] = copy.deepcopy(concrete[0])
        wrapper = {key: value for key, value in schema.items() if key != keyword}
        wrapper[keyword] = [{"$ref": f"#/components/schemas/{inner_name}"}, *nulls]
        selected[name] = wrapper


def query_schema(
    operation: dict[str, Any], name: str, *, include: set[str] | None = None
) -> dict[str, Any]:
    properties: dict[str, Any] = {}
    for parameter in operation.get("parameters", []):
        if parameter.get("in") != "query":
            continue
        parameter_name = parameter["name"]
        if include is not None and parameter_name not in include:
            continue
        properties[parameter_name] = parameter["schema"]
    return {"title": name, "type": "object", "properties": properties}


def extract(spec: dict[str, Any]) -> dict[str, Any]:
    paths = spec["paths"]
    operations = {
        "create": paths["/responses"]["post"],
        "retrieve": paths["/responses/{response_id}"]["get"],
        "delete": paths["/responses/{response_id}"]["delete"],
        "cancel": paths["/responses/{response_id}/cancel"]["post"],
        "list_input_items": paths["/responses/{response_id}/input_items"]["get"],
        "count_input_tokens": paths["/responses/input_tokens"]["post"],
        "compact": paths["/responses/compact"]["post"],
    }

    roots: dict[str, Any] = {}
    roots["CreateResponse"] = operations["create"]["requestBody"]["content"][
        "application/json"
    ]["schema"]
    roots["Response"] = operations["create"]["responses"]["200"]["content"][
        "application/json"
    ]["schema"]
    roots["ResponseStreamEvent"] = operations["create"]["responses"]["200"][
        "content"
    ]["text/event-stream"]["schema"]

    roots["DeleteResponse"] = copy.deepcopy(
        operations["delete"]["responses"]["200"]["content"]["application/json"][
            "schema"
        ]
    )
    roots["DeleteResponse"]["title"] = "DeleteResponse"
    roots["ResponseItemList"] = operations["list_input_items"]["responses"]["200"][
        "content"
    ]["application/json"]["schema"]
    roots["TokenCountsBody"] = operations["count_input_tokens"]["requestBody"][
        "content"
    ]["application/json"]["schema"]
    roots["TokenCountsResource"] = operations["count_input_tokens"]["responses"][
        "200"
    ]["content"]["application/json"]["schema"]
    roots["CompactResponseBody"] = operations["compact"]["requestBody"]["content"][
        "application/json"
    ]["schema"]
    roots["CompactResource"] = operations["compact"]["responses"]["200"]["content"][
        "application/json"
    ]["schema"]

    roots["RetrieveResponseParams"] = query_schema(
        operations["retrieve"], "RetrieveResponseParams", include={"include"}
    )
    roots["RetrieveResponseStreamParams"] = query_schema(
        operations["retrieve"],
        "RetrieveResponseStreamParams",
        include={"include", "starting_after", "include_obfuscation"},
    )
    roots["ListInputItemsParams"] = query_schema(
        operations["list_input_items"], "ListInputItemsParams"
    )

    for operation in operations.values():
        for status, response in operation["responses"].items():
            if status.startswith("2"):
                continue
            collect_refs(response, refs := set())
            for name in refs:
                roots.setdefault(name, {"$ref": f"#/components/schemas/{name}"})

    refs: set[str] = set()
    collect_refs(roots, refs)
    schemas = spec["components"]["schemas"]
    selected: dict[str, Any] = {}
    pending = list(refs)
    while pending:
        name = pending.pop()
        if name in selected:
            continue
        if name not in schemas:
            raise ValueError(f"unresolved OpenAPI schema reference: {name}")
        schema = schemas[name]
        selected[name] = schema
        discovered: set[str] = set()
        collect_refs(schema, discovered)
        pending.extend(discovered - selected.keys())

    normalize_nullable_aliases(selected)

    # OpenAI's published list is intentionally finite, while compatible
    # providers and newly released OpenAI models can use other identifiers.
    # Keep model IDs strongly typed as strings instead of locking consumers
    # to the snapshot's current allowlist.
    if "ModelIdsResponses" in selected:
        selected["ModelIdsResponses"] = {"type": "string"}
    response = selected.get("Response")
    if isinstance(response, dict):
        for branch in response.get("allOf", []):
            required = branch.get("required", []) if isinstance(branch, dict) else []
            if "access_programs" in required:
                required.remove("access_programs")
    response_usage = selected.get("ResponseUsage")
    if isinstance(response_usage, dict):
        input_details = response_usage.get("properties", {}).get("input_tokens_details", {})
        required = input_details.get("required", []) if isinstance(input_details, dict) else []
        if "cache_write_tokens" in required:
            required.remove("cache_write_tokens")

    return {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "ResponsesApiSchema",
        "type": "object",
        "properties": {name: rewrite_refs(value) for name, value in roots.items()},
        "required": sorted(roots),
        "definitions": {
            name: rewrite_refs(value) for name, value in sorted(selected.items())
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("source", type=Path, help="pinned OpenAI OpenAPI JSON file")
    parser.add_argument("output", type=Path, help="extracted JSON Schema output")
    args = parser.parse_args()

    spec = json.loads(args.source.read_text())
    schema = extract(spec)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
    print(f"extracted {len(schema['definitions'])} schemas into {args.output}")


if __name__ == "__main__":
    main()
