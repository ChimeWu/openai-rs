#!/usr/bin/env python3
"""Generate the set used to recognize OpenAI Responses SSE event types."""

from __future__ import annotations

import json
import sys
from pathlib import Path


def main() -> None:
    schema = json.loads(Path(sys.argv[1]).read_text())
    definitions = schema["definitions"]
    event_union = definitions["ResponseStreamEvent"]
    event_types: set[str] = set()
    variants = event_union.get("oneOf", event_union.get("anyOf", []))
    for variant in variants:
        ref = variant["$ref"]
        name = ref.rsplit("/", 1)[1]
        event_schema = definitions[name]
        event_types.update(event_schema["properties"]["type"]["enum"])

    output = "// Generated from spec/responses.schema.json by scripts/generate_known_events.py.\n"
    output += "pub(super) const KNOWN_RESPONSE_EVENT_TYPES: &[&str] = &[\n"
    output += "".join(f"    {json.dumps(event_type)},\n" for event_type in sorted(event_types))
    output += "];\n"
    Path(sys.argv[2]).write_text(output)
    print(f"generated {len(event_types)} known Responses event names")


if __name__ == "__main__":
    main()
