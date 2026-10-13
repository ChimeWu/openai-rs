#!/usr/bin/env python3
"""Verify every generated protocol object struct exposes a builder."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
GENERATED = ROOT / "src/generated"
OBJECT_STRUCT = re.compile(r"^pub struct ([A-Za-z0-9_]+)\s*\{", re.MULTILINE)


def main() -> None:
    total = 0
    missing: list[str] = []
    for path in sorted(GENERATED.glob("*.rs")):
        source = path.read_text()
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
