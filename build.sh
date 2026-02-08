#!/bin/bash

# Load NVM to ensure consistent Node.js version with production
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

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

  rm -f src/static/generated/*.* # when reusing workspaces on the build server, don't let generated index nodes build up over time. -f flag to ignore errors.

  # Check for missing translations
  echo "Checking for missing translations..."
  node src/check-translations.js
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

  echo "Generating SDK documentation..."
  if ! node src/sdk-guide-generator.js; then
    echo "ERROR: SDK documentation generation failed"
    exit 1
  fi
  echo "SDK documentation generation complete."

  echo "Generating custom styling guide..."
  if ! node src/custom-styling-guide-generator.js; then
    echo "ERROR: Custom styling guide generation failed"
    exit 1
  fi
  echo "Custom styling guide generation complete."

  echo "Building content..."
  if ! NODE_OPTIONS="--max-old-space-size=8192" MAX_BROWSERS=1 npm run build-content; then
    echo "ERROR: Content build failed"
    exit 1
  fi
  echo "Content build complete."

  echo "Building static..."
  if ! npm run build-static; then
    echo "ERROR: Static build failed"
    exit 1
  fi
  echo "Static build complete."

  echo "Building search indexes..."
  if ! npm run build-search-index; then
    echo "ERROR: Search index build failed"
    exit 1
  fi
  echo "Search index build complete."

  echo "Build Complete!"
fi
