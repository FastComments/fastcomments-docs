#!/bin/bash

set -e  # Exit immediately if any command fails
set -o pipefail  # Exit if any command in a pipeline fails

if [ "$PARTIAL_BUILD" != "true" ]; then
  mkdir -p src/static/generated
  mkdir -p src/static/generated/image-cache
  mkdir -p src/static/css
  mkdir -p src/static/csv
  mkdir -p src/static/images
  mkdir -p src/static/js
  mkdir -p db
  npm install
  rm -f src/static/generated/*.* # when reusing workspaces on the build server, don't let generated index nodes build up over time. -f flag to ignore errors.

  # Check for missing translations
  echo "Checking for missing translations..."
  set +e  # Temporarily disable exit-on-error for translation check
  node src/check-translations.js
  translation_check_result=$?
  set -e  # Re-enable exit-on-error
  if [ $translation_check_result -ne 0 ]; then
    echo "Missing translations detected. Running automated translation..."

    # Run translation
    node src/translate-with-gpt.js
    if [ $? -ne 0 ]; then
      echo "Build failed: Translation failed"
      exit 1
    fi

    # Check if there are changes to commit
    if [ -n "$(git status --porcelain src/content src/translation-cache.json 2>/dev/null)" ]; then
      echo "Committing translation changes..."
      git add -A src/content
      git add src/translation-cache.json
      git commit -m "Automated translation update"
      echo "Translation changes committed."

      echo "Pushing translation changes..."
      git push
      echo "Translation changes pushed."
    else
      echo "No translation changes to commit."
    fi
  else
    echo "All translations up to date."
  fi

  echo "Generating SDK documentation..."
  node src/sdk-guide-generator.js
  if [ $? -ne 0 ]; then
    echo "ERROR: SDK documentation generation failed with exit code $?"
    exit 1
  fi

  echo "Generating custom styling guide..."
  node src/custom-styling-guide-generator.js
  if [ $? -ne 0 ]; then
    echo "ERROR: Custom styling guide generation failed with exit code $?"
    exit 1
  fi

  echo "Building content..."
  NODE_OPTIONS="--max-old-space-size=8192" MAX_BROWSERS=1 npm run build-content
  if [ $? -ne 0 ]; then
    echo "ERROR: Content build failed with exit code $?"
    exit 1
  fi

  echo "Building static..."
  npm run build-static
  if [ $? -ne 0 ]; then
    echo "ERROR: Static build failed with exit code $?"
    exit 1
  fi

  echo "Building search indexes..."
  npm run build-search-index
  if [ $? -ne 0 ]; then
    echo "ERROR: Search index build failed with exit code $?"
    exit 1
  fi

  echo "Build Complete!"
fi
