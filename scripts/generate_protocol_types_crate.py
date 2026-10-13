#!/usr/bin/env python3
"""Generate the standalone, complete protocol-types crate from the pinned OpenAPI snapshot."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "spec/openai-openapi.json"
SCOPE = ROOT / "spec/API_SCOPE.json"
VERSION = json.loads((ROOT / "spec/VERSION.json").read_text())
OVERRIDES = json.loads((ROOT / "spec/COMPATIBILITY_OVERRIDES.json").read_text())
EXTRACTOR = ROOT / "scripts/extract_api_family_schema.py"
OUT = ROOT / "crates/openai-api-types/src/generated"
ALLOW = [
    "#[allow(clippy::redundant_closure_call)]",
    "#[allow(clippy::needless_lifetimes)]",
    "#[allow(clippy::match_single_binding)]",
    "#[allow(clippy::clone_on_copy)]",
]


def module_name(family: str) -> str:
    aliases = {
        "Batch": "batches",
        "Chat": "chat_completions",
        "Content provenance": "content_provenance",
        "Fine-tuning": "fine_tuning",
        "Vector stores": "vector_stores",
    }
    if family in aliases:
        return aliases[family]
    value = re.sub(r"[^a-z0-9]+", "_", family.lower()).strip("_")
    return value or "uncategorized"


def type_name(schema_name: str) -> str:
    words = re.findall(
        r"[A-Z]+(?=[A-Z][a-z]|$)|[A-Z]?[a-z]+|[0-9]+", schema_name
    )
    if not words:
        raise SystemExit(f"cannot make a Rust type name from schema {schema_name!r}")
    return "".join(word[:1].upper() + word[1:].lower() for word in words)


def map_shared_refs(value: object, names: set[str]) -> object:
    if isinstance(value, dict):
        ref = value.get("$ref")
        prefix = "#/definitions/"
        if isinstance(ref, str) and ref.startswith(prefix):
            name = ref[len(prefix):].replace("~1", "/").replace("~0", "~")
            if name in names:
                return {
                    "x-rust-type": {
                        "crate": "openai-api-types",
                        "version": "0.1.0",
                        "path": f"openai_api_types::common::{type_name(name)}",
                    }
                }
        return {key: map_shared_refs(nested, names) for key, nested in value.items()}
    if isinstance(value, list):
        return [map_shared_refs(item, names) for item in value]
    return value


def rust_source(path: Path) -> bytes:
    lines = path.read_text().splitlines()
    while lines and (lines[0].startswith("#![allow(") or not lines[0].strip()):
        lines.pop(0)
    return ("\n".join(lines) + "\n").encode()


def apply_overrides(family: str, schema_path: Path) -> None:
    overrides = OVERRIDES.get(family, {})
    if not overrides:
        return
    schema = json.loads(schema_path.read_text())
    for name, override in overrides.items():
        definition = schema.get("definitions", {}).get(name)
        if not isinstance(definition, dict):
            continue
        required = definition.get("required", [])
        for prop in override.get("optional_properties", []):
            if prop in required:
                required.remove(prop)
    schema_path.write_text(json.dumps(schema, ensure_ascii=False, indent=2) + "\n")


def components_schema(spec: dict) -> dict:
    definitions = spec["components"]["schemas"]
    return {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "OpenAIComponents",
        "type": "object",
        "properties": {},
        "required": [],
        "definitions": definitions,
    }


def generate(mode: str) -> None:
    from extract_api_family_schema import collect_refs, normalize_nullable_aliases, rewrite_refs

    raw_spec = SPEC.read_bytes()
    digest = hashlib.sha256(raw_spec).hexdigest()
    if digest != VERSION["sha256"]:
        raise SystemExit(f"OpenAPI snapshot hash mismatch: {digest}")
    spec = json.loads(raw_spec)
    scope = json.loads(SCOPE.read_text())
    operations = scope["operations"]
    families = sorted({item["family"] for item in operations})
    modules = [(family, module_name(family)) for family in families]
    names = [module for _, module in modules]
    if len(names) != len(set(names)):
        collisions = sorted(name for name in set(names) if names.count(name) > 1)
        raise SystemExit(f"API family module name collision: {', '.join(collisions)}")

    generated: dict[str, bytes] = {}
    manifest = {"openapi_commit": VERSION["commit"], "openapi_sha256": digest,
                "operations": {}, "schemas": sorted(spec["components"]["schemas"])}
    with tempfile.TemporaryDirectory(prefix="openai-api-types-") as temp_name:
        temp = Path(temp_name)
        family_schemas: dict[str, dict] = {}
        for family, module in modules:
            schema = temp / f"{module}.schema.json"
            subprocess.run(
                [sys.executable, str(EXTRACTOR), family, str(schema), "--all-operations"],
                cwd=ROOT, check=True,
            )
            apply_overrides(family, schema)
            family_schemas[family] = json.loads(schema.read_text())
            if family == "Responses":
                # Keep the SDK's established Responses type names and shapes
                # while adding the broader OpenAPI operation surface.
                legacy = json.loads((ROOT / "spec/responses.schema.json").read_text())
                family_schemas[family]["properties"].update(legacy["properties"])
                family_schemas[family]["definitions"].update(legacy["definitions"])
            for operation in operations:
                if operation["family"] == family:
                    manifest["operations"][operation["operation_id"]] = module

        component_families: dict[str, set[str]] = {}
        for family, family_schema in family_schemas.items():
            for name in family_schema["definitions"]:
                if name in spec["components"]["schemas"]:
                    component_families.setdefault(name, set()).add(family)
        shared_names = {name for name, owners in component_families.items() if len(owners) > 1}
        shared_closure = set(shared_names)
        pending = list(shared_names)
        while pending:
            name = pending.pop()
            refs: set[str] = set()
            collect_refs(spec["components"]["schemas"][name], refs)
            for ref in refs - shared_closure:
                if ref not in spec["components"]["schemas"]:
                    raise SystemExit(f"unresolved shared schema reference: {ref}")
                shared_closure.add(ref)
                pending.append(ref)
        manifest["shared_schemas"] = sorted(shared_names)

        common_definitions = {
            name: spec["components"]["schemas"][name]
            for name in sorted(shared_closure)
        }
        normalize_nullable_aliases(common_definitions)
        for family, overrides in OVERRIDES.items():
            for name, override in overrides.items():
                definition = common_definitions.get(name)
                if isinstance(definition, dict):
                    for prop in override.get("optional_properties", []):
                        if prop in definition.get("required", []):
                            definition["required"].remove(prop)
        for name in list(common_definitions):
            if name.startswith("ModelIds"):
                common_definitions[name] = {"type": "string"}
        common_schema = temp / "common.schema.json"
        common_rust = temp / "common.raw.rs"
        common_schema.write_text(json.dumps({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "OpenAISharedComponents",
            "type": "object",
            "properties": {},
            "required": [],
            "definitions": {
                name: rewrite_refs(value)
                for name, value in common_definitions.items()
            },
        }, ensure_ascii=False, indent=2) + "\n")
        subprocess.run(
            ["cargo", "typify", "--additional-derive", "PartialEq", "--crate",
             "openai-api-types@0.1.0", "--unknown-crates", "allow", "--output",
             str(common_rust), str(common_schema)], cwd=ROOT, check=True,
        )
        generated["common.rs"] = rust_source(common_rust)

        for family, module in modules:
            family_schema = family_schemas[family]
            family_shared = sorted(
                shared_names & set(family_schema["definitions"])
            )
            family_schema["definitions"] = {
                name: value for name, value in family_schema["definitions"].items()
                if name not in shared_names
            }
            family_schema = map_shared_refs(family_schema, shared_names)
            schema = temp / f"{module}.schema.json"
            schema.write_text(json.dumps(family_schema, ensure_ascii=False, indent=2) + "\n")
            raw_rust = temp / f"{module}.raw.rs"
            subprocess.run(
                ["cargo", "typify", "--additional-derive", "PartialEq", "--crate",
                 "openai-api-types@0.1.0", "--unknown-crates", "allow", "--output",
                 str(raw_rust), str(schema)], cwd=ROOT, check=True,
            )
            contents = rust_source(raw_rust)
            if family_shared:
                exports = ", ".join(type_name(name) for name in family_shared)
                contents = f"pub use crate::common::{{{exports}}};\n".encode() + contents
            if module == "audio":
                contents = (
                    b"pub use crate::common::VoiceIdsOrCustomVoice as Voice;\n"
                    + contents
                )
            generated[f"{module}.rs"] = contents

        component_schema = temp / "components.schema.json"
        component_rust = temp / "components.raw.rs"
        component_schema.write_text(json.dumps(components_schema(spec), indent=2) + "\n")
        # The component schema uses the same OpenAPI-to-JSON-Schema reference
        # normalization as operation families.
        doc = json.loads(component_schema.read_text())
        normalize_nullable_aliases(doc["definitions"])
        for name in list(doc["definitions"]):
            if name.startswith("ModelIds"):
                doc["definitions"][name] = {"type": "string"}
        doc["definitions"] = {k: rewrite_refs(v) for k, v in doc["definitions"].items()}
        for family in OVERRIDES:
            for name, override in OVERRIDES[family].items():
                definition = doc["definitions"].get(name)
                if isinstance(definition, dict):
                    for prop in override.get("optional_properties", []):
                        if prop in definition.get("required", []):
                            definition["required"].remove(prop)
        component_schema.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
        subprocess.run(
            ["cargo", "typify", "--additional-derive", "PartialEq", "--output",
             str(component_rust), str(component_schema)], cwd=ROOT, check=True,
        )
        generated["components.rs"] = rust_source(component_rust)

    mod_lines = ["// Generated by scripts/generate_protocol_types_crate.py."]
    for _, module in modules:
        mod_lines.extend([*ALLOW, f'#[cfg(feature = "{module}")]',
                          f'pub mod {module} {{', f'    include!("{module}.rs");', "}"])
    mod_lines.extend([*ALLOW, '#[cfg(feature = "components")]',
                      'pub mod components {', '    include!("components.rs");', '}'])
    mod_lines.extend([*ALLOW, '#[cfg(feature = "common")]',
                      'pub mod common {', '    include!("common.rs");', '}'])
    generated["mod.rs"] = ("\n".join(mod_lines) + "\n").encode()
    generated["coverage.json"] = (json.dumps(manifest, ensure_ascii=False, indent=2) + "\n").encode()
    feature_lines = ["[features]", "default = []", "components = []", "common = []"]
    for _, module in modules:
        feature_lines.append(f'{module} = ["common"]')
    feature_lines.append(
        'full = [' + ', '.join(json.dumps(name) for name in ["components", "common", *names]) + ']'
    )
    cargo_path = ROOT / "crates/openai-api-types/Cargo.toml"
    cargo_contents = cargo_path.read_text()
    cargo_contents = re.sub(r"(?ms)^\[features\]\n.*?(?=^\[|\Z)", "\n".join(feature_lines) + "\n\n", cargo_contents)
    generated["Cargo.toml"] = cargo_contents.encode()

    if mode == "write":
        OUT.mkdir(parents=True, exist_ok=True)
        for name, contents in generated.items():
            destination = cargo_path if name == "Cargo.toml" else OUT / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(contents)
        expected = set(generated)
        for stale in OUT.glob("*.rs"):
            if stale.name not in expected:
                stale.unlink()
        print(f"generated {len(operations)} operations, {len(manifest['schemas'])} component schemas, {len(modules)} API families")
        return
    stale = [name for name, contents in generated.items()
             if not ((cargo_path if name == "Cargo.toml" else OUT / name).is_file())
             or (cargo_path if name == "Cargo.toml" else OUT / name).read_bytes() != contents]
    extras = sorted(path.name for path in OUT.glob("*.rs") if path.name not in generated)
    if stale or extras:
        raise SystemExit(f"protocol crate output is stale; missing/changed={stale}, extra={extras}; run scripts/generate-types.sh --write")
    print(f"protocol crate generation is current ({len(operations)} operations, {len(manifest['schemas'])} component schemas)")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=("check", "write"))
    args = parser.parse_args()
    generate(args.mode)


if __name__ == "__main__":
    main()
