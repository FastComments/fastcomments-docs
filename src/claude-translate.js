#!/usr/bin/env node

/**
 * CLI tool for Claude Code to manage translations.
 * Wraps check-translations.js to expose translation infrastructure
 * without requiring an external LLM API — Claude Code IS the translator.
 *
 * Usage:
 *   node src/claude-translate.js list [--locale X] [--guide Y] [--limit N]
 *   node src/claude-translate.js source <guideId> <filename>
 *   node src/claude-translate.js save <guideId> <locale> <filename> <filePath>
 *   node src/claude-translate.js list-meta [--locale X] [--guide Y]
 *   node src/claude-translate.js save-meta <guideId> <locale> <filePath>
 *   node src/claude-translate.js list-ui [--locale X]
 */

const fs = require('fs');
const path = require('path');
const {
    checkTranslations,
    getSourceContent,
    saveTranslation,
    countInlineCode,
    getGuideDirectories,
    GUIDES_DIR,
    TRANSLATIONS_FILE,
    locales,
    defaultLocale
} = require('./check-translations');

const { hashContent } = require('./translation-snapshot');

const CACHE_FILE = path.join(__dirname, 'translation-cache.json');

function loadCache() {
    if (!fs.existsSync(CACHE_FILE)) return {};
    try {
        return JSON.parse(fs.readFileSync(CACHE_FILE, 'utf8'));
    } catch (e) {
        return {};
    }
}

function saveCache(cache) {
    fs.writeFileSync(CACHE_FILE, JSON.stringify(cache, null, 2), 'utf8');
}

function parseArgs(args) {
    const opts = {};
    for (let i = 0; i < args.length; i++) {
        if (args[i] === '--locale' && args[i + 1]) {
            opts.locale = args[++i];
        } else if (args[i] === '--guide' && args[i + 1]) {
            opts.guide = args[++i];
        } else if (args[i] === '--limit' && args[i + 1]) {
            opts.limit = parseInt(args[++i], 10);
        }
    }
    return opts;
}

// --- Commands ---

function cmdList(opts) {
    const { missingTranslations, staleTranslations, totalMissing, totalStale } = checkTranslations();
    const items = [];

    for (const [key, files] of Object.entries(missingTranslations)) {
        // key is "guideId (locale)"
        const match = key.match(/^(.+) \((.+)\)$/);
        if (!match) continue;
        const [, guideId, locale] = match;
        if (opts.locale && locale !== opts.locale) continue;
        if (opts.guide && guideId !== opts.guide) continue;
        for (const file of files) {
            items.push({ guideId, locale, file, status: 'missing' });
        }
    }

    for (const [key, files] of Object.entries(staleTranslations)) {
        const match = key.match(/^(.+) \((.+)\)$/);
        if (!match) continue;
        const [, guideId, locale] = match;
        if (opts.locale && locale !== opts.locale) continue;
        if (opts.guide && guideId !== opts.guide) continue;
        for (const file of files) {
            items.push({ guideId, locale, file, status: 'stale' });
        }
    }

    const limit = opts.limit || 50;
    const shown = items.slice(0, limit);

    console.log(JSON.stringify({
        total: items.length,
        totalMissing,
        totalStale,
        showing: shown.length,
        items: shown
    }, null, 2));
}

function cmdSource(guideId, filename) {
    if (!guideId || !filename) {
        console.error('Usage: source <guideId> <filename>');
        process.exit(1);
    }
    const content = getSourceContent(guideId, filename);
    process.stdout.write(content);
}

function cmdSave(guideId, locale, filename, filePath) {
    if (!guideId || !locale || !filename || !filePath) {
        console.error('Usage: save <guideId> <locale> <filename> <filePath>');
        process.exit(1);
    }

    const translatedContent = fs.readFileSync(filePath, 'utf8');
    const sourceContent = getSourceContent(guideId, filename);

    // Validate inline-code counts
    const sourceCounts = countInlineCode(sourceContent);
    const translatedCounts = countInlineCode(translatedContent);
    if (sourceCounts.start !== translatedCounts.start || sourceCounts.end !== translatedCounts.end) {
        console.error(`WARNING: Inline-code count mismatch!`);
        console.error(`  Source: ${sourceCounts.start} start, ${sourceCounts.end} end`);
        console.error(`  Translation: ${translatedCounts.start} start, ${translatedCounts.end} end`);
    }

    // Save the translated file
    saveTranslation(guideId, locale, filename, translatedContent);

    // Update cache
    const cache = loadCache();
    const cacheKey = `${guideId}/${locale}/${filename}`;
    cache[cacheKey] = hashContent(sourceContent);
    saveCache(cache);

    console.log(`Saved ${guideId}/items/${locale}/${filename} and updated cache.`);
}

function cmdListMeta(opts) {
    const guides = getGuideDirectories();
    const nonDefaultLocales = Object.keys(locales).filter(l => l !== defaultLocale);
    const cache = loadCache();
    const items = [];

    for (const guideId of guides) {
        if (opts.guide && guideId !== opts.guide) continue;
        const metaPath = path.join(GUIDES_DIR, guideId, 'meta.json');
        if (!fs.existsSync(metaPath)) continue;

        const metaContent = fs.readFileSync(metaPath, 'utf8');
        const sourceHash = hashContent(metaContent);

        for (const locale of nonDefaultLocales) {
            if (opts.locale && locale !== opts.locale) continue;
            const cacheKey = `${guideId}/${locale}/meta.json`;
            const localeMetaPath = path.join(GUIDES_DIR, guideId, 'meta_translated', `meta_${locale}.json`);
            const missing = !fs.existsSync(localeMetaPath);
            const stale = !missing && cache[cacheKey] !== sourceHash;

            if (missing || stale) {
                items.push({ guideId, locale, status: missing ? 'missing' : 'stale' });
            }
        }
    }

    const limit = opts.limit || 50;
    console.log(JSON.stringify({
        total: items.length,
        showing: Math.min(items.length, limit),
        items: items.slice(0, limit)
    }, null, 2));
}

function cmdSaveMeta(guideId, locale, filePath) {
    if (!guideId || !locale || !filePath) {
        console.error('Usage: save-meta <guideId> <locale> <filePath>');
        process.exit(1);
    }

    const translatedContent = fs.readFileSync(filePath, 'utf8');

    // Validate it's valid JSON
    try {
        JSON.parse(translatedContent);
    } catch (e) {
        console.error(`ERROR: Invalid JSON: ${e.message}`);
        process.exit(1);
    }

    // Save the meta file
    const metaDir = path.join(GUIDES_DIR, guideId, 'meta_translated');
    if (!fs.existsSync(metaDir)) {
        fs.mkdirSync(metaDir, { recursive: true });
    }
    fs.writeFileSync(path.join(metaDir, `meta_${locale}.json`), translatedContent, 'utf8');

    // Update cache
    const metaPath = path.join(GUIDES_DIR, guideId, 'meta.json');
    const sourceHash = hashContent(fs.readFileSync(metaPath, 'utf8'));
    const cache = loadCache();
    cache[`${guideId}/${locale}/meta.json`] = sourceHash;
    saveCache(cache);

    console.log(`Saved meta_${locale}.json for ${guideId} and updated cache.`);
}

function cmdListUI(opts) {
    if (!fs.existsSync(TRANSLATIONS_FILE)) {
        console.error('No translations.json found');
        process.exit(1);
    }

    const translations = JSON.parse(fs.readFileSync(TRANSLATIONS_FILE, 'utf8'));
    const defaultKeys = Object.keys(translations[defaultLocale] || {});
    const nonDefaultLocales = Object.keys(locales).filter(l => l !== defaultLocale);
    const items = [];

    for (const locale of nonDefaultLocales) {
        if (opts.locale && locale !== opts.locale) continue;
        if (!translations[locale]) {
            items.push({ locale, missingKeys: defaultKeys.length, status: 'locale_missing' });
        } else {
            const missing = defaultKeys.filter(k => !translations[locale][k]);
            if (missing.length > 0) {
                items.push({ locale, missingKeys: missing.length, keys: missing, status: 'keys_missing' });
            }
        }
    }

    console.log(JSON.stringify({ total: items.length, items }, null, 2));
}

// --- Main ---

const [,, command, ...rest] = process.argv;
const opts = parseArgs(rest);

switch (command) {
    case 'list':
        cmdList(opts);
        break;
    case 'source':
        cmdSource(rest[0], rest[1]);
        break;
    case 'save':
        cmdSave(rest[0], rest[1], rest[2], rest[3]);
        break;
    case 'list-meta':
        cmdListMeta(opts);
        break;
    case 'save-meta':
        cmdSaveMeta(rest[0], rest[1], rest[2]);
        break;
    case 'list-ui':
        cmdListUI(opts);
        break;
    default:
        console.log(`Usage: claude-translate.js <command>

Commands:
  list [--locale X] [--guide Y] [--limit N]   List missing/stale translations
  source <guideId> <filename>                  Print source content
  save <guideId> <locale> <filename> <file>    Save translation from file, update cache
  list-meta [--locale X] [--guide Y]           List missing/stale meta translations
  save-meta <guideId> <locale> <file>          Save meta translation from file, update cache
  list-ui [--locale X]                         List missing UI translations`);
        process.exit(command ? 1 : 0);
}
