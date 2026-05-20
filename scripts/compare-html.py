#!/usr/bin/env python3
"""
Site-output parity harness. Runs the Node site generator
(`npm run build-content`) and the Rust sitegen (`./rust/target/release/sitegen
build`) against the same source tree, snapshots each output, and reports
per-file diffs.

Usage:
  python3 scripts/compare-html.py                # build both, diff all
  python3 scripts/compare-html.py --skip-node    # use the existing
                                                 # src/static/generated as
                                                 # the baseline snapshot
                                                 # (faster iteration)
  python3 scripts/compare-html.py --locale en    # restrict to one locale's
                                                 # guide-*-LOCALE.html files
  python3 scripts/compare-html.py --max-diffs N  # cap the number of
                                                 # per-file diff samples

Acceptance: every guide-*.html, index*.html, and sitemap.xml should be
byte-identical except for the random build-id and the current-timestamp
`lastUpdateDate`. Both are normalized before comparison.

Designed to be run from the repo root.
"""

from __future__ import annotations

import argparse
import difflib
import re
import shutil
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
GENERATED = REPO / "src/static/generated"
NODE_SNAPSHOT = REPO / "_parity/node"
RUST_SNAPSHOT = REPO / "_parity/rust"

BUILD_ID_RE = re.compile(r"build-id[^\"]*\"\?v=[A-Za-z0-9_-]+\"|\?v=[A-Za-z0-9_-]+")
LAST_UPDATE_RE = re.compile(
    r"Documentation Last Updated [A-Za-z0-9/:., ]+(?=</div>)"
)


def normalize(html: str) -> str:
    """Normalize away values that are expected to differ between runs."""
    # Random build-id appended to script src URLs and the version-check
    # comparator.
    html = re.sub(r"\?v=[A-Za-z0-9_-]+", "?v=BUILDID", html)
    # `lastUpdateDate` is a wall-clock timestamp.
    html = re.sub(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}", "TIMESTAMP", html)
    # Node's `new Date().toLocaleString()` may use locale-ish format
    # (e.g. "5/19/2026, 11:30:00 PM") — collapse to TIMESTAMP too.
    html = re.sub(
        r"\d{1,2}/\d{1,2}/\d{4},?\s+\d{1,2}:\d{2}(:\d{2})?\s*(?:AM|PM|am|pm)?",
        "TIMESTAMP",
        html,
    )
    return html


def snapshot_to(dst: Path) -> int:
    """Copy GENERATED into dst, replacing whatever was there."""
    if dst.exists():
        shutil.rmtree(dst)
    if not GENERATED.exists():
        sys.exit(f"missing source: {GENERATED}")
    shutil.copytree(GENERATED, dst)
    return sum(1 for _ in dst.rglob("*") if _.is_file())


def run_node() -> int:
    if GENERATED.exists():
        shutil.rmtree(GENERATED)
    GENERATED.mkdir(parents=True)
    print("==> running npm run build-content ...", flush=True)
    proc = subprocess.run(
        ["npm", "run", "build-content"],
        cwd=REPO,
        env={**__import__("os").environ, "NODE_OPTIONS": "--max-old-space-size=8192", "MAX_BROWSERS": "1"},
    )
    if proc.returncode != 0:
        sys.exit(f"npm run build-content failed: {proc.returncode}")
    return snapshot_to(NODE_SNAPSHOT)


def run_rust(locale: str | None) -> int:
    if GENERATED.exists():
        shutil.rmtree(GENERATED)
    GENERATED.mkdir(parents=True)
    bin_candidates = [
        Path.home() / ".cache/cargo-target/release/sitegen",
        REPO / "rust/target/release/sitegen",
    ]
    sitegen = next((p for p in bin_candidates if p.exists()), None)
    if sitegen is None:
        sys.exit(
            "no compiled sitegen binary; build with:\n"
            "  (cd rust && cargo build --release -p fcdocs-sitegen)"
        )
    args = [str(sitegen), "build"]
    if locale:
        args.extend(["--locale", locale])
    print(f"==> running {' '.join(args)} ...", flush=True)
    proc = subprocess.run(args, cwd=REPO)
    if proc.returncode != 0:
        sys.exit(f"sitegen build failed: {proc.returncode}")
    return snapshot_to(RUST_SNAPSHOT)


def diff(args) -> int:
    if not NODE_SNAPSHOT.exists() or not RUST_SNAPSHOT.exists():
        sys.exit("missing one or both snapshots — re-run without --skip-* flags")

    node_files = {p.relative_to(NODE_SNAPSHOT) for p in NODE_SNAPSHOT.rglob("*") if p.is_file()}
    rust_files = {p.relative_to(RUST_SNAPSHOT) for p in RUST_SNAPSHOT.rglob("*") if p.is_file()}

    only_node = sorted(node_files - rust_files)
    only_rust = sorted(rust_files - node_files)
    shared = sorted(node_files & rust_files)

    print(f"\nFiles: node={len(node_files)}  rust={len(rust_files)}  shared={len(shared)}")
    print(f"  only-in-node:  {len(only_node)}")
    print(f"  only-in-rust:  {len(only_rust)}")

    if only_node:
        print("\nMissing from Rust (sample):")
        for p in only_node[:20]:
            print(f"  - {p}")
    if only_rust:
        print("\nExtra in Rust (sample):")
        for p in only_rust[:20]:
            print(f"  + {p}")

    exact = 0
    diff_sample = 0
    differing = []
    for rel in shared:
        if args.locale and "-" in rel.name and not str(rel).endswith(f"-{args.locale}.html"):
            # crude per-locale filter for the guide pages
            if rel.suffix == ".html" and not str(rel).startswith("index"):
                continue
        try:
            n = (NODE_SNAPSHOT / rel).read_text(encoding="utf-8")
            r = (RUST_SNAPSHOT / rel).read_text(encoding="utf-8")
        except UnicodeDecodeError:
            # Binary file: byte-compare.
            n_b = (NODE_SNAPSHOT / rel).read_bytes()
            r_b = (RUST_SNAPSHOT / rel).read_bytes()
            if n_b == r_b:
                exact += 1
            else:
                differing.append(rel)
            continue
        if normalize(n) == normalize(r):
            exact += 1
        else:
            differing.append(rel)
            if diff_sample < args.max_diffs:
                show_diff(rel, n, r)
                diff_sample += 1

    print(f"\n=== summary ===")
    print(f"exact (after normalization): {exact}/{len(shared)} ({100 * exact / max(len(shared),1):.1f}%)")
    print(f"differing: {len(differing)}")
    if differing:
        print("\nFirst 20 differing files:")
        for d in differing[:20]:
            print(f"  ~ {d}")
    return 0 if (len(only_node) == 0 and len(differing) == 0) else 1


def show_diff(rel: Path, node_text: str, rust_text: str):
    n_lines = normalize(node_text).splitlines()
    r_lines = normalize(rust_text).splitlines()
    diff = list(
        difflib.unified_diff(
            n_lines, r_lines, fromfile=f"node:{rel}", tofile=f"rust:{rel}", n=2, lineterm=""
        )
    )
    if not diff:
        return
    print(f"\n--- {rel} ---")
    for line in diff[:60]:
        print(line)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--skip-node", action="store_true", help="reuse last node snapshot")
    ap.add_argument("--skip-rust", action="store_true", help="reuse last rust snapshot")
    ap.add_argument("--locale", help="restrict comparison to one locale's guide pages")
    ap.add_argument("--max-diffs", type=int, default=3, help="cap on diff samples printed")
    args = ap.parse_args()

    if not args.skip_node:
        n = run_node()
        print(f"node snapshot: {n} files")
    if not args.skip_rust:
        r = run_rust(args.locale)
        print(f"rust snapshot: {r} files")
    return diff(args)


if __name__ == "__main__":
    raise SystemExit(main())
