#!/usr/bin/env node
/**
 * One-shot dumper: reads src/locales.js and writes src/locales.json.
 *
 * After running this once, src/locales.js is refactored to load from the JSON
 * so both Node and the Rust crates share a single source of truth.
 */

const fs = require('fs');
const path = require('path');

const locales = require(path.join(__dirname, '..', 'src', 'locales.js'));
const outPath = path.join(__dirname, '..', 'src', 'locales.json');

const out = {
    defaultLocale: locales.defaultLocale,
    locales: locales.locales,
};

fs.writeFileSync(outPath, JSON.stringify(out, null, 2) + '\n', 'utf8');
console.log(`Wrote ${outPath}`);
