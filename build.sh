#!/bin/bash

# Per-phase wall-clock timing. Every long phase below wraps in `phase`
# so the orchestrator log surfaces a `[phase: NAME] took Ns` line that
# makes "why is the build slow today" answerable from log alone.
BUILD_T0=$SECONDS
phase() {
  local name="$1"; shift
  local t0=$SECONDS
  echo "[phase: $name] start"
  if ! "$@"; then
    echo "[phase: $name] FAILED after $((SECONDS - t0))s"
    return 1
  fi
  echo "[phase: $name] took $((SECONDS - t0))s"
}

# Load NVM and use project-pinned Node.js version
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
nvm use

if [ "$PARTIAL_BUILD" != "true" ]; then
  echo "Creating directories..."
  mkdir -p src/static/generated
  mkdir -p src/static/generated/image-cache
  mkdir -p src/static/css
  mkdir -p src/static/csv
  mkdir -p src/static/images
  mkdir -p src/static/js
  mkdir -p index

  # `npm ci` is faster than `npm install` (uses package-lock.json
  # verbatim, skips the resolver) and guarantees the lockfile is
  # respected so a transitive-dep float can't slip into a build. Falls
  # back to `npm install` when the lockfile is missing (eg. fresh
  # local dev). content-sidecar uses highlight.js; jscpd lives in
  # devDependencies for the dup-code gate below.
  npm_install() {
    if [ -f package-lock.json ]; then
      npm ci --no-audit --no-fund
    else
      npm install --no-audit --no-fund
    fi
  }
  if ! phase "npm" npm_install; then
    echo "ERROR: npm install failed"
    exit 1
  fi

  cargo_build() {
    cargo build --release --manifest-path rust/Cargo.toml
  }
  if ! phase "cargo build" cargo_build; then
    echo "ERROR: Rust build failed. Install rustup from https://rustup.rs/ if missing."
    exit 1
  fi
  # Sanity check: run.sh and rust/target/release/indexer reference the
  # default cargo target dir. If a dev has CARGO_TARGET_DIR set, copy
  # the binaries into the expected location so the rest of build.sh +
  # the service entrypoint still find them.
  CARGO_TARGET="${CARGO_TARGET_DIR:-rust/target}"
  if [ "$CARGO_TARGET" != "rust/target" ]; then
    mkdir -p rust/target/release
    cp -f "$CARGO_TARGET/release/server"  rust/target/release/server
    cp -f "$CARGO_TARGET/release/indexer" rust/target/release/indexer
    cp -f "$CARGO_TARGET/release/sitegen" rust/target/release/sitegen 2>/dev/null || true
    cp -f "$CARGO_TARGET/release/sdkgen"  rust/target/release/sdkgen  2>/dev/null || true
    cp -f "$CARGO_TARGET/release/trans"   rust/target/release/trans   2>/dev/null || true
  fi
  for bin in server indexer sitegen sdkgen trans; do
    if [ ! -x "rust/target/release/$bin" ]; then
      echo "ERROR: rust/target/release/$bin missing after cargo build"
      exit 1
    fi
  done
  echo "Rust build complete."

  # Rust duplicate-code gate. Threshold + scope configured in
  # .jscpd.json; the wrapper script handles jscpd 4.2.3's broken
  # exit-code behavior on threshold breach.
  jscpd_check() {
    node scripts/check-dup-rust.js
  }
  if ! phase "jscpd" jscpd_check; then
    echo "ERROR: Rust duplicate-code threshold exceeded (see jscpd output above)"
    exit 1
  fi

  rm -f src/static/generated/*.* # when reusing workspaces on the build server, don't let generated index nodes build up over time. -f flag to ignore errors.

  # SDK documentation. Rust sdkgen owns the full pipeline now:
  #   - README parser + per-section emission
  #   - OpenAPI generator (fails the build on missing methods / missing
  #     return types, matching the contract Node's
  #     src/sdk-guide-generator.js:268-309 enforced)
  #   - 4 AI generators (typescript/rust/cpp/nim) with the shared
  #     LlmClient hitting src/sdk-ai-cache/ for committed code examples
  #   - meta.json emission with the Node-shaped ordering/categories
  # Output was verified byte-identical to `node src/sdk-guide-generator.js`
  # across all 26 SDKs before this cutover. The Node script remains in
  # the tree for parity comparisons during the transition window but is
  # no longer on the build path.
  sdkgen_run() { ./rust/target/release/sdkgen; }
  if ! phase "sdkgen" sdkgen_run; then
    echo "ERROR: SDK documentation generation failed"
    exit 1
  fi

  if [ -n "$(git status --porcelain src/sdk-ai-cache 2>/dev/null)" ]; then
    echo "Committing SDK AI cache changes..."
    git add -A src/sdk-ai-cache
    if ! git commit -m "Automated SDK AI cache update"; then
      echo "ERROR: SDK AI cache commit failed"
      exit 1
    fi
    echo "Pushing SDK AI cache changes..."
    if ! git push; then
      echo "ERROR: SDK AI cache push failed"
      exit 1
    fi
    echo "SDK AI cache changes pushed."
  else
    echo "No SDK AI cache changes to commit."
  fi

  styling_run() { ./rust/target/release/sitegen custom-styling; }
  if ! phase "custom-styling" styling_run; then
    echo "ERROR: Custom styling guide generation failed"
    exit 1
  fi

  # Translation pipeline. Rust trans owns all three phases now
  # (markdown items, UI strings, meta.json — see rust/trans/src/main.rs).
  # `trans check` flags any gap as a non-zero exit; on miss we branch
  # into `trans run` which translates+writes back. The Node script
  # src/translate-with-gpt.js remains in tree as a parity reference
  # but is no longer on the build path.
  trans_check_t0=$SECONDS
  echo "[phase: trans check] start"
  ./rust/target/release/trans check
  translation_check_result=$?
  echo "[phase: trans check] took $((SECONDS - trans_check_t0))s"
  if [ $translation_check_result -ne 0 ]; then
    trans_run_run() { ./rust/target/release/trans run; }
    if ! phase "trans run" trans_run_run; then
      echo "ERROR: Translation failed"
      exit 1
    fi

    # Check if there are changes to commit. translate covers
    # src/content (markdown items + meta_translated/), src/translations.json
    # (UI strings), src/translation-cache.json (markdown + meta hashes),
    # and src/ui-translation-cache.json (UI hashes).
    if [ -n "$(git status --porcelain src/content src/translations.json src/translation-cache.json src/ui-translation-cache.json 2>/dev/null)" ]; then
      echo "Committing translation changes..."
      git add -A src/content
      git add src/translations.json
      git add src/translation-cache.json
      git add src/ui-translation-cache.json
      if ! git commit -m "Automated translation update"; then
        echo "ERROR: Git commit failed"
        exit 1
      fi
      echo "Translation changes committed."

      echo "Pushing translation changes..."
      if ! git push; then
        echo "ERROR: Git push failed"
        exit 1
      fi
      echo "Translation changes pushed."
    else
      echo "No translation changes to commit."
    fi
  else
    echo "All translations up to date."
  fi

  # MAX_BROWSERS=1 caps chromiumoxide concurrency for the screenshot
  # marker. The Rust sitegen replaced `node src/app` here; the
  # legacy NODE_OPTIONS=--max-old-space-size flag is gone with it.
  sitegen_build() { MAX_BROWSERS=1 ./rust/target/release/sitegen build; }
  if ! phase "sitegen build" sitegen_build; then
    echo "ERROR: Content build failed"
    exit 1
  fi

  # Static file copies (Rust replaces bash build-static.sh).
  sitegen_static() { ./rust/target/release/sitegen build-static; }
  if ! phase "sitegen build-static" sitegen_static; then
    echo "ERROR: Static build failed"
    exit 1
  fi

  # Search indexes. Use the prebuilt binary directly so we don't pay
  # cargo's resolve+check cost on every prod run. The Rust server in
  # run.sh reads exactly these `index/<locale>/` dirs.
  indexer_run() { ./rust/target/release/indexer; }
  if ! phase "indexer" indexer_run; then
    echo "ERROR: Search index build failed"
    exit 1
  fi

  echo "Build Complete in $((SECONDS - BUILD_T0))s!"
fi
