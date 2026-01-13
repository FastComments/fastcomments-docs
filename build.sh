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
  npm install
  rm -f src/static/generated/*.* # when reusing workspaces on the build server, don't let generated index nodes build up over time. -f flag to ignore errors.

  # Check for missing translations
  echo "Checking for missing translations..."
  node src/check-translations.js
  if [ $? -ne 0 ]; then
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
      git add src/content src/translation-cache.json
      git commit -m "Automated translation update"
      echo "Translation changes committed."
    else
      echo "No translation changes to commit."
    fi
  else
    echo "All translations up to date."
  fi

  echo "Generating SDK documentation..."
  node src/sdk-guide-generator.js
  echo "Generating custom styling guide..."
  node src/custom-styling-guide-generator.js
  MAX_BROWSERS=1 npm run build-content
  npm run build-static
fi
