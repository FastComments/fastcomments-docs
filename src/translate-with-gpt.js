#!/usr/bin/env node

/**
 * Script to translate missing documentation using OpenAI GPT API.
 * Uses check-translations.js as a library to identify missing translations.
 * Uses snapshot mechanism to detect when source files have changed.
 *
 * Usage: node src/translate-with-gpt.js [--locale <locale>] [--guide <guideId>] [--concurrency <n>] [--dry-run] [--force]
 *
 * Environment variables:
 *   OPENAI_API_KEY - Required. Your OpenAI API key.
 *   OPENAI_MODEL - Optional. Model to use (default: gpt-5-mini).
 */

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const {
    getMissingTranslations,
    getMissingUITranslations,
    getSourceContent,
    saveTranslation,
    countInlineCode,
    getGuideDirectories,
    getDefaultLocaleFiles,
    getLocaleFiles,
    loadUITranslationCache,
    saveUITranslationCache,
    getUITranslationCacheKey,
    GUIDES_DIR,
    TRANSLATIONS_FILE,
    locales,
    defaultLocale
} = require('./check-translations');

const { hashContent } = require('./translation-snapshot');

const OPENAI_API_KEY = process.env.OPENAI_API_KEY;
const OPENAI_MODEL = process.env.OPENAI_MODEL || 'gpt-5-mini';
const ALTERNATE_MODELS = ['gpt-4.1', 'gpt-5.2']; // Fallback models for large files
const CACHE_FILE = path.join(__dirname, 'translation-cache.json');

/**
 * Load translation cache from disk
 * @returns {Object} - Cache object { "guideId/locale/filename": "md5hash", ... }
 */
function loadCache() {
    if (!fs.existsSync(CACHE_FILE)) {
        return {};
    }
    try {
        return JSON.parse(fs.readFileSync(CACHE_FILE, 'utf8'));
    } catch (e) {
        console.warn(`Failed to load cache: ${e.message}`);
        return {};
    }
}

/**
 * Save translation cache to disk
 * @param {Object} cache - Cache object
 */
function saveCache(cache) {
    fs.writeFileSync(CACHE_FILE, JSON.stringify(cache, null, 2), 'utf8');
}

/**
 * Generate cache key from path components
 * @param {string} guideId - Guide ID
 * @param {string} locale - Target locale
 * @param {string} filename - Filename
 * @returns {string} - Cache key
 */
function getCacheKey(guideId, locale, filename) {
    return `${guideId}/${locale}/${filename}`;
}

/**
 * Check if translation is cached with matching source hash
 * @param {Object} cache - Cache object
 * @param {string} key - Cache key
 * @param {string} sourceHash - MD5 hash of source content
 * @returns {boolean} - True if cached and hash matches
 */
function isCached(cache, key, sourceHash) {
    return cache[key] === sourceHash;
}

/**
 * Update cache entry
 * @param {Object} cache - Cache object
 * @param {string} key - Cache key
 * @param {string} sourceHash - MD5 hash of source content
 */
function updateCache(cache, key, sourceHash) {
    cache[key] = sourceHash;
}

/**
 * Translation client using OpenAI API
 */
class TranslationClient {
    constructor() {
        this.apiKey = OPENAI_API_KEY;
        this.model = OPENAI_MODEL;
    }

    /**
     * Build system message for translation
     * @param {string} locale - Target locale
     * @returns {string} - System message
     */
    getSystemMessage(locale) {
        const localeInfo = locales[locale];
        const localeName = localeInfo ? localeInfo.nativeName : locale;

        return `You are an expert technical translator specializing in software documentation.
You translate from English to ${localeName} (${locale}).
You maintain the exact same formatting, structure, and technical accuracy.
You NEVER translate code, variable names, API endpoints, or technical identifiers.
You preserve all markdown formatting and special tags exactly as they appear.`;
    }

    /**
     * Build translation prompt
     * @param {string} content - Source content
     * @param {string} locale - Target locale
     * @returns {string} - Prompt text
     */
    buildPrompt(content, locale) {
        const localeInfo = locales[locale];
        const localeName = localeInfo ? localeInfo.nativeName : locale;

        const lines = [];

        lines.push(`Translate the following FastComments documentation from English to ${localeName}.`);
        lines.push('');
        lines.push('CRITICAL RULES:');
        lines.push('1. Retain code and logic in [inline-code-start] and [inline-code-end] blocks exactly, just translate comments.');
        lines.push('2. DO NOT translate anything inside [inline-code-attrs-start ...] tags - preserve them exactly');
        lines.push('3. DO NOT translate [api-resource-header-start ...] tags - preserve them exactly');
        lines.push('4. DO NOT translate code blocks (```...```) or inline code (`...`) except comments.');
        lines.push('5. DO NOT translate URLs, API endpoints, variable names, or technical identifiers');
        lines.push('6. DO NOT translate property names in TypeScript/JavaScript interfaces');
        lines.push('7. PRESERVE all special tags and their attributes exactly as written');
        lines.push('8. PRESERVE all markdown formatting (headers, lists, bold, links, etc.)');
        lines.push('9. Translate ONLY the natural language text (descriptions, explanations)');
        lines.push('10. Keep the same line structure and paragraph breaks');
        lines.push('');
        lines.push('The title attributes in [inline-code-attrs-start] tags SHOULD be translated.');
        lines.push('For example: title = \'Example cURL Request\' should become title = \'Exemple de requête cURL\' in French.');
        lines.push('');
        lines.push('SOURCE CONTENT:');
        lines.push('---');
        lines.push(content);
        lines.push('---');
        lines.push('');
        lines.push('Return ONLY the translated content, nothing else. No explanations or notes.');

        return lines.join('\n');
    }

    /**
     * Sleep for a given number of milliseconds
     * @param {number} ms - Milliseconds to sleep
     * @returns {Promise<void>}
     */
    async sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    /**
     * Call OpenAI API to translate content with retry logic
     * @param {string} content - Source content
     * @param {string} locale - Target locale
     * @param {string} filename - For logging
     * @returns {Promise<string>} - Translated content
     */
    async translate(content, locale, filename) {
        const prompt = this.buildPrompt(content, locale);
        const systemMessage = this.getSystemMessage(locale);
        const maxRetries = 10;
        const baseDelay = 1000; // 1 second

        // Build list of models to try: primary model + alternates for length issues
        const modelsToTry = [this.model, ...ALTERNATE_MODELS];

        for (const currentModel of modelsToTry) {
            for (let attempt = 1; attempt <= maxRetries; attempt++) {
                try {
                    const response = await fetch('https://api.openai.com/v1/chat/completions', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                            'Authorization': `Bearer ${this.apiKey}`
                        },
                        body: JSON.stringify({
                            model: currentModel,
                            messages: [
                                { role: 'system', content: systemMessage },
                                { role: 'user', content: prompt }
                            ],
                            max_completion_tokens: 16000
                        })
                    });

                    if (!response.ok) {
                        const errorText = await response.text();
                        throw new Error(`OpenAI API error: ${response.status} ${errorText}`);
                    }

                    const data = await response.json();
                    const translation = data.choices?.[0]?.message?.content?.trim() || '';
                    const finishReason = data.choices?.[0]?.finish_reason;

                    if (!translation) {
                        // If hit length limit, try next model instead of retrying same one
                        if (finishReason === 'length') {
                            const nextModelIndex = modelsToTry.indexOf(currentModel) + 1;
                            if (nextModelIndex < modelsToTry.length) {
                                console.warn(`  [length limit] ${filename}: Output truncated with ${currentModel}, trying ${modelsToTry[nextModelIndex]}...`);
                                break; // Break inner retry loop, continue to next model
                            }
                            throw new Error(`API returned empty translation. Finish reason: ${finishReason} (exhausted all models)`);
                        }
                        throw new Error(`API returned empty translation. Finish reason: ${finishReason}`);
                    }

                    const modelSuffix = currentModel !== this.model ? ` [${currentModel}]` : '';
                    console.log(`  [translated] ${filename} (${data.usage?.total_tokens || 0} tokens)${modelSuffix}`);

                    return translation;
                } catch (error) {
                    const isLastAttempt = attempt === maxRetries;
                    const isLengthError = error.message.includes('Finish reason: length');

                    // Always log the error
                    console.error(`  [error] ${filename}: ${error.message}`);

                    // If length error, break to try next model
                    if (isLengthError) {
                        break;
                    }

                    if (isLastAttempt) {
                        // Continue to next model if available
                        const nextModelIndex = modelsToTry.indexOf(currentModel) + 1;
                        if (nextModelIndex < modelsToTry.length) {
                            console.warn(`  [fallback] Trying ${modelsToTry[nextModelIndex]}...`);
                            break;
                        }
                        return null;
                    }

                    // Exponential backoff: 1s, 2s, 4s, 8s, 16s, 32s, 64s, 128s, 256s, 512s
                    const delay = baseDelay * Math.pow(2, attempt - 1);
                    console.warn(`  [retry ${attempt}/${maxRetries}] Retrying in ${delay}ms...`);
                    await this.sleep(delay);
                }
            }
        }

        return null;
    }

    /**
     * Validate translation by checking inline-code counts
     * @param {string} source - Source content
     * @param {string} translation - Translated content
     * @returns {Object} - Validation result
     */
    validateTranslation(source, translation) {
        const sourceCounts = countInlineCode(source);
        const translationCounts = countInlineCode(translation);

        const valid = sourceCounts.start === translationCounts.start &&
                      sourceCounts.end === translationCounts.end;

        return {
            valid,
            sourceCounts,
            translationCounts
        };
    }
}

/**
 * Process translations with concurrent workers
 * @param {Array} tasks - Array of {guideId, locale, filename, sourceHash} objects
 * @param {TranslationClient} client - Translation client
 * @param {Object} options - Options
 * @returns {Promise<Object>} - Results summary
 */
async function processTranslations(tasks, client, options = {}) {
    const { concurrency = 5, dryRun = false, cache = {} } = options;
    const results = { success: 0, failed: 0, skipped: 0, validationErrors: [] };
    let currentIndex = 0;

    const next = () => {
        if (currentIndex < tasks.length) {
            const task = tasks[currentIndex];
            currentIndex++;
            return task;
        }
        return null;
    };

    const worker = async () => {
        while (true) {
            const task = next();
            if (!task) break;

            const { guideId, locale, filename, sourceHash } = task;
            const cacheKey = getCacheKey(guideId, locale, filename);

            try {
                const source = getSourceContent(guideId, filename);

                // Skip empty or very small files (likely placeholders)
                if (source.trim().length < 10) {
                    console.log(`  [skipped] ${guideId}/${locale}/${filename} (empty or too small)`);
                    results.skipped++;
                    // Still update cache to avoid retrying
                    updateCache(cache, cacheKey, sourceHash);
                    saveCache(cache);
                    continue;
                }

                if (dryRun) {
                    console.log(`  [dry-run] Would translate ${guideId}/${locale}/${filename}`);
                    results.skipped++;
                    continue;
                }

                const translation = await client.translate(source, locale, `${guideId}/${locale}/${filename}`);

                if (!translation) {
                    results.failed++;
                    continue;
                }

                // Validate translation
                const validation = client.validateTranslation(source, translation);
                if (!validation.valid) {
                    console.warn(`  [warning] Inline-code mismatch in ${guideId}/${locale}/${filename}`);
                    results.validationErrors.push({
                        guideId, locale, filename,
                        expected: validation.sourceCounts,
                        actual: validation.translationCounts
                    });
                }

                // Save translation
                saveTranslation(guideId, locale, filename, translation);

                // Update cache with source hash and save immediately
                updateCache(cache, cacheKey, sourceHash);
                saveCache(cache);

                results.success++;
            } catch (error) {
                console.error(`  [error] ${guideId}/${locale}/${filename}: ${error.message || error}`);
                console.error(error.stack || error);
                results.failed++;
            }
        }
    };

    // Start concurrent workers
    const workers = [];
    for (let i = 0; i < concurrency; i++) {
        workers.push(worker());
    }

    await Promise.all(workers);

    return results;
}

/**
 * Check if a guide has locale structure
 * @param {string} guideId - Guide ID
 * @returns {boolean}
 */
function hasLocaleStructure(guideId) {
    const defaultPath = path.join(GUIDES_DIR, guideId, 'items', defaultLocale);
    return fs.existsSync(defaultPath) && fs.statSync(defaultPath).isDirectory();
}

/**
 * Load translations.json file
 * @returns {Object} - Translations object
 */
function loadUITranslations() {
    if (!fs.existsSync(TRANSLATIONS_FILE)) {
        return {};
    }
    return JSON.parse(fs.readFileSync(TRANSLATIONS_FILE, 'utf8'));
}

/**
 * Save translations.json file
 * @param {Object} translations - Translations object
 */
function saveUITranslations(translations) {
    fs.writeFileSync(TRANSLATIONS_FILE, JSON.stringify(translations, null, 2), 'utf8');
}

/**
 * Translate UI strings for a locale
 * @param {string} locale - Target locale
 * @param {Object} sourceStrings - Object of key: value pairs to translate
 * @param {TranslationClient} client - Translation client
 * @returns {Promise<Object|null>} - Translated strings or null on failure
 */
async function translateUIStrings(locale, sourceStrings, client) {
    const localeInfo = locales[locale];
    const localeName = localeInfo ? localeInfo.nativeName : locale;

    const systemMessage = `You are an expert translator. Translate UI strings from English to ${localeName} (${locale}).
Return ONLY a valid JSON object with the same keys but translated values.
Do not include any explanation or markdown formatting - just the raw JSON.`;

    const prompt = `Translate these UI strings to ${localeName}:

${JSON.stringify(sourceStrings, null, 2)}

Return ONLY a valid JSON object with the translated values. No explanations.`;

    const maxRetries = 5;
    const baseDelay = 1000;

    // Build list of models to try: primary model + alternates for length issues
    const modelsToTry = [client.model, ...ALTERNATE_MODELS];

    for (const currentModel of modelsToTry) {
        for (let attempt = 1; attempt <= maxRetries; attempt++) {
            try {
                const response = await fetch('https://api.openai.com/v1/chat/completions', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${client.apiKey}`
                    },
                    body: JSON.stringify({
                        model: currentModel,
                        messages: [
                            { role: 'system', content: systemMessage },
                            { role: 'user', content: prompt }
                        ],
                        max_completion_tokens: 16000
                    })
                });

                if (!response.ok) {
                    const errorText = await response.text();
                    throw new Error(`OpenAI API error: ${response.status} ${errorText}`);
                }

                const data = await response.json();
                let content = data.choices?.[0]?.message?.content?.trim() || '';
                const finishReason = data.choices?.[0]?.finish_reason;

                // Strip markdown code block if present
                if (content.startsWith('```json')) {
                    content = content.slice(7);
                } else if (content.startsWith('```')) {
                    content = content.slice(3);
                }
                if (content.endsWith('```')) {
                    content = content.slice(0, -3);
                }
                content = content.trim();

                if (!content) {
                    // If hit length limit, try next model instead of retrying same one
                    if (finishReason === 'length') {
                        const nextModelIndex = modelsToTry.indexOf(currentModel) + 1;
                        if (nextModelIndex < modelsToTry.length) {
                            console.warn(`  [length limit] UI strings for ${locale}: Output truncated with ${currentModel}, trying ${modelsToTry[nextModelIndex]}...`);
                            break; // Break inner retry loop, continue to next model
                        }
                        const errorDetails = {
                            finishReason,
                            usage: data.usage,
                            model: data.model
                        };
                        throw new Error(`API returned empty translation. Details: ${JSON.stringify(errorDetails)} (exhausted all models)`);
                    }
                    const errorDetails = {
                        finishReason,
                        usage: data.usage,
                        model: data.model
                    };
                    throw new Error(`API returned empty translation. Details: ${JSON.stringify(errorDetails)}`);
                }

                let translated;
                try {
                    translated = JSON.parse(content);
                } catch (parseError) {
                    throw new Error(`Failed to parse API response as JSON: ${parseError.message}. Content: ${content.substring(0, 200)}...`);
                }

                const modelSuffix = currentModel !== client.model ? ` [${currentModel}]` : '';
                console.log(`  [translated] UI strings for ${locale} (${data.usage?.total_tokens || 0} tokens)${modelSuffix}`);
                return translated;
            } catch (error) {
                const isLastAttempt = attempt === maxRetries;
                const isLengthError = error.message.includes('Finish reason: length');

                // Always log the error
                console.error(`  [error] UI strings for ${locale}: ${error.message}`);

                // If length error, break to try next model
                if (isLengthError) {
                    break;
                }

                if (isLastAttempt) {
                    // Continue to next model if available
                    const nextModelIndex = modelsToTry.indexOf(currentModel) + 1;
                    if (nextModelIndex < modelsToTry.length) {
                        console.warn(`  [fallback] Trying ${modelsToTry[nextModelIndex]}...`);
                        break;
                    }
                    return null;
                }

                const delay = baseDelay * Math.pow(2, attempt - 1);
                console.warn(`  [retry ${attempt}/${maxRetries}] Retrying in ${delay}ms...`);
                await new Promise(resolve => setTimeout(resolve, delay));
            }
        }
    }

    return null;
}

/**
 * Process UI translations
 * @param {TranslationClient} client - Translation client
 * @param {Object} options - Options
 * @returns {Promise<Object>} - Results summary
 */
async function processUITranslations(client, options = {}) {
    const { filterLocale, dryRun = false, force = false } = options;
    const results = { success: 0, failed: 0, skipped: 0 };

    const uiCache = loadUITranslationCache();
    const missingUITranslations = getMissingUITranslations({ cache: uiCache, force });
    const translations = loadUITranslations();
    const sourceStrings = translations[defaultLocale] || {};

    for (const [locale, { missing, stale }] of Object.entries(missingUITranslations)) {
        if (filterLocale && locale !== filterLocale) continue;

        const localeInfo = locales[locale];
        const localeName = localeInfo ? localeInfo.nativeName : locale;

        // Combine missing and stale keys
        const allKeys = [...new Set([...missing, ...stale])];
        if (allKeys.length === 0) continue;

        // Build object of strings to translate
        const toTranslate = {};
        for (const key of allKeys) {
            toTranslate[key] = sourceStrings[key];
        }

        if (dryRun) {
            const parts = [];
            if (missing.length > 0) parts.push(`${missing.length} missing`);
            if (stale.length > 0) parts.push(`${stale.length} stale`);
            console.log(`  [dry-run] Would translate ${allKeys.length} UI string(s) for ${locale} (${localeName}) [${parts.join(', ')}]`);
            results.skipped++;
            continue;
        }

        const parts = [];
        if (missing.length > 0) parts.push(`${missing.length} missing`);
        if (stale.length > 0) parts.push(`${stale.length} stale`);
        console.log(`  Translating ${allKeys.length} UI string(s) for ${locale} (${localeName}) [${parts.join(', ')}]...`);
        const translated = await translateUIStrings(locale, toTranslate, client);

        if (translated) {
            // Merge with existing translations for this locale
            if (!translations[locale]) {
                translations[locale] = {};
            }
            Object.assign(translations[locale], translated);
            saveUITranslations(translations);

            // Update cache with source hashes for translated keys
            for (const key of Object.keys(translated)) {
                const sourceValue = sourceStrings[key];
                if (sourceValue !== undefined) {
                    const sourceHash = hashContent(sourceValue);
                    const cacheKey = getUITranslationCacheKey(locale, key);
                    uiCache[cacheKey] = sourceHash;
                }
            }
            saveUITranslationCache(uiCache);

            results.success++;
        } else {
            results.failed++;
        }
    }

    return results;
}

/**
 * Translate meta.json structure for a locale
 * @param {string} guideId - Guide ID
 * @param {string} locale - Target locale
 * @param {Object} meta - Source meta.json object
 * @param {TranslationClient} client - Translation client
 * @returns {Promise<Object|null>} - Translated meta object or null on failure
 */
async function translateMetaJson(guideId, locale, meta, client) {
    const localeInfo = locales[locale];
    const localeName = localeInfo ? localeInfo.nativeName : locale;

    // Build object to translate
    const toTranslate = {
        guideName: meta.name
    };

    if (meta.pageHeader) {
        toTranslate.pageHeader = meta.pageHeader;
    }

    // Add all item names and subCats
    meta.itemsOrdered.forEach((item, idx) => {
        toTranslate[`item_${idx}_name`] = item.name;
        toTranslate[`item_${idx}_subCat`] = item.subCat;
    });

    const systemMessage = `You are an expert translator. Translate metadata from English to ${localeName} (${locale}).
Return ONLY a valid JSON object with the same keys but translated values.
Do not include any explanation or markdown formatting - just the raw JSON.`;

    const prompt = `Translate these guide metadata strings to ${localeName}:

${JSON.stringify(toTranslate, null, 2)}

Return ONLY a valid JSON object with the translated values. No explanations.`;

    const maxRetries = 5;
    const baseDelay = 1000;

    // Build list of models to try: primary model + alternates for length issues
    const modelsToTry = [client.model, ...ALTERNATE_MODELS];

    for (const currentModel of modelsToTry) {
        for (let attempt = 1; attempt <= maxRetries; attempt++) {
            try {
                const response = await fetch('https://api.openai.com/v1/chat/completions', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${client.apiKey}`
                    },
                    body: JSON.stringify({
                        model: currentModel,
                        messages: [
                            { role: 'system', content: systemMessage },
                            { role: 'user', content: prompt }
                        ],
                        max_completion_tokens: 16000
                    })
                });

                if (!response.ok) {
                    const errorText = await response.text();
                    throw new Error(`OpenAI API error: ${response.status} ${errorText}`);
                }

                const data = await response.json();
                let content = data.choices?.[0]?.message?.content?.trim() || '';
                const finishReason = data.choices?.[0]?.finish_reason;

                // Strip markdown code block if present
                if (content.startsWith('```json')) {
                    content = content.slice(7);
                } else if (content.startsWith('```')) {
                    content = content.slice(3);
                }
                if (content.endsWith('```')) {
                    content = content.slice(0, -3);
                }
                content = content.trim();

                if (!content) {
                    // If hit length limit, try next model instead of retrying same one
                    if (finishReason === 'length') {
                        const nextModelIndex = modelsToTry.indexOf(currentModel) + 1;
                        if (nextModelIndex < modelsToTry.length) {
                            console.warn(`  [length limit] meta.json for ${guideId}/${locale}: Output truncated with ${currentModel}, trying ${modelsToTry[nextModelIndex]}...`);
                            break; // Break inner retry loop, continue to next model
                        }
                        const errorDetails = {
                            finishReason,
                            usage: data.usage,
                            model: data.model
                        };
                        throw new Error(`API returned empty translation. Details: ${JSON.stringify(errorDetails)} (exhausted all models)`);
                    }
                    const errorDetails = {
                        finishReason,
                        usage: data.usage,
                        model: data.model
                    };
                    throw new Error(`API returned empty translation. Details: ${JSON.stringify(errorDetails)}`);
                }

                let translated;
                try {
                    translated = JSON.parse(content);
                } catch (parseError) {
                    throw new Error(`Failed to parse API response as JSON: ${parseError.message}. Content: ${content.substring(0, 200)}...`);
                }

                // Build translated meta object
                const translatedMeta = {
                    ...meta,
                    name: translated.guideName
                };

                if (meta.pageHeader && translated.pageHeader) {
                    translatedMeta.pageHeader = translated.pageHeader;
                }

                // Update item names and subCats
                translatedMeta.itemsOrdered = meta.itemsOrdered.map((item, idx) => ({
                    ...item,
                    name: translated[`item_${idx}_name`] || item.name,
                    subCat: translated[`item_${idx}_subCat`] || item.subCat
                }));

                const modelSuffix = currentModel !== client.model ? ` [${currentModel}]` : '';
                console.log(`  [translated] meta.json for ${guideId}/${locale} (${data.usage?.total_tokens || 0} tokens)${modelSuffix}`);
                return translatedMeta;
            } catch (error) {
                const isLastAttempt = attempt === maxRetries;
                const isLengthError = error.message.includes('Finish reason: length');

                // Always log the error
                console.error(`  [error] meta.json for ${guideId}/${locale}: ${error.message}`);

                // If length error, break to try next model
                if (isLengthError) {
                    break;
                }

                if (isLastAttempt) {
                    // Continue to next model if available
                    const nextModelIndex = modelsToTry.indexOf(currentModel) + 1;
                    if (nextModelIndex < modelsToTry.length) {
                        console.warn(`  [fallback] Trying ${modelsToTry[nextModelIndex]}...`);
                        break;
                    }
                    return null;
                }

                const delay = baseDelay * Math.pow(2, attempt - 1);
                console.warn(`  [retry ${attempt}/${maxRetries}] Retrying in ${delay}ms...`);
                await new Promise(resolve => setTimeout(resolve, delay));
            }
        }
    }

    return null;
}

/**
 * Check which meta.json files need translation using cache
 * @param {Object} cache - Current cache
 * @param {Object} options - Filter options
 * @returns {Array} - Array of {guideId, locale, sourceHash} objects
 */
function getMetaJsonTranslationNeeds(cache, options = {}) {
    const { filterLocale, filterGuide, force = false } = options;
    const guides = getGuideDirectories();
    const nonDefaultLocales = Object.keys(locales).filter(l => l !== defaultLocale);
    const needed = [];

    for (const guideId of guides) {
        if (filterGuide && guideId !== filterGuide) continue;

        const metaPath = path.join(GUIDES_DIR, guideId, 'meta.json');
        if (!fs.existsSync(metaPath)) continue;

        const metaContent = fs.readFileSync(metaPath, 'utf8');
        const sourceHash = hashContent(metaContent);

        for (const locale of nonDefaultLocales) {
            if (filterLocale && locale !== filterLocale) continue;

            const cacheKey = getCacheKey(guideId, locale, 'meta.json');

            // Check if translation is needed using cache
            if (force || !isCached(cache, cacheKey, sourceHash)) {
                needed.push({ guideId, locale, sourceHash });
            }
        }
    }

    return needed;
}

/**
 * Process meta.json translations with concurrency
 * @param {TranslationClient} client - Translation client
 * @param {Object} options - Options
 * @returns {Promise<Object>} - Results summary
 */
async function processMetaJsonTranslations(client, options = {}) {
    const { filterLocale, filterGuide, dryRun = false, cache = {}, concurrency = 5 } = options;
    const results = { success: 0, failed: 0, skipped: 0 };

    const needed = getMetaJsonTranslationNeeds(cache, { filterLocale, filterGuide, force: options.force });

    if (needed.length === 0) {
        return results;
    }

    let currentIndex = 0;

    const next = () => {
        if (currentIndex < needed.length) {
            const task = needed[currentIndex];
            currentIndex++;
            return task;
        }
        return null;
    };

    const worker = async () => {
        while (true) {
            const task = next();
            if (!task) break;

            const { guideId, locale, sourceHash } = task;
            const metaPath = path.join(GUIDES_DIR, guideId, 'meta.json');
            const localeInfo = locales[locale];
            const localeName = localeInfo ? localeInfo.nativeName : locale;
            const cacheKey = getCacheKey(guideId, locale, 'meta.json');

            try {
                if (dryRun) {
                    console.log(`  [dry-run] Would translate meta.json for ${guideId} to ${locale} (${localeName})`);
                    results.skipped++;
                    continue;
                }

                const meta = JSON.parse(fs.readFileSync(metaPath, 'utf8'));
                const translatedMeta = await translateMetaJson(guideId, locale, meta, client);

                if (!translatedMeta) {
                    results.failed++;
                    continue;
                }

                // Create meta_translated directory if it doesn't exist
                const metaTranslatedDir = path.join(GUIDES_DIR, guideId, 'meta_translated');
                if (!fs.existsSync(metaTranslatedDir)) {
                    fs.mkdirSync(metaTranslatedDir, { recursive: true });
                }

                const outputPath = path.join(metaTranslatedDir, `meta_${locale}.json`);
                fs.writeFileSync(outputPath, JSON.stringify(translatedMeta, null, 2), 'utf8');

                // Update cache with source hash and save immediately
                updateCache(cache, cacheKey, sourceHash);
                saveCache(cache);

                results.success++;
            } catch (error) {
                console.error(`  [error] meta.json for ${guideId}/${locale}: ${error.message || error}`);
                console.error(error.stack || error);
                results.failed++;
            }
        }
    };

    // Start concurrent workers
    const workers = [];
    for (let i = 0; i < concurrency; i++) {
        workers.push(worker());
    }

    await Promise.all(workers);

    return results;
}

/**
 * Build list of translation tasks using cache to detect changes
 * @param {Object} cache - Current cache
 * @param {Object} options - Filter options
 * @returns {Array} - Array of tasks
 */
function buildTaskList(cache, options = {}) {
    const { filterLocale, filterGuide, force = false } = options;
    const tasks = [];
    const nonDefaultLocales = Object.keys(locales).filter(l => l !== defaultLocale);
    const guides = getGuideDirectories();

    for (const guideId of guides) {
        if (filterGuide && guideId !== filterGuide) continue;
        if (!hasLocaleStructure(guideId)) continue;

        const sourceFiles = getDefaultLocaleFiles(guideId);
        if (sourceFiles.length === 0) continue;

        for (const filename of sourceFiles) {
            const source = getSourceContent(guideId, filename);

            // Skip empty or very small files (less than 10 characters)
            if (source.trim().length < 10) {
                continue;
            }

            const sourceHash = hashContent(source);

            for (const locale of nonDefaultLocales) {
                if (filterLocale && locale !== filterLocale) continue;

                const cacheKey = getCacheKey(guideId, locale, filename);

                // Check if translation is needed using cache
                if (force || !isCached(cache, cacheKey, sourceHash)) {
                    tasks.push({ guideId, locale, filename, sourceHash });
                }
            }
        }
    }

    return tasks;
}

function parseArgs() {
    const args = process.argv.slice(2);
    const options = {
        locale: null,
        guide: null,
        concurrency: 20,
        dryRun: false,
        force: false
    };

    for (let i = 0; i < args.length; i++) {
        switch (args[i]) {
            case '--locale':
                options.locale = args[++i];
                break;
            case '--guide':
                options.guide = args[++i];
                break;
            case '--concurrency':
                options.concurrency = parseInt(args[++i], 10);
                break;
            case '--dry-run':
                options.dryRun = true;
                break;
            case '--force':
                options.force = true;
                break;
            case '--help':
                console.log(`
Usage: node src/translate-with-gpt.js [options]

Options:
  --locale <locale>      Only translate for this locale (e.g., fr_fr, de_de)
  --guide <guideId>      Only translate for this guide (e.g., api, installation)
  --concurrency <n>      Number of concurrent API calls (default: 20)
  --dry-run              Show what would be translated without making API calls
  --force                Force re-translation even if snapshot says up-to-date
  --help                 Show this help message

Environment variables:
  OPENAI_API_KEY         Required. Your OpenAI API key.
  OPENAI_MODEL           Optional. Model to use (default: gpt-5-mini).

Examples:
  node src/translate-with-gpt.js
  node src/translate-with-gpt.js --locale fr_fr
  node src/translate-with-gpt.js --guide api --locale de_de
  node src/translate-with-gpt.js --dry-run
  node src/translate-with-gpt.js --force --locale hr_hr
`);
                process.exit(0);
        }
    }

    return options;
}

async function main() {
    const options = parseArgs();

    if (!OPENAI_API_KEY) {
        console.error('Error: OPENAI_API_KEY environment variable is required');
        process.exit(1);
    }

    console.log(`Translation Script`);
    console.log(`Model: ${OPENAI_MODEL}`);
    console.log(`Concurrency: ${options.concurrency}`);
    if (options.locale) console.log(`Filter locale: ${options.locale}`);
    if (options.guide) console.log(`Filter guide: ${options.guide}`);
    if (options.dryRun) console.log(`Mode: DRY RUN`);
    if (options.force) console.log(`Mode: FORCE (ignoring cache)`);
    console.log('');

    const client = new TranslationClient();
    let hasFailures = false;

    // Process UI translations first (translations.json)
    if (!options.guide) {
        console.log('--- UI Translations (translations.json) ---\n');
        const uiCache = loadUITranslationCache();
        const missingUI = getMissingUITranslations({ cache: uiCache, force: options.force });
        const uiLocales = Object.keys(missingUI).filter(l => !options.locale || l === options.locale);

        if (uiLocales.length > 0) {
            console.log(`Found ${uiLocales.length} locale(s) needing UI translations.`);
            let totalMissing = 0;
            let totalStale = 0;
            for (const locale of uiLocales.sort()) {
                const localeInfo = locales[locale];
                const name = localeInfo ? localeInfo.nativeName : locale;
                const { missing, stale } = missingUI[locale];
                const parts = [];
                if (missing.length > 0) parts.push(`${missing.length} missing`);
                if (stale.length > 0) parts.push(`${stale.length} stale`);
                console.log(`  ${locale} (${name}): ${parts.join(', ')}`);
                totalMissing += missing.length;
                totalStale += stale.length;
            }
            if (totalStale > 0) {
                console.log(`\n  (${totalStale} stale translations will be re-translated due to source changes)`);
            }
            console.log('');

            const uiResults = await processUITranslations(client, {
                filterLocale: options.locale,
                dryRun: options.dryRun,
                force: options.force
            });

            console.log('');
            console.log('UI Translation Results:');
            console.log(`  Success: ${uiResults.success} locale(s)`);
            console.log(`  Failed: ${uiResults.failed} locale(s)`);
            if (uiResults.skipped > 0) {
                console.log(`  Skipped: ${uiResults.skipped} locale(s)`);
            }

            if (uiResults.failed > 0) {
                hasFailures = true;
            }
        } else {
            console.log('All UI translations are up-to-date.');
        }
        console.log('');
    }

    // Load cache (used for both meta.json and guide files)
    console.log('Loading translation cache...');
    const cache = loadCache();
    const cacheEntries = Object.keys(cache).length;
    console.log(`Cache has ${cacheEntries} entries.`);
    console.log('');

    // Process meta.json translations
    console.log('--- Meta.json Translations ---\n');
    const metaNeeded = getMetaJsonTranslationNeeds(cache, {
        filterLocale: options.locale,
        filterGuide: options.guide,
        force: options.force
    });

    if (metaNeeded.length > 0) {
        console.log(`Found ${metaNeeded.length} meta.json file(s) needing translation.`);

        // Group by locale for summary
        const byLocale = {};
        for (const { locale } of metaNeeded) {
            byLocale[locale] = (byLocale[locale] || 0) + 1;
        }
        for (const [locale, count] of Object.entries(byLocale).sort()) {
            const localeInfo = locales[locale];
            const name = localeInfo ? localeInfo.nativeName : locale;
            console.log(`  ${locale} (${name}): ${count} guide(s)`);
        }
        console.log('');

        const metaResults = await processMetaJsonTranslations(client, {
            filterLocale: options.locale,
            filterGuide: options.guide,
            dryRun: options.dryRun,
            force: options.force,
            concurrency: options.concurrency,
            cache
        });

        console.log('');
        console.log('Meta.json Translation Results:');
        console.log(`  Success: ${metaResults.success} file(s)`);
        console.log(`  Failed: ${metaResults.failed} file(s)`);
        if (metaResults.skipped > 0) {
            console.log(`  Skipped: ${metaResults.skipped} file(s)`);
        }

        if (metaResults.failed > 0) {
            hasFailures = true;
        }
    } else {
        console.log('All meta.json files are up-to-date (based on cache).');
    }
    console.log('');

    // Process guide file translations
    console.log('--- Guide File Translations ---\n');

    // Build task list using cache
    console.log('Scanning for guide file translations needed...');
    const tasks = buildTaskList(cache, {
        filterLocale: options.locale,
        filterGuide: options.guide,
        force: options.force
    });

    if (tasks.length === 0) {
        console.log('No guide file translations needed matching the criteria.');
        console.log('(All translations are up-to-date based on cache)');
        if (hasFailures) {
            process.exit(1);
        }
        process.exit(0);
    }

    console.log(`Found ${tasks.length} file(s) to translate.`);
    console.log('');

    // Group by locale for summary
    const byLocale = {};
    for (const task of tasks) {
        byLocale[task.locale] = (byLocale[task.locale] || 0) + 1;
    }
    for (const [locale, count] of Object.entries(byLocale).sort()) {
        const localeInfo = locales[locale];
        const name = localeInfo ? localeInfo.nativeName : locale;
        console.log(`  ${locale} (${name}): ${count} file(s)`);
    }
    console.log('');

    // Process translations
    console.log('Starting translations...');
    console.log('');

    const results = await processTranslations(tasks, client, {
        concurrency: options.concurrency,
        dryRun: options.dryRun,
        cache
    });

    // Print summary
    console.log('');
    console.log('--- Guide File Summary ---');
    console.log(`Success: ${results.success}`);
    console.log(`Failed: ${results.failed}`);
    if (results.skipped > 0) {
        console.log(`Skipped (dry-run): ${results.skipped}`);
    }

    if (results.validationErrors.length > 0) {
        console.log('');
        console.log('Validation warnings (inline-code mismatches):');
        for (const err of results.validationErrors) {
            console.log(`  ${err.guideId}/${err.locale}/${err.filename}`);
            console.log(`    Expected: ${err.expected.start} start, ${err.expected.end} end`);
            console.log(`    Actual: ${err.actual.start} start, ${err.actual.end} end`);
        }
    }

    if (results.failed > 0 || hasFailures) {
        process.exit(1);
    }
}

// Run main only if called directly
if (require.main === module) {
    main().catch(error => {
        console.error('Fatal error:', error);
        process.exit(1);
    });
}

module.exports = {
    TranslationClient,
    processTranslations,
    buildTaskList
};
