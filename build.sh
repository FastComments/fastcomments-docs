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

  # SDK documentation. Rust sdkgen owns the full pipeline:
  # README parser, OpenAPI generator (fails on missing methods/return
  # types), 4 AI generators (typescript/rust/cpp/nim) sharing LlmClient
  # against src/sdk-ai-cache/, and meta.json emission.
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

  # Translation pipeline. Rust trans owns all three phases (markdown
  # items, UI strings, meta.json - see rust/trans/src/main.rs).
  # `trans check` flags any gap as a non-zero exit; on miss we branch
  # into `trans run` which translates+writes back.
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

  # Image parity gate. Every translated item must reference the exact
  # same images as its default-locale source (<img src>, markdown
  # ![](url), and [app-screenshot-*] attributes minus the translatable
  # title). A translator that drops, duplicates, or "translates" an
  # image path silently ships a localized page with a missing or wrong
  # picture, so this is a hard failure rather than a warning. Runs
  # after the translation block so freshly-written output is covered.
  validate_images_run() { ./rust/target/release/trans validate-images; }
  if ! phase "trans validate-images" validate_images_run; then
    echo "ERROR: translated guides do not have the same images as the reference locale"
    exit 1
  fi

  # Link parity gate. Same argument as the image gate, for the other class
  # of technical value a translator mangles: it has dropped a path segment,
  # injected a zero-width space into a hostname, and truncated paths
  # outright, each shipping a 404 that only an external crawl ever caught.
  # `trans run` now restores link targets from the source deterministically,
  # so this can only fail on something that pass didn't cover. Link COUNT
  # mismatches are deliberately not fatal - there is no deterministic repair
  # for a dropped link, so gating on it would wedge the build behind the
  # translator; those re-translate via `trans check` instead.
  validate_links_run() { ./rust/target/release/trans validate-links; }
  if ! phase "trans validate-links" validate_links_run; then
    echo "ERROR: translated guides link somewhere the reference locale does not"
    exit 1
  fi

  # MAX_BROWSERS=1 caps chromiumoxide concurrency for the screenshot marker.
  sitegen_build() { MAX_BROWSERS=1 ./rust/target/release/sitegen build; }
  if ! phase "sitegen build" sitegen_build; then
    echo "ERROR: Content build failed"
    exit 1
  fi

  # Heading gate. Duplicate H1s dilute the page topic and reached
  # production from two unrelated directions (a README H1 that survived
  # sdkgen's strip, and translators promoting a leading paragraph to a
  # heading), so this checks the rendered HTML - the only place both
  # causes are visible. Pages with no H1 are warned about, not failed.
  validate_headings_run() { ./rust/target/release/sitegen validate-headings; }
  if ! phase "sitegen validate-headings" validate_headings_run; then
    echo "ERROR: generated pages have more than one <h1>"
    exit 1
  fi

  # Static file copies.
  sitegen_static() { ./rust/target/release/sitegen build-static; }
  if ! phase "sitegen build-static" sitegen_static; then
    echo "ERROR: Static build failed"
    exit 1
  fi

  # Asset gate. `process_screenshots` inlines the <img> for an
  # [app-screenshot-*] marker before it captures the PNG, so a capture
  # that fails leaves a dangling reference and the build still exits 0.
  # That shipped a 404 image on the ja_jp comment-vote-verification page
  # that only an external crawl caught. Runs after build-static because
  # that is what puts src/static/images/** under generated/.
  validate_assets_run() { ./rust/target/release/sitegen validate-assets; }
  if ! phase "sitegen validate-assets" validate_assets_run; then
    echo "ERROR: generated pages reference assets that do not exist"
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
