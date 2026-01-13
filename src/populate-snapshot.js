#!/usr/bin/env node

/**
 * One-time script to populate the translation snapshot with existing translations.
 * This scans all source files and their existing translations to build the initial snapshot.
 *
 * Usage: node src/populate-snapshot.js [--dry-run]
 */

const fs = require('fs');
const path = require('path');

const {
    getGuideDirectories,
    getDefaultLocaleFiles,
    getLocaleFiles,
    GUIDES_DIR,
    locales,
    defaultLocale
} = require('./check-translations');

const {
    loadSnapshot,
    saveSnapshot,
    hashFile,
    getSourceKey,
    markTranslated,
    SNAPSHOT_FILE
} = require('./translation-snapshot');

function hasLocaleStructure(guideId) {
    const defaultPath = path.join(GUIDES_DIR, guideId, 'items', defaultLocale);
    return fs.existsSync(defaultPath) && fs.statSync(defaultPath).isDirectory();
}

function main() {
    const dryRun = process.argv.includes('--dry-run');

    console.log('Populating translation snapshot from existing translations...');
    if (dryRun) {
        console.log('Mode: DRY RUN\n');
    }
    console.log('');

    const snapshot = {};
    const nonDefaultLocales = Object.keys(locales).filter(l => l !== defaultLocale);
    const guides = getGuideDirectories();

    let totalSources = 0;
    let totalTranslations = 0;

    for (const guideId of guides) {
        if (!hasLocaleStructure(guideId)) {
            continue;
        }

        const sourceFiles = getDefaultLocaleFiles(guideId);
        if (sourceFiles.length === 0) continue;

        console.log(`Processing ${guideId}...`);

        for (const filename of sourceFiles) {
            const sourcePath = path.join(GUIDES_DIR, guideId, 'items', defaultLocale, filename);
            const sourceHash = hashFile(sourcePath);
            const key = getSourceKey(guideId, filename);

            // Initialize entry
            snapshot[key] = {
                hash: sourceHash,
                translatedLocales: []
            };
            totalSources++;

            // Check which locales have translations
            for (const locale of nonDefaultLocales) {
                const localePath = path.join(GUIDES_DIR, guideId, 'items', locale, filename);
                if (fs.existsSync(localePath)) {
                    snapshot[key].translatedLocales.push(locale);
                    totalTranslations++;
                }
            }

            // Sort locales for consistency
            snapshot[key].translatedLocales.sort();
        }

        const guideTranslations = sourceFiles.reduce((sum, filename) => {
            const key = getSourceKey(guideId, filename);
            return sum + (snapshot[key]?.translatedLocales?.length || 0);
        }, 0);

        console.log(`  ${sourceFiles.length} source files, ${guideTranslations} existing translations`);
    }

    console.log('');
    console.log('--- Summary ---');
    console.log(`Source files: ${totalSources}`);
    console.log(`Existing translations: ${totalTranslations}`);
    console.log(`Snapshot entries: ${Object.keys(snapshot).length}`);

    if (dryRun) {
        console.log('');
        console.log('Dry run - snapshot not saved.');
        console.log('Sample entries:');
        const entries = Object.entries(snapshot).slice(0, 5);
        for (const [key, value] of entries) {
            console.log(`  ${key}: ${value.translatedLocales.length} translations`);
        }
    } else {
        saveSnapshot(snapshot);
        console.log('');
        console.log(`Snapshot saved to: ${SNAPSHOT_FILE}`);
    }
}

main();
