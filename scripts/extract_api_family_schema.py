#!/usr/bin/env python3
"""Extract operation request, parameter, and response schemas for one API family."""

from __future__ import annotations

import argparse
import copy
import json
import re
from pathlib import Path
from typing import Any


HTTP_METHODS = {"GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"}


def pascal_case(value: str) -> str:
    parts = re.findall(r"[A-Za-z0-9]+", value)
    if not parts:
        raise ValueError(f"cannot make a type name from operation ID {value!r}")
    return "".join(part[:1].upper() + part[1:] for part in parts)


def resolve_component(spec: dict[str, Any], value: Any, section: str) -> Any:
    while isinstance(value, dict) and "$ref" in value:
        ref = value["$ref"]
        prefix = f"#/components/{section}/"
        if not isinstance(ref, str) or not ref.startswith(prefix):
            return value
        name = ref[len(prefix) :].replace("~1", "/").replace("~0", "~")
        value = spec["components"][section][name]
    return value


def collect_refs(value: Any, refs: set[str]) -> None:
    if isinstance(value, dict):
        ref = value.get("$ref")
        prefix = "#/components/schemas/"
        if isinstance(ref, str) and ref.startswith(prefix):
            refs.add(ref[len(prefix) :].replace("~1", "/").replace("~0", "~"))
        for nested in value.values():
            collect_refs(nested, refs)
    elif isinstance(value, list):
        for nested in value:
            collect_refs(nested, refs)


def rewrite_refs(value: Any) -> Any:
    if isinstance(value, dict):
        properties = value.get("properties")
        exclusive_required = value.get("anyOf")
        if (
            isinstance(properties, dict)
            and isinstance(exclusive_required, list)
            and len(exclusive_required) > 1
            and all(
                isinstance(item, dict)
                and set(item) == {"required"}
                and isinstance(item["required"], list)
                and len(item["required"]) == 1
                and item["required"][0] in properties
                for item in exclusive_required
            )
            and isinstance(value.get("not"), dict)
        ):
            # OpenAPI uses anyOf/not to require exactly one of several
            # optional object properties. Serde does not validate that rule;
            # keep the useful object shape and leave this constraint to the
            # server instead of generating an uninhabitable empty enum.
            value = {
                key: nested
                for key, nested in value.items()
                if key not in {"anyOf", "not"}
            }
        # OpenAPI occasionally describes a free-form scalar as a union with
        # an enum subset (for example, any model id or one of three known
        # embedding model ids). The unrestricted scalar already includes that
        # subset, and typify 0.8.0 cannot deserialize this union correctly.
        for union_key in ("anyOf", "oneOf"):
            variants = value.get(union_key)
            if not isinstance(variants, list) or len(variants) < 2:
                continue
            # Flatten nested scalar unions when one branch is an open string
            # reference and the sibling branches only enumerate known values.
            # This preserves the full string wire shape and avoids typify
            # generating an untagged struct for unions such as TTS Voice.
            normalized_variants = []
            changed_nested_scalar = False
            for variant in variants:
                nested = variant.get(union_key) if isinstance(variant, dict) else None
                if (
                    isinstance(nested, list)
                    and len(nested) >= 2
                    and all(isinstance(item, dict) for item in nested)
                    and any(isinstance(item.get("$ref"), str) for item in nested)
                    and all(
                        isinstance(item.get("$ref"), str)
                        or item.get("type") == "string"
                        for item in nested
                    )
                ):
                    open_reference = next(
                        (item for item in nested if isinstance(item.get("$ref"), str)),
                        None,
                    )
                    constrained = [item for item in nested if item is not open_reference]
                    if open_reference is not None and all(
                        item.get("type") == "string" and "enum" in item
                        for item in constrained
                    ):
                        normalized_variants.append(open_reference)
                        changed_nested_scalar = True
                        continue
                normalized_variants.append(variant)
            if changed_nested_scalar:
                value = {key: nested for key, nested in value.items() if key != union_key}
                value[union_key] = normalized_variants
                variants = normalized_variants
            primitive_types = {
                item.get("type")
                for item in variants
                if isinstance(item, dict)
            }
            unrestricted = [
                item
                for item in variants
                if isinstance(item, dict)
                and set(item) <= {"type", "title"}
                and isinstance(item.get("type"), str)
            ]
            if (
                len(primitive_types) == 1
                and len(unrestricted) == 1
                and all(isinstance(item, dict) for item in variants)
            ):
                collapsed = {
                    key: nested for key, nested in value.items() if key != union_key
                }
                collapsed["type"] = unrestricted[0]["type"]
                return rewrite_refs(collapsed)
        result = {}
        for key, nested in value.items():
            if key == "default":
                continue
            # Keep date-shaped API fields as wire strings rather than coupling
            # the public SDK to chrono just for an OpenAPI format annotation.
            if key == "format" and nested == "date":
                continue
            if key == "minLength" and nested == 0:
                continue
            # These branches constrain schema validation conditionally. Typify
            # 0.8.0 does not support them; serde models represent the union of
            # the declared fields, while request validation remains server-side.
            if key in {"if", "then", "else"}:
                continue
            if key == "$ref" and isinstance(nested, str):
                prefix = "#/components/schemas/"
                if nested.startswith(prefix):
                    nested = "#/definitions/" + nested[len(prefix) :]
            result[key] = rewrite_refs(nested)
        return result
    if isinstance(value, list):
        return [rewrite_refs(item) for item in value]
    return value


def normalize_nullable_aliases(selected: dict[str, Any]) -> None:
    for name, schema in list(selected.items()):
        if not isinstance(schema, dict):
            continue
        keyword = "anyOf" if isinstance(schema.get("anyOf"), list) else "oneOf"
        variants = schema.get(keyword)
        if not isinstance(variants, list):
            continue
        concrete = [
            item
            for item in variants
            if not (isinstance(item, dict) and item.get("type") == "null")
        ]
        nulls = [
            item
            for item in variants
            if isinstance(item, dict) and item.get("type") == "null"
        ]
        if len(concrete) != 1 or not nulls or not isinstance(concrete[0], dict):
            continue
        inner_name = f"{name}Value"
        counter = 2
        while inner_name in selected:
            inner_name = f"{name}Value{counter}"
            counter += 1
        selected[inner_name] = copy.deepcopy(concrete[0])
        wrapper = {key: value for key, value in schema.items() if key != keyword}
        wrapper[keyword] = [{"$ref": f"#/components/schemas/{inner_name}"}, *nulls]
        selected[name] = wrapper


def parameter_schema(spec: dict[str, Any], parameter: dict[str, Any]) -> dict[str, Any]:
    parameter = resolve_component(spec, parameter, "parameters")
    schema = parameter.get("schema")
    if isinstance(schema, dict):
        return schema
    content = parameter.get("content", {})
    for value in content.values():
        if isinstance(value.get("schema"), dict):
            return value["schema"]
    return {"type": "string"}


def extract(spec: dict[str, Any], scope: dict[str, Any], family: str) -> dict[str, Any]:
    operations = [
        item
        for item in scope["operations"]
        if item["family"] == family and item["decision"] == "include"
    ]
    if not operations:
        raise ValueError(f"API_SCOPE.json has no included operations for family {family!r}")

    roots: dict[str, Any] = {}
    inline_definitions: dict[str, Any] = {}
    paths = spec["paths"]
    for item in operations:
        path_item = paths[item["path"]]
        operation = path_item[item["method"].lower()]
        if operation.get("operationId") != item["operation_id"]:
            raise ValueError(f"stale API_SCOPE.json entry: {item['operation_id']}")
        operation_id = operation["operationId"]
        prefix = pascal_case(operation_id)

        parameter_map: dict[tuple[str, str], dict[str, Any]] = {}
        for parameter in [*path_item.get("parameters", []), *operation.get("parameters", [])]:
            parameter = resolve_component(spec, parameter, "parameters")
            parameter_map[(parameter["in"], parameter["name"])] = parameter
        serializable_parameters = [
            parameter
            for parameter in parameter_map.values()
            if parameter.get("in") == "query"
        ]
        if serializable_parameters:
            properties = {
                parameter["name"]: parameter_schema(spec, parameter)
                for parameter in serializable_parameters
            }
            required = [
                parameter["name"]
                for parameter in serializable_parameters
                if parameter.get("required", False)
            ]
            roots[f"{prefix}Params"] = {
                "title": f"{prefix}Params",
                "type": "object",
                "properties": properties,
                "required": required,
            }

        request_body = operation.get("requestBody")
        if request_body is not None:
            request_body = resolve_component(spec, request_body, "requestBodies")
            content = request_body.get("content", {})
            media_order = (
                "application/json",
                "multipart/form-data",
                "application/x-www-form-urlencoded",
                "application/octet-stream",
            )
            request_body_added = False
            for media_type in media_order:
                media = content.get(media_type)
                if isinstance(media, dict) and isinstance(media.get("schema"), dict):
                    schema = copy.deepcopy(media["schema"])
                    suffix = (
                        "Request"
                        if not request_body_added
                        else f"{pascal_case(media_type)}Request"
                    )
                    if "$ref" not in schema:
                        definition_name = f"{prefix}{suffix}"
                        schema["title"] = definition_name
                        inline_definitions[definition_name] = schema
                        schema = {"$ref": f"#/components/schemas/{definition_name}"}
                    # Keep every supported body shape available to generated
                    # consumers. JSON remains the canonical `Request` root;
                    # alternate encodings receive an explicit media suffix.
                    root_name = f"{prefix}{suffix}"
                    roots[root_name] = schema
                    request_body_added = True

        response_map: dict[str, Any] = {}
        for status, raw_response in operation.get("responses", {}).items():
            if not status.startswith("2"):
                continue
            response = resolve_component(spec, raw_response, "responses")
            for media_type, media in response.get("content", {}).items():
                if not isinstance(media, dict) or not isinstance(media.get("schema"), dict):
                    continue
                if media_type == "application/json":
                    suffix = "Response"
                elif media_type == "text/event-stream":
                    suffix = "StreamResponse"
                else:
                    continue
                root_name = f"{prefix}{suffix}"
                if root_name in response_map and response_map[root_name] != media["schema"]:
                    root_name = f"{prefix}{suffix}{status}"
                schema = copy.deepcopy(media["schema"])
                if "$ref" not in schema:
                    schema.setdefault("title", root_name)
                response_map[root_name] = schema
        roots.update(response_map)

    # WebSocket protocol events are not HTTP operation request/response
    # schemas, but they are published in the same pinned OpenAPI document.
    websocket_roots = {
        "Realtime": (
            "RealtimeClientEvent",
            "RealtimeServerEvent",
            "RealtimeTranslationClientEvent",
            "RealtimeTranslationServerEvent",
        ),
        "Live": (
            "LiveClientEvent",
            "LiveServerEvent",
            "LiveForkClientEvent",
            "LiveForkServerEvent",
            "LiveSidebandClientEvent",
            "LiveSidebandServerEvent",
        ),
    }
    for name in websocket_roots.get(family, ()):
        if name in spec["components"]["schemas"]:
            roots[name] = {"$ref": f"#/components/schemas/{name}"}

    refs: set[str] = set()
    collect_refs(roots, refs)
    schemas = spec["components"]["schemas"]
    selected: dict[str, Any] = copy.deepcopy(inline_definitions)
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
    for name in list(selected):
        if name.startswith("ModelIds"):
            selected[name] = {"type": "string"}
    return {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": f"{pascal_case(family)}ApiSchema",
        "type": "object",
        "properties": {name: rewrite_refs(schema) for name, schema in roots.items()},
        "required": sorted(roots),
        "definitions": {
            name: rewrite_refs(schema) for name, schema in sorted(selected.items())
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("family")
    parser.add_argument("output", type=Path)
    parser.add_argument("--spec", type=Path, default=Path("spec/openai-openapi.json"))
    parser.add_argument("--scope", type=Path, default=Path("spec/API_SCOPE.json"))
    args = parser.parse_args()
    spec = json.loads(args.spec.read_text())
    scope = json.loads(args.scope.read_text())
    schema = extract(spec, scope, args.family)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
    print(f"extracted {len(schema['definitions'])} schemas for {args.family}")


if __name__ == "__main__":
    main()
