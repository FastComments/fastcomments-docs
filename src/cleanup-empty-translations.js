#!/usr/bin/env node

/**
 * Script to clean up translated intro.md and conclusion.md files that only contain "---" markers.
 * These were generated from translating empty source files.
 */

const fs = require('fs');
const path = require('path');

const GUIDES_DIR = path.join(__dirname, 'content', 'guides');

function cleanupEmptyTranslations() {
    let cleaned = 0;

    // Find all translated intro.md and conclusion.md files
    const guides = fs.readdirSync(GUIDES_DIR).filter(name => {
        const guidePath = path.join(GUIDES_DIR, name);
        return fs.statSync(guidePath).isDirectory();
    });

    for (const guideId of guides) {
        const itemsPath = path.join(GUIDES_DIR, guideId, 'items');
        if (!fs.existsSync(itemsPath)) continue;

        const locales = fs.readdirSync(itemsPath).filter(name => {
            const localePath = path.join(itemsPath, name);
            return fs.statSync(localePath).isDirectory();
        });

        for (const locale of locales) {
            const localePath = path.join(itemsPath, locale);

            // Check intro.md
            const introPath = path.join(localePath, 'intro.md');
            if (fs.existsSync(introPath)) {
                const content = fs.readFileSync(introPath, 'utf8').trim();
                // Check if it's just "---" markers or empty
                if (content === '---\n\n---' || content === '---' || content === '') {
                    fs.unlinkSync(introPath);
                    console.log(`Deleted: ${guideId}/${locale}/intro.md`);
                    cleaned++;
                }
            }

            // Check conclusion.md
            const conclusionPath = path.join(localePath, 'conclusion.md');
            if (fs.existsSync(conclusionPath)) {
                const content = fs.readFileSync(conclusionPath, 'utf8').trim();
                // Check if it's just "---" markers or empty
                if (content === '---\n\n---' || content === '---' || content === '') {
                    fs.unlinkSync(conclusionPath);
                    console.log(`Deleted: ${guideId}/${locale}/conclusion.md`);
                    cleaned++;
                }
            }
        }
    }

    console.log(`\nCleaned up ${cleaned} empty translation file(s)`);
    return cleaned;
}

if (require.main === module) {
    cleanupEmptyTranslations();
}

module.exports = { cleanupEmptyTranslations };
