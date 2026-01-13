#!/bin/bash

set -e  # Exit immediately if any command fails
set -o pipefail  # Exit if any command in a pipeline fails

echo "Copying static files..."
cp -rv src/static/css src/static/generated/
cp -rv src/static/csv src/static/generated/
cp -rv src/static/images src/static/generated/
cp -rv src/static/js src/static/generated/
#cp src/static/*.html src/static/generated/ #uncomment when we add the google verification stuff
