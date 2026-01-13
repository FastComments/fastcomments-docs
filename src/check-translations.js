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
const TRANSLATIONS_FILE = path.join(__dirname, 'translations.json');
const verbose = process.argv.includes('--verbose');

function countInlineCode(content) {
    const startMatches = (content.match(/\[inline-code-start\]/g) || []).length;
    const endMatches = (content.match(/\[inline-code-end\]/g) || []).length;
    return {start: startMatches, end: endMatches};
}

/**
 * Count words in content (excluding code blocks)
 * @param {string} content - File content
 * @returns {number} - Word count
 */
function countWords(content) {
    // Remove inline code blocks
    let text = content.replace(/\[inline-code-start\][\s\S]*?\[inline-code-end\]/g, '');
    // Remove code block attributes
    text = text.replace(/\[inline-code-attrs-start[^\]]*\]/g, '');
    // Remove api resource headers
    text = text.replace(/\[api-resource-header-start[^\]]*api-resource-header-end\]/g, '');
    // Remove markdown code blocks
    text = text.replace(/```[\s\S]*?```/g, '');
    // Remove inline code
    text = text.replace(/`[^`]+`/g, '');
    // Count words (split on whitespace and filter empty)
    const words = text.split(/\s+/).filter(w => w.length > 0);
    return words.length;
}

/**
 * Estimate LLM tokens for content
 * Uses approximation: ~4 characters per token for English text
 * For translation prompts, we count both input and approximate output
 * @param {string} content - File content
 * @returns {Object} - {inputTokens, outputTokens, totalTokens}
 */
function estimateTokens(content) {
    // Rough estimate: 1 token ≈ 4 characters
    const inputTokens = Math.ceil(content.length / 4);
    // Output is similar size (translated content) + some overhead
    const outputTokens = Math.ceil(content.length / 4);
    return {
        inputTokens,
        outputTokens,
        totalTokens: inputTokens + outputTokens
    };
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

    const files = fs.readdirSync(defaultPath).filter(file => file.endsWith('.md'));

    // Also check for intro.md and conclusion.md in items/{defaultLocale} or root
    const introInItems = path.join(defaultPath, 'intro.md');
    const conclusionInItems = path.join(defaultPath, 'conclusion.md');
    const introInRoot = path.join(GUIDES_DIR, guideId, 'intro.md');
    const conclusionInRoot = path.join(GUIDES_DIR, guideId, 'conclusion.md');

    if ((fs.existsSync(introInItems) || fs.existsSync(introInRoot)) && !files.includes('intro.md')) {
        files.push('intro.md');
    }
    if ((fs.existsSync(conclusionInItems) || fs.existsSync(conclusionInRoot)) && !files.includes('conclusion.md')) {
        files.push('conclusion.md');
    }

    return files;
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

/**
 * Check translations.json for missing locale translations
 * @returns {Object} - { missingLocales: string[], missingKeys: { locale: string[] } }
 */
function checkUITranslations() {
    if (!fs.existsSync(TRANSLATIONS_FILE)) {
        return { missingLocales: Object.keys(locales), missingKeys: {} };
    }

    const translations = JSON.parse(fs.readFileSync(TRANSLATIONS_FILE, 'utf8'));
    const defaultKeys = Object.keys(translations[defaultLocale] || {});
    const nonDefaultLocales = Object.keys(locales).filter(l => l !== defaultLocale);

    const missingLocales = [];
    const missingKeys = {};

    for (const locale of nonDefaultLocales) {
        if (!translations[locale]) {
            missingLocales.push(locale);
        } else {
            const localeKeys = Object.keys(translations[locale]);
            const missing = defaultKeys.filter(k => !localeKeys.includes(k));
            if (missing.length > 0) {
                missingKeys[locale] = missing;
            }
        }
    }

    return { missingLocales, missingKeys, defaultKeys };
}

/**
 * Get UI translations that need to be translated
 * @returns {Object} - { locale: [keys] }
 */
function getMissingUITranslations() {
    const { missingLocales, missingKeys, defaultKeys } = checkUITranslations();
    const result = {};

    // Locales completely missing get all keys
    for (const locale of missingLocales) {
        result[locale] = defaultKeys;
    }

    // Locales with partial translations get only missing keys
    for (const [locale, keys] of Object.entries(missingKeys)) {
        result[locale] = keys;
    }

    return result;
}

function checkTranslations() {
    const missingTranslations = {};
    const inlineCodeErrors = [];
    let totalMissing = 0;
    let totalFiles = 0;
    let totalWords = 0;
    let totalInputTokens = 0;
    let totalOutputTokens = 0;

    // Cache source file stats to avoid re-reading
    const sourceFileCache = new Map();

    const guides = getGuideDirectories();
    const nonDefaultLocales = Object.keys(locales).filter(l => l !== defaultLocale);

    for (const guideId of guides) {
        if (!hasLocaleStructure(guideId)) {
            if (verbose) {
                console.log(`Skipping ${guideId} (no locale structure)`);
            }
            continue;
        }

        let defaultFiles = getDefaultLocaleFiles(guideId);
        if (defaultFiles.length === 0) continue;

        // Pre-cache source file content and stats, and filter out empty files
        const nonEmptyFiles = [];
        for (const file of defaultFiles) {
            const cacheKey = `${guideId}/${file}`;
            if (!sourceFileCache.has(cacheKey)) {
                const content = getSourceContent(guideId, file);
                // Skip empty or very small files (less than 10 characters)
                if (content.trim().length < 10) {
                    if (verbose) {
                        console.log(`Skipping empty file: ${guideId}/${file}`);
                    }
                    continue;
                }
                sourceFileCache.set(cacheKey, {
                    content,
                    words: countWords(content),
                    tokens: estimateTokens(content)
                });
                nonEmptyFiles.push(file);
            } else {
                // File was already cached, include it
                nonEmptyFiles.push(file);
            }
        }
        defaultFiles = nonEmptyFiles;
        if (defaultFiles.length === 0) continue;

        for (const locale of nonDefaultLocales) {
            const localeFiles = new Set(getLocaleFiles(guideId, locale));
            const missing = defaultFiles.filter(file => !localeFiles.has(file));

            if (missing.length > 0) {
                const key = `${guideId} (${locale})`;
                missingTranslations[key] = missing;
                totalMissing += missing.length;

                // Add word and token counts for missing files
                for (const file of missing) {
                    const cacheKey = `${guideId}/${file}`;
                    const cached = sourceFileCache.get(cacheKey);
                    if (cached) {
                        totalWords += cached.words;
                        totalInputTokens += cached.tokens.inputTokens;
                        totalOutputTokens += cached.tokens.outputTokens;
                    }
                }
            }
            totalFiles += defaultFiles.length;

            // Check inline-code counts for existing translations
            for (const file of defaultFiles) {
                if (!localeFiles.has(file)) continue;

                const cacheKey = `${guideId}/${file}`;
                const cached = sourceFileCache.get(cacheKey);
                const defaultContent = cached ? cached.content : getSourceContent(guideId, file);
                const localeFilePath = path.join(GUIDES_DIR, guideId, 'items', locale, file);
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

    return {
        missingTranslations,
        totalMissing,
        totalFiles,
        inlineCodeErrors,
        totalWords,
        totalInputTokens,
        totalOutputTokens,
        totalTokens: totalInputTokens + totalOutputTokens
    };
}

/**
 * Get detailed missing translation info for programmatic use
 * @returns {Object} Object with missing translations organized by guide and locale
 */
function getMissingTranslations() {
    const result = {};
    const guides = getGuideDirectories();
    const nonDefaultLocales = Object.keys(locales).filter(l => l !== defaultLocale);

    for (const guideId of guides) {
        if (!hasLocaleStructure(guideId)) continue;

        let defaultFiles = getDefaultLocaleFiles(guideId);
        if (defaultFiles.length === 0) continue;

        // Filter out empty or very small files (less than 10 characters)
        defaultFiles = defaultFiles.filter(file => {
            try {
                const content = getSourceContent(guideId, file);
                return content.trim().length >= 10;
            } catch (e) {
                return false;
            }
        });
        if (defaultFiles.length === 0) continue;

        for (const locale of nonDefaultLocales) {
            const localeFiles = new Set(getLocaleFiles(guideId, locale));
            const missing = defaultFiles.filter(file => !localeFiles.has(file));

            if (missing.length > 0) {
                if (!result[guideId]) {
                    result[guideId] = {};
                }
                result[guideId][locale] = missing;
            }
        }
    }

    return result;
}

/**
 * Get content of a source file for translation
 * @param {string} guideId - The guide ID
 * @param {string} filename - The filename
 * @returns {string} - File content
 */
function getSourceContent(guideId, filename) {
    // Check for intro.md and conclusion.md in items/{defaultLocale} first, then root
    if (filename === 'intro.md' || filename === 'conclusion.md') {
        const itemsPath = path.join(GUIDES_DIR, guideId, 'items', defaultLocale, filename);
        if (fs.existsSync(itemsPath)) {
            return fs.readFileSync(itemsPath, 'utf8');
        }
        const rootPath = path.join(GUIDES_DIR, guideId, filename);
        if (fs.existsSync(rootPath)) {
            return fs.readFileSync(rootPath, 'utf8');
        }
        throw new Error(`Source file not found: ${filename} in guide ${guideId}`);
    }

    const filePath = path.join(GUIDES_DIR, guideId, 'items', defaultLocale, filename);
    return fs.readFileSync(filePath, 'utf8');
}

/**
 * Save translated content
 * @param {string} guideId - The guide ID
 * @param {string} locale - Target locale
 * @param {string} filename - The filename
 * @param {string} content - Translated content
 */
function saveTranslation(guideId, locale, filename, content) {
    const localeDir = path.join(GUIDES_DIR, guideId, 'items', locale);
    if (!fs.existsSync(localeDir)) {
        fs.mkdirSync(localeDir, { recursive: true });
    }
    const filePath = path.join(localeDir, filename);
    fs.writeFileSync(filePath, content, 'utf8');
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

    const {
        missingTranslations,
        totalMissing,
        totalFiles,
        inlineCodeErrors,
        totalWords,
        totalInputTokens,
        totalOutputTokens,
        totalTokens
    } = checkTranslations();
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
        console.log('');
        console.log('Estimated translation workload:');
        console.log(`  Words to translate: ${totalWords.toLocaleString()}`);
        console.log(`  LLM tokens (approx): ${totalTokens.toLocaleString()} (${totalInputTokens.toLocaleString()} input + ${totalOutputTokens.toLocaleString()} output)`);
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

    // Check UI translations (translations.json)
    const missingUITranslations = getMissingUITranslations();
    const uiLocalesNeedingTranslation = Object.keys(missingUITranslations);

    if (uiLocalesNeedingTranslation.length > 0) {
        hasErrors = true;
        console.log('\n--- UI Translations (translations.json) ---\n');
        console.log('Missing UI translations:\n');

        let totalMissingKeys = 0;
        for (const locale of uiLocalesNeedingTranslation.sort()) {
            const keys = missingUITranslations[locale];
            const localeInfo = locales[locale];
            const localeName = localeInfo ? localeInfo.nativeName : locale;
            console.log(`  ${locale} (${localeName}): ${keys.length} key(s)`);
            totalMissingKeys += keys.length;
        }
        console.log(`\nTotal: ${totalMissingKeys} missing UI translation key(s) across ${uiLocalesNeedingTranslation.length} locale(s)`);
    }

    if (!hasErrors) {
        console.log('All content is translated and inline-code counts match.');
        process.exit(0);
    }

    process.exit(1);
}

// Export functions for programmatic use
module.exports = {
    getMissingTranslations,
    getMissingUITranslations,
    checkUITranslations,
    getSourceContent,
    saveTranslation,
    checkTranslations,
    checkGuidesNeedingMigration,
    getGuideDirectories,
    getDefaultLocaleFiles,
    getLocaleFiles,
    countInlineCode,
    countWords,
    estimateTokens,
    GUIDES_DIR,
    TRANSLATIONS_FILE,
    locales,
    defaultLocale
};

// Run main only if called directly
if (require.main === module) {
    main();
}
