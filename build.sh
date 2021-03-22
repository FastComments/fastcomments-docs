#!/bin/bash

mkdir -p src/static/generated
mkdir -p src/static/css
mkdir -p src/static/images
mkdir -p src/static/js
npm install
rm src/static/generated/*.* # when reusing workspaces on the build server, don't let generated index nodes build up over time
npm run build
cp -rv src/static/css src/static/generated/
cp -rv src/static/images src/static/generated/
cp -rv src/static/js src/static/generated/
#cp src/static/*.html src/static/generated/ #uncomment when we add the google verification stuff
