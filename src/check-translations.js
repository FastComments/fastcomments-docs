#!/usr/bin/env node

/**
 * Script to check for untranslated content.
 * Returns exit code 1 if there are missing translations, 0 if all content is translated.
 *
 * Usage: node src/check-translations.js [--verbose]
 */

const fs = require('fs');
const path = require('path');
const {locales, defaultLocale} = require('./locales');

const GUIDES_DIR = path.join(__dirname, 'content', 'guides');
const verbose = process.argv.includes('--verbose');

function getGuideDirectories() {
    return fs.readdirSync(GUIDES_DIR).filter(name => {
        const guidePath = path.join(GUIDES_DIR, name);
        return fs.statSync(guidePath).isDirectory() &&
               fs.existsSync(path.join(guidePath, 'meta.json'));
    });
}

function hasLocaleStructure(guideId) {
    const itemsPath = path.join(GUIDES_DIR, guideId, 'items');
    if (!fs.existsSync(itemsPath)) return false;

    const defaultLocalePath = path.join(itemsPath, defaultLocale);
    return fs.existsSync(defaultLocalePath) && fs.statSync(defaultLocalePath).isDirectory();
}

function getDefaultLocaleFiles(guideId) {
    const defaultPath = path.join(GUIDES_DIR, guideId, 'items', defaultLocale);
    if (!fs.existsSync(defaultPath)) return [];

    return fs.readdirSync(defaultPath).filter(file => file.endsWith('.md'));
}

function getLocaleFiles(guideId, locale) {
    const localePath = path.join(GUIDES_DIR, guideId, 'items', locale);
    if (!fs.existsSync(localePath)) return [];

    return fs.readdirSync(localePath).filter(file => file.endsWith('.md'));
}

function checkTranslations() {
    const missingTranslations = {};
    let totalMissing = 0;
    let totalFiles = 0;

    const guides = getGuideDirectories();
    const nonDefaultLocales = Object.keys(locales).filter(l => l !== defaultLocale);

    for (const guideId of guides) {
        if (!hasLocaleStructure(guideId)) {
            if (verbose) {
                console.log(`Skipping ${guideId} (no locale structure)`);
            }
            continue;
        }

        const defaultFiles = getDefaultLocaleFiles(guideId);
        if (defaultFiles.length === 0) continue;

        for (const locale of nonDefaultLocales) {
            const localeFiles = new Set(getLocaleFiles(guideId, locale));
            const missing = defaultFiles.filter(file => !localeFiles.has(file));

            if (missing.length > 0) {
                const key = `${guideId} (${locale})`;
                missingTranslations[key] = missing;
                totalMissing += missing.length;
            }
            totalFiles += defaultFiles.length;
        }
    }

    return {missingTranslations, totalMissing, totalFiles};
}

function main() {
    console.log('Checking for untranslated content...\n');

    const {missingTranslations, totalMissing, totalFiles} = checkTranslations();
    const guides = Object.keys(missingTranslations);

    if (guides.length === 0) {
        console.log('All content is translated.');
        process.exit(0);
    }

    console.log('Missing translations:\n');

    for (const guide of guides.sort()) {
        const files = missingTranslations[guide];
        console.log(`${guide}:`);
        for (const file of files.sort()) {
            console.log(`  - ${file}`);
        }
        console.log('');
    }

    const translated = totalFiles - totalMissing;
    const percentage = totalFiles > 0 ? ((translated / totalFiles) * 100).toFixed(1) : 0;

    console.log('---');
    console.log(`Total: ${totalMissing} missing translations across ${guides.length} guide(s)`);
    console.log(`Coverage: ${translated}/${totalFiles} files translated (${percentage}%)`);

    process.exit(1);
}

main();
