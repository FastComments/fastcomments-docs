#!/usr/bin/env node

/**
 * Script to check for untranslated content.
 * Returns exit code 1 if there are missing translations or guides needing migration, 0 if all content is translated.
 *
 * Usage: node src/check-translations.js [--verbose]
 */

const fs = require('fs');
const path = require('path');
const {locales, defaultLocale} = require('./locales');

const GUIDES_DIR = path.join(__dirname, 'content', 'guides');
const verbose = process.argv.includes('--verbose');

function countInlineCode(content) {
    const startMatches = (content.match(/\[inline-code-start\]/g) || []).length;
    const endMatches = (content.match(/\[inline-code-end\]/g) || []).length;
    return {start: startMatches, end: endMatches};
}

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

function getFlatMarkdownFiles(guideId) {
    const itemsPath = path.join(GUIDES_DIR, guideId, 'items');
    if (!fs.existsSync(itemsPath)) return [];

    return fs.readdirSync(itemsPath).filter(file => file.endsWith('.md'));
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

function checkGuidesNeedingMigration() {
    const guides = getGuideDirectories();
    const needsMigration = [];

    for (const guideId of guides) {
        if (!hasLocaleStructure(guideId)) {
            const flatFiles = getFlatMarkdownFiles(guideId);
            if (flatFiles.length > 0) {
                needsMigration.push({
                    guideId,
                    fileCount: flatFiles.length
                });
            }
        }
    }

    return needsMigration;
}

function checkTranslations() {
    const missingTranslations = {};
    const inlineCodeErrors = [];
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

            // Check inline-code counts for existing translations
            for (const file of defaultFiles) {
                if (!localeFiles.has(file)) continue;

                const defaultFilePath = path.join(GUIDES_DIR, guideId, 'items', defaultLocale, file);
                const localeFilePath = path.join(GUIDES_DIR, guideId, 'items', locale, file);

                const defaultContent = fs.readFileSync(defaultFilePath, 'utf8');
                const localeContent = fs.readFileSync(localeFilePath, 'utf8');

                const defaultCounts = countInlineCode(defaultContent);
                const localeCounts = countInlineCode(localeContent);

                if (defaultCounts.start !== localeCounts.start || defaultCounts.end !== localeCounts.end) {
                    inlineCodeErrors.push({
                        guide: guideId,
                        locale,
                        file,
                        expected: defaultCounts,
                        actual: localeCounts
                    });
                }
            }
        }
    }

    return {missingTranslations, totalMissing, totalFiles, inlineCodeErrors};
}

function main() {
    console.log('Checking for untranslated content...\n');

    // Check for guides needing migration first
    const needsMigration = checkGuidesNeedingMigration();
    if (needsMigration.length > 0) {
        console.log('Guides needing locale structure migration:\n');
        let totalMigrationFiles = 0;
        for (const {guideId, fileCount} of needsMigration.sort((a, b) => a.guideId.localeCompare(b.guideId))) {
            console.log(`  ${guideId}: ${fileCount} file(s)`);
            totalMigrationFiles += fileCount;
        }
        console.log(`\nTotal: ${needsMigration.length} guide(s) with ${totalMigrationFiles} file(s) need migration to items/en/ structure\n`);
        console.log('Run: node src/migrate-to-locale-structure.js\n');
        console.log('---\n');
    }

    const {missingTranslations, totalMissing, totalFiles, inlineCodeErrors} = checkTranslations();
    const guides = Object.keys(missingTranslations);
    let hasErrors = needsMigration.length > 0;

    if (guides.length > 0) {
        hasErrors = true;
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
    }

    if (inlineCodeErrors.length > 0) {
        hasErrors = true;
        console.log('\nInline-code count mismatches:\n');

        for (const error of inlineCodeErrors) {
            console.log(`${error.guide}/${error.locale}/${error.file}:`);
            console.log(`  Expected: ${error.expected.start} start, ${error.expected.end} end`);
            console.log(`  Actual:   ${error.actual.start} start, ${error.actual.end} end`);
            console.log('');
        }

        console.log(`Total: ${inlineCodeErrors.length} file(s) with inline-code mismatches`);
    }

    if (!hasErrors) {
        console.log('All content is translated and inline-code counts match.');
        process.exit(0);
    }

    process.exit(1);
}

main();
