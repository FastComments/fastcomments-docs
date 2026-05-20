/**
 * Locale configuration for documentation localization.
 *
 * The source of truth is src/locales.json — both this module and the Rust
 * crates under rust/ read from there. To regenerate the JSON after editing
 * the locale list, run: node scripts/dump-locales.js
 *
 * If you need to add or remove a locale, edit src/locales.json directly.
 */

const fs = require('fs');
const path = require('path');

const data = JSON.parse(fs.readFileSync(path.join(__dirname, 'locales.json'), 'utf8'));

module.exports = {
    defaultLocale: data.defaultLocale,
    locales: data.locales,
};
