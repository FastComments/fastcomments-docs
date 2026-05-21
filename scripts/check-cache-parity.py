#!/usr/bin/env python3
"""
Cross-check Rust vs Node cache-key derivation for the OpenAI client.

Mirrors the Node implementation in `src/sdk-doc-generators/openai-client.js`
(stableStringify + SHA256). Generates a fixed corpus of payloads, computes
the Node-side digest in Python (the algorithm is short enough to inline),
and emits a JSON file that the Rust integration test can read to assert
equality.

This is the gate that lets us trust the existing `sdk-ai-cache/` directory
will continue to hit after the migration.

Usage:
  python3 scripts/check-cache-parity.py        # prints the corpus + hashes
  python3 scripts/check-cache-parity.py --write # also writes
    rust/llm/tests/cache_parity_fixtures.json
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent


def stable_stringify(obj):
    """Replicates the Node `stableStringify` at openai-client.js:13-34."""
    return json.dumps(obj, separators=(",", ":"), sort_keys=True, ensure_ascii=False)


def sha256_hex(obj):
    return hashlib.sha256(stable_stringify(obj).encode("utf-8")).hexdigest()


CORPUS = [
    {
        "name": "ts_get_comment",
        "payload": {
            "methodName": "getComment",
            "parameters": [{"name": "id", "type": "string"}],
            "responseType": "Comment",
            "nestedTypes": None,
            "httpMethod": "GET",
            "path": "/api/v1/comments/:id",
            "prompt": "Generate a TypeScript example for getComment.",
            "model": "gpt-5-mini",
        },
    },
    {
        "name": "rust_post_comment",
        "payload": {
            "methodName": "postComment",
            "parameters": [
                {"name": "body", "type": "String"},
                {"name": "url_id", "type": "String"},
            ],
            "responseType": "Comment",
            "nestedTypes": {"User": {"id": "String"}},
            "httpMethod": "POST",
            "path": "/api/v1/comments",
            "prompt": "Generate a Rust example for postComment.",
            "model": "gpt-5-mini",
        },
    },
    {
        "name": "empty_nested",
        "payload": {
            "methodName": "noop",
            "parameters": [],
            "responseType": None,
            "nestedTypes": None,
            "httpMethod": "GET",
            "path": "/",
            "prompt": "",
            "model": "gpt-4.1",
        },
    },
]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    args = ap.parse_args()

    output = []
    for item in CORPUS:
        h = sha256_hex(item["payload"])
        output.append({"name": item["name"], "payload": item["payload"], "expected_sha256": h})
        print(f"{item['name']}: {h}")

    if args.write:
        out_path = REPO / "rust/llm/tests/cache_parity_fixtures.json"
        out_path.write_text(json.dumps(output, indent=2) + "\n")
        print(f"\nWrote {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
