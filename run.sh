#!/bin/bash

# Search server entrypoint (used by fastcomments-docs.service).
#
# This now launches the Rust search server at
# `rust/target/release/server`, which reads Tantivy indexes under
# `index/<locale>/`. The build pipeline (build.sh) only writes Tantivy
# now, so the old Node server (src/server-search-engine.js) reading
# SQLite from `db/` would serve stale results (or fail to start on a
# fresh box).
#
# The env file still provides PORT, OPENAI_API_KEY, OPENAI_MODEL,
# SEARCH_API_KEY — all consumed by the Rust binary with the same
# variable names.

set -eu

DEPLOY_DIR=/home/winrid/fastcomments-docs
BIN="$DEPLOY_DIR/rust/target/release/server"

if [ ! -x "$BIN" ]; then
    echo "ERROR: $BIN missing or not executable. Run build.sh on this host." >&2
    exit 1
fi

# `env $(cat env)` preserves the legacy contract: each line in the env
# file is `KEY=value`, no quoting, one var per line. The Rust server
# reads them via std::env::var, same as the old Node server.
exec env $(cat "$DEPLOY_DIR/env") "$BIN" >> /tmp/fastcomments-docs-search.log 2>&1
