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
  mkdir -p db

  echo "Installing dependencies..."
  if ! npm install; then
    echo "ERROR: npm install failed"
    exit 1
  fi

  echo "Rebuilding native modules..."
  npm rebuild better-sqlite3

  echo "Building Rust workspace (indexer + server)..."
  if ! cargo build --release --manifest-path rust/Cargo.toml; then
    echo "ERROR: Rust build failed. Install rustup from https://rustup.rs/ if missing."
    exit 1
  fi
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

  echo "Building search indexes (Rust)..."
  if ! npm run build-search-index-rs; then
    echo "ERROR: Search index build failed"
    exit 1
  fi
  echo "Search index build complete."

  echo "Build Complete!"
fi
