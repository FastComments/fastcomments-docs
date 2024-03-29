#!/bin/bash

mkdir -p src/static/generated
mkdir -p src/static/generated/image-cache
mkdir -p src/static/css
mkdir -p src/static/csv
mkdir -p src/static/images
mkdir -p src/static/js
npm install
rm -f src/static/generated/*.* # when reusing workspaces on the build server, don't let generated index nodes build up over time. -f flag to ignore errors.
MAX_BROWSERS=1 npm run build-content
cp -rv src/static/css src/static/generated/
cp -rv src/static/csv src/static/generated/
cp -rv src/static/images src/static/generated/
cp -rv src/static/js src/static/generated/
#cp src/static/*.html src/static/generated/ #uncomment when we add the google verification stuff
