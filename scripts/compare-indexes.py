#!/usr/bin/env python3
"""
Diff search-index content between the legacy SQLite FTS5 dbs (db/) and the
new Tantivy indexes (index/).

For each locale where BOTH formats are present, reports:

  * doc_ids missing from one side (additive drift, e.g. new guides added
    after the SQLite db was last built, shows up as `+` rows only).
  * For shared doc_ids, normalized-text mismatches in title or search_text.

Reads SQLite directly via the stdlib sqlite3 module. Reads Tantivy via a
companion Rust example binary (`dump_docs`) — build it once with:

  (cd rust && cargo build --release --example dump_docs)

Usage:
  python3 scripts/compare-indexes.py [--locale en[,fr_fr...]]
                                     [--sample N]
                                     [--max-text-diffs N]

By default, processes every locale where both indexes exist and prints a
summary at the end.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sqlite3
import sys
from collections import Counter
from pathlib import Path
from typing import Iterable

REPO = Path(__file__).resolve().parent.parent
SQLITE_DIR = REPO / "db"
TANTIVY_DIR = REPO / "index"
DUMP_DOCS_BIN = Path.home() / ".cache/cargo-target/release/examples/dump_docs"
DUMP_DOCS_LOCAL = REPO / "rust/target/release/examples/dump_docs"


def find_dump_docs() -> Path:
    for candidate in (DUMP_DOCS_BIN, DUMP_DOCS_LOCAL):
        if candidate.exists():
            return candidate
    sys.exit(
        "could not find compiled dump_docs binary. Build it first:\n"
        "  (cd rust && cargo build --release --example dump_docs)"
    )


def sqlite_locales() -> set[str]:
    return {p.stem.removeprefix("search-") for p in SQLITE_DIR.glob("search-*.db")}


def tantivy_locales() -> set[str]:
    return {p.name for p in TANTIVY_DIR.iterdir() if (p / "meta.json").exists()}


WHITESPACE = re.compile(r"\s+")


def normalize(s: str) -> str:
    """Whitespace-collapse for fair text comparison across renderers."""
    return WHITESPACE.sub(" ", s).strip()


def load_sqlite(locale: str) -> dict[str, dict]:
    db_path = SQLITE_DIR / f"search-{locale}.db"
    out: dict[str, dict] = {}
    conn = sqlite3.connect(f"file:{db_path}?mode=ro", uri=True)
    try:
        cur = conn.execute(
            "SELECT doc_id, title, parent_title, url, parent_url, icon, search_text "
            "FROM search_index"
        )
        for doc_id, title, parent_title, url, parent_url, icon, search_text in cur:
            out[doc_id] = {
                "title": title or "",
                "parent_title": parent_title,
                "url": url or "",
                "parent_url": parent_url,
                "icon": icon,
                "search_text": search_text or "",
            }
    finally:
        conn.close()
    return out


def load_tantivy(dump_bin: Path, locale: str) -> dict[str, dict]:
    out: dict[str, dict] = {}
    proc = subprocess.run(
        [str(dump_bin), str(TANTIVY_DIR / locale)],
        check=True,
        capture_output=True,
    )
    for raw in proc.stdout.splitlines():
        if not raw:
            continue
        row = json.loads(raw)
        out[row["doc_id"]] = row
    return out


def diff_locale(
    locale: str,
    sqlite_docs: dict[str, dict],
    tantivy_docs: dict[str, dict],
    sample: int,
    max_text_diffs: int,
) -> dict:
    sq_ids = set(sqlite_docs.keys())
    tv_ids = set(tantivy_docs.keys())
    missing_in_tantivy = sorted(sq_ids - tv_ids)
    extra_in_tantivy = sorted(tv_ids - sq_ids)
    shared = sorted(sq_ids & tv_ids)

    title_mismatches: list[tuple[str, str, str]] = []
    text_mismatches: list[tuple[str, int, int, str]] = []
    text_match_count = 0

    check_ids = shared if sample <= 0 or sample >= len(shared) else shared[:sample]

    for doc_id in check_ids:
        sq = sqlite_docs[doc_id]
        tv = tantivy_docs[doc_id]

        sq_title = normalize(sq["title"])
        tv_title = normalize(tv["title"])
        if sq_title != tv_title:
            title_mismatches.append((doc_id, sq_title, tv_title))

        sq_text = normalize(sq["search_text"])
        tv_text = normalize(tv["search_text"])
        if sq_text == tv_text:
            text_match_count += 1
        else:
            if len(text_mismatches) < max_text_diffs:
                sample_diff = _short_diff(sq_text, tv_text)
                text_mismatches.append((doc_id, len(sq_text), len(tv_text), sample_diff))

    return {
        "locale": locale,
        "sqlite_count": len(sq_ids),
        "tantivy_count": len(tv_ids),
        "missing_in_tantivy": missing_in_tantivy,
        "extra_in_tantivy": extra_in_tantivy,
        "shared_count": len(shared),
        "title_mismatches": title_mismatches,
        "text_mismatches": text_mismatches,
        "text_exact_matches": text_match_count,
    }


def _short_diff(a: str, b: str, window: int = 80) -> str:
    if not a:
        return f"(sqlite empty; tantivy starts with {b[:window]!r})"
    if not b:
        return f"(tantivy empty; sqlite starts with {a[:window]!r})"
    # Find first index where the two diverge.
    n = min(len(a), len(b))
    i = 0
    while i < n and a[i] == b[i]:
        i += 1
    lo = max(0, i - 20)
    return f"@{i}: sqlite={a[lo:i + window]!r}; tantivy={b[lo:i + window]!r}"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--locale", help="comma-separated locale filter")
    ap.add_argument(
        "--sample",
        type=int,
        default=0,
        help="only compare the first N shared doc_ids per locale (0 = all)",
    )
    ap.add_argument(
        "--max-text-diffs",
        type=int,
        default=10,
        help="cap on text-diff samples printed per locale",
    )
    args = ap.parse_args()

    dump_bin = find_dump_docs()

    sq = sqlite_locales()
    tv = tantivy_locales()
    available = sorted(sq & tv)
    if not available:
        sys.exit("No locales with both SQLite and Tantivy indexes.")

    if args.locale:
        wanted = {s.strip() for s in args.locale.split(",") if s.strip()}
        available = [l for l in available if l in wanted]
        if not available:
            sys.exit(f"--locale filter matched no available locales (have: {sorted(sq & tv)})")

    print(f"Comparing {len(available)} locale(s): {', '.join(available)}")
    summary: list[dict] = []
    for locale in available:
        print(f"\n=== {locale} ===")
        sqlite_docs = load_sqlite(locale)
        tantivy_docs = load_tantivy(dump_bin, locale)
        result = diff_locale(
            locale,
            sqlite_docs,
            tantivy_docs,
            sample=args.sample,
            max_text_diffs=args.max_text_diffs,
        )
        summary.append(result)
        print(
            f"sqlite={result['sqlite_count']} tantivy={result['tantivy_count']} "
            f"shared={result['shared_count']} "
            f"missing_in_tantivy={len(result['missing_in_tantivy'])} "
            f"extra_in_tantivy={len(result['extra_in_tantivy'])}"
        )
        if result["missing_in_tantivy"]:
            preview = result["missing_in_tantivy"][:10]
            print(f"  REGRESSION: {len(result['missing_in_tantivy'])} doc(s) missing from Tantivy")
            print(f"    sample: {preview}")
        if result["extra_in_tantivy"]:
            # Likely new content since the SQLite db was last built.
            top_guides = Counter(
                d.split(">", 1)[0] for d in result["extra_in_tantivy"]
            ).most_common(5)
            print(f"  +{len(result['extra_in_tantivy'])} new docs in Tantivy (likely new guides)")
            print(f"    top new guides: {top_guides}")
        if result["title_mismatches"]:
            print(f"  title mismatches: {len(result['title_mismatches'])}")
            for doc_id, a, b in result["title_mismatches"][:5]:
                print(f"    {doc_id}\n      sqlite : {a!r}\n      tantivy: {b!r}")
        print(
            f"  search_text: {result['text_exact_matches']}/{result['shared_count']} exact matches"
        )
        if result["text_mismatches"]:
            print(f"  first {len(result['text_mismatches'])} text mismatch sample(s):")
            for doc_id, n_a, n_b, sample in result["text_mismatches"]:
                print(f"    {doc_id}  (sqlite_len={n_a} tantivy_len={n_b})")
                print(f"      {sample}")

    # Aggregate verdict.
    total_regressions = sum(len(r["missing_in_tantivy"]) for r in summary)
    print(f"\n=== overall ===")
    print(f"total locales: {len(summary)}")
    print(f"total missing-in-tantivy (regressions): {total_regressions}")
    print(f"total extra-in-tantivy (additive): {sum(len(r['extra_in_tantivy']) for r in summary)}")
    return 1 if total_regressions > 0 else 0


if __name__ == "__main__":
    raise SystemExit(main())
