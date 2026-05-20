#!/bin/bash

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
  # `db/` is the legacy SQLite path used by src/server-search-engine.js.
  # Production now runs the Rust server which reads Tantivy from
  # `index/<locale>/`; the dir is still created so the legacy Node
  # server can be started in dev if anyone needs it for comparison.
  mkdir -p db
  mkdir -p index

  echo "Installing dependencies..."
  if ! npm install; then
    echo "ERROR: npm install failed"
    exit 1
  fi

  # better-sqlite3 is only used by the legacy Node search server +
  # legacy indexer scripts. Production search no longer touches it,
  # but `npm install` may still need a native rebuild for dev users.
  echo "Rebuilding native modules..."
  npm rebuild better-sqlite3

  echo "Building Rust workspace (indexer + server)..."
  if ! cargo build --release --manifest-path rust/Cargo.toml; then
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
  for bin in server indexer; do
    if [ ! -x "rust/target/release/$bin" ]; then
      echo "ERROR: rust/target/release/$bin missing after cargo build"
      exit 1
    fi
  done
  echo "Rust build complete."

  rm -f src/static/generated/*.* # when reusing workspaces on the build server, don't let generated index nodes build up over time. -f flag to ignore errors.

  # SDK documentation: README content goes through the Rust sdkgen
  # framework; the existing Node generator continues to fill in
  # OpenAPI + AI-generated sections until those generators are fully
  # ported. Running Node second so its richer content wins for the SDKs
  # that need it.
  echo "Generating SDK documentation..."
  if ! node src/sdk-guide-generator.js; then
    echo "ERROR: SDK documentation generation failed"
    exit 1
  fi
  echo "SDK documentation generation complete."

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

  echo "Generating custom styling guide (Rust)..."
  if ! "$HOME/.cache/cargo-target/release/sitegen" custom-styling \
       && ! ./rust/target/release/sitegen custom-styling; then
    echo "ERROR: Custom styling guide generation failed"
    exit 1
  fi
  echo "Custom styling guide generation complete."

  # Check for missing translations (Rust replaces node src/check-translations.js)
  echo "Checking for missing translations..."
  "$HOME/.cache/cargo-target/release/trans" check 2>/dev/null \
    || ./rust/target/release/trans check
  translation_check_result=$?
  if [ $translation_check_result -ne 0 ]; then
    echo "Missing translations detected. Running automated translation..."

    # Run translation
    if ! node src/translate-with-gpt.js; then
      echo "ERROR: Translation failed"
      exit 1
    fi

    # Check if there are changes to commit
    if [ -n "$(git status --porcelain src/content src/translation-cache.json 2>/dev/null)" ]; then
      echo "Committing translation changes..."
      git add -A src/content
      git add src/translation-cache.json
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

  echo "Building content..."
  if ! NODE_OPTIONS="--max-old-space-size=8192" MAX_BROWSERS=1 npm run build-content; then
    echo "ERROR: Content build failed"
    exit 1
  fi
  echo "Content build complete."

  # Static file copies (Rust replaces bash build-static.sh).
  echo "Building static (Rust)..."
  if ! "$HOME/.cache/cargo-target/release/sitegen" build-static \
       && ! ./rust/target/release/sitegen build-static; then
    echo "ERROR: Static build failed"
    exit 1
  fi
  echo "Static build complete."

  # Search indexes. Use the prebuilt binary directly so we don't pay
  # cargo's resolve+check cost on every prod run. The Rust server in
  # run.sh reads exactly these `index/<locale>/` dirs.
  echo "Building search indexes (Rust)..."
  if ! ./rust/target/release/indexer; then
    echo "ERROR: Search index build failed"
    exit 1
  fi
  echo "Search index build complete."

  echo "Build Complete!"
fi
