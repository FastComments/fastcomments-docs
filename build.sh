#!/bin/bash

mkdir -p src/static/generated
mkdir src/static/css
mkdir src/static/images
mkdir src/static/js
npm install
node src/app
cp -rv src/static/css src/static/generated/
cp -rv src/static/images src/static/generated/
cp -rv src/static/js src/static/generated/
cp src/static/*.html src/static/generated/
