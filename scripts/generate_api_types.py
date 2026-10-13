#!/usr/bin/env python3
"""Generate protocol types in separate modules for all included API families."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCOPE = ROOT / "spec/API_SCOPE.json"
EXTRACTOR = ROOT / "scripts/extract_api_family_schema.py"
GENERATED = ROOT / "src/generated"
COMPATIBILITY_OVERRIDES = ROOT / "spec/COMPATIBILITY_OVERRIDES.json"

MODULE_NAMES = {
    "Batch": "batches",
    "Chat": "chat_completions",
    "Content provenance": "content_provenance",
    "Fine-tuning": "fine_tuning",
    "Vector stores": "vector_stores",
}
ALLOW_ATTRIBUTES = [
    "#[allow(clippy::redundant_closure_call)]",
    "#[allow(clippy::needless_lifetimes)]",
    "#[allow(clippy::match_single_binding)]",
    "#[allow(clippy::clone_on_copy)]",
]


def module_name(family: str) -> str:
    if family in MODULE_NAMES:
        return MODULE_NAMES[family]
    return family.lower().replace(" ", "_").replace("-", "_")


def rust_source(path: Path) -> bytes:
    lines = path.read_text().splitlines()
    while lines and (lines[0].startswith("#![allow(") or not lines[0].strip()):
        lines.pop(0)
    return ("\n".join(lines) + "\n").encode()


def apply_compatibility_overrides(family: str, schema_path: Path) -> None:
    overrides = json.loads(COMPATIBILITY_OVERRIDES.read_text()).get(family, {})
    if not overrides:
        return
    schema = json.loads(schema_path.read_text())
    for name, override in overrides.items():
        definition = schema.get("definitions", {}).get(name)
        if not isinstance(definition, dict):
            raise SystemExit(f"compatibility override references missing {family}.{name}")
        properties = definition.get("properties", {})
        required = definition.get("required", [])
        for prop in override.get("optional_properties", []):
            if prop not in properties or prop not in required:
                raise SystemExit(
                    f"compatibility override expected required property {family}.{name}.{prop}"
                )
            required.remove(prop)
    schema_path.write_text(json.dumps(schema, ensure_ascii=False, indent=2) + "\n")


def generate(mode: str) -> None:
    scope = json.loads(SCOPE.read_text())
    families = sorted(
        {
            item["family"]
            for item in scope["operations"]
            if item["decision"] == "include" and item["family"] != "Responses"
        }
    )
    modules = [(family, module_name(family)) for family in families]
    if len({module for _, module in modules}) != len(modules):
        raise SystemExit("API family module name collision")

    with tempfile.TemporaryDirectory(prefix="openai-rs-api-types-") as tmp_name:
        tmp = Path(tmp_name)
        generated_files: dict[str, bytes] = {}
        for family, module in modules:
            family_schema = tmp / f"{module}.schema.json"
            raw_rust = tmp / f"{module}.raw.rs"
            subprocess.run(
                [
                    sys.executable,
                    str(EXTRACTOR),
                    family,
                    str(family_schema),
                ],
                cwd=ROOT,
                check=True,
            )
            apply_compatibility_overrides(family, family_schema)
            subprocess.run(
                [
                    "cargo",
                    "typify",
                    "--additional-derive",
                    "PartialEq",
                    "--output",
                    str(raw_rust),
                    str(family_schema),
                ],
                cwd=ROOT,
                check=True,
            )
            generated_files[f"{module}.rs"] = rust_source(raw_rust)

        api_modules = []
        type_reexports = []
        for _, module in modules:
            api_modules.extend(ALLOW_ATTRIBUTES)
            if module == "realtime":
                # Realtime event unions mirror OpenAPI discriminators and have
                # schema-sized payload variants rather than hand-sized enums.
                api_modules.append("#[allow(clippy::large_enum_variant)]")
            api_modules.extend(
                [f'pub mod {module} {{', f'    include!("{module}.rs");', "}", ""]
            )
            type_reexports.extend(
                [
                    f"pub mod {module} {{",
                    f"    pub use crate::generated::{module}::*;",
                    "}",
                    "",
                ]
            )
        generated_files["api_families.rs"] = ("\n".join(api_modules).rstrip() + "\n").encode()
        generated_files["type_families.rs"] = (
            "\n".join(type_reexports).rstrip() + "\n"
        ).encode()

        if mode == "write":
            for name, contents in generated_files.items():
                (GENERATED / name).write_bytes(contents)
            keep = set(generated_files) | {"responses.rs", "event_types.rs", "mod.rs"}
            for path in GENERATED.glob("*.rs"):
                if path.name not in keep:
                    path.unlink()
            print(f"generated protocol modules for {len(modules)} API families")
            return

        stale = [
            name
            for name, contents in generated_files.items()
            if not (GENERATED / name).exists()
            or (GENERATED / name).read_bytes() != contents
        ]
        if stale:
            raise SystemExit(
                "generated API family files are stale: "
                + ", ".join(f"src/generated/{name}" for name in stale)
                + " (run scripts/generate-types.sh --write)"
            )
        print(f"API family protocol types are current ({len(modules)} families)")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices={"check", "write"})
    args = parser.parse_args()
    generate(args.mode)


if __name__ == "__main__":
    main()
