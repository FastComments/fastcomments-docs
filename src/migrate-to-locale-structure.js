#!/usr/bin/env node

/**
 * Migrates guides from flat items/*.md structure to items/en/*.md locale structure.
 *
 * Usage: node src/migrate-to-locale-structure.js
 */

const fs = require('fs');
const path = require('path');
const {defaultLocale} = require('./locales');

const GUIDES_DIR = path.join(__dirname, 'content', 'guides');

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

function migrateGuide(guideId) {
    const itemsPath = path.join(GUIDES_DIR, guideId, 'items');
    const enPath = path.join(itemsPath, defaultLocale);

    // Get all markdown files in items/
    const mdFiles = fs.readdirSync(itemsPath).filter(file => file.endsWith('.md'));

    if (mdFiles.length === 0) {
        return {migrated: 0};
    }

    // Create en/ directory
    if (!fs.existsSync(enPath)) {
        fs.mkdirSync(enPath, {recursive: true});
    }

    // Move each file
    for (const file of mdFiles) {
        const srcPath = path.join(itemsPath, file);
        const destPath = path.join(enPath, file);
        fs.renameSync(srcPath, destPath);
    }

    return {migrated: mdFiles.length};
}

function main() {
    console.log('Migrating guides to locale structure...\n');

    const guides = getGuideDirectories();
    let totalMigrated = 0;
    let guidesProcessed = 0;

    for (const guideId of guides.sort()) {
        if (hasLocaleStructure(guideId)) {
            continue;
        }

        const itemsPath = path.join(GUIDES_DIR, guideId, 'items');
        if (!fs.existsSync(itemsPath)) {
            continue;
        }

        const {migrated} = migrateGuide(guideId);
        if (migrated > 0) {
            console.log(`  ${guideId}: moved ${migrated} file(s) to items/${defaultLocale}/`);
            totalMigrated += migrated;
            guidesProcessed++;
        }
    }

    console.log(`\nMigration complete: ${guidesProcessed} guide(s), ${totalMigrated} file(s) moved.`);
}

main();
