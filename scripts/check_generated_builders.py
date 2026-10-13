#!/usr/bin/env python3
"""Verify every generated protocol object struct exposes a builder."""

from __future__ import annotations

import re
import sys
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
GENERATED_DIRS = [ROOT / "crates/openai-api-types/src/generated"]
OBJECT_STRUCT = re.compile(r"^pub struct ([A-Za-z0-9_]+)\s*\{", re.MULTILINE)
TYPE_DECL = re.compile(r"^pub (?:struct|enum|union) ([A-Za-z0-9_]+)", re.MULTILINE)


def main() -> None:
    total = 0
    missing: list[str] = []
    for directory in GENERATED_DIRS:
        for path in sorted(directory.glob("*.rs")):
            source = path.read_text()
            duplicates = sorted(
                name for name, count in Counter(TYPE_DECL.findall(source)).items() if count > 1
            )
            if duplicates:
                missing.extend(f"{path.relative_to(ROOT)}:duplicate type {name}" for name in duplicates)
            for name in OBJECT_STRUCT.findall(source):
                total += 1
                builder_impl = re.compile(
                    rf"^impl {re.escape(name)}\s*\{{\s*pub fn builder\s*\(",
                    re.MULTILINE,
                )
                if not builder_impl.search(source):
                    missing.append(f"{path.relative_to(ROOT)}:{name}")

    if missing:
        print("generated protocol structs without builders:", file=sys.stderr)
        print("\n".join(missing), file=sys.stderr)
        raise SystemExit(1)
    print(f"all generated protocol object structs expose builders ({total} structs)")


if __name__ == "__main__":
    main()
