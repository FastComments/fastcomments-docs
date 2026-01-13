#!/usr/bin/env node

/**
 * Snapshot management for translation tracking.
 * Stores MD5 hashes of source files to detect when re-translation is needed.
 */

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const SNAPSHOT_FILE = path.join(__dirname, 'translation-snapshot.json');

/**
 * Load the current snapshot from disk
 * @returns {Object} - Snapshot object { filePath: { hash, translatedLocales: [...] } }
 */
function loadSnapshot() {
    if (!fs.existsSync(SNAPSHOT_FILE)) {
        return {};
    }
    try {
        return JSON.parse(fs.readFileSync(SNAPSHOT_FILE, 'utf8'));
    } catch (e) {
        console.warn(`Failed to load snapshot: ${e.message}`);
        return {};
    }
}

/**
 * Save snapshot to disk
 * @param {Object} snapshot - Snapshot object
 */
function saveSnapshot(snapshot) {
    fs.writeFileSync(SNAPSHOT_FILE, JSON.stringify(snapshot, null, 2), 'utf8');
}

/**
 * Calculate MD5 hash of file content
 * @param {string} filePath - Path to file
 * @returns {string} - MD5 hash
 */
function hashFile(filePath) {
    const content = fs.readFileSync(filePath, 'utf8');
    return crypto.createHash('md5').update(content).digest('hex');
}

/**
 * Calculate MD5 hash of string content
 * @param {string} content - Content to hash
 * @returns {string} - MD5 hash
 */
function hashContent(content) {
    return crypto.createHash('md5').update(content).digest('hex');
}

/**
 * Get the relative path key for a source file
 * @param {string} guideId - Guide ID
 * @param {string} filename - Filename
 * @returns {string} - Relative path key
 */
function getSourceKey(guideId, filename) {
    return `${guideId}/${filename}`;
}

/**
 * Check if a translation is needed for a given source file and locale
 * @param {Object} snapshot - Current snapshot
 * @param {string} guideId - Guide ID
 * @param {string} filename - Filename
 * @param {string} locale - Target locale
 * @param {string} currentHash - Current hash of source file
 * @returns {boolean} - True if translation is needed
 */
function needsTranslation(snapshot, guideId, filename, locale, currentHash) {
    const key = getSourceKey(guideId, filename);
    const entry = snapshot[key];

    if (!entry) {
        // No snapshot entry - translation needed
        return true;
    }

    if (entry.hash !== currentHash) {
        // Source file has changed - translation needed
        return true;
    }

    if (!entry.translatedLocales || !entry.translatedLocales.includes(locale)) {
        // Locale not yet translated - translation needed
        return true;
    }

    // Already translated and source hasn't changed
    return false;
}

/**
 * Mark a translation as complete in the snapshot
 * @param {Object} snapshot - Current snapshot
 * @param {string} guideId - Guide ID
 * @param {string} filename - Filename
 * @param {string} locale - Target locale
 * @param {string} sourceHash - Hash of source file
 */
function markTranslated(snapshot, guideId, filename, locale, sourceHash) {
    const key = getSourceKey(guideId, filename);

    if (!snapshot[key]) {
        snapshot[key] = {
            hash: sourceHash,
            translatedLocales: []
        };
    }

    // Update hash (in case it changed)
    snapshot[key].hash = sourceHash;

    // Add locale if not already present
    if (!snapshot[key].translatedLocales.includes(locale)) {
        snapshot[key].translatedLocales.push(locale);
        snapshot[key].translatedLocales.sort();
    }
}

/**
 * Update source hash and clear stale translations
 * Call this when a source file changes to invalidate translations
 * @param {Object} snapshot - Current snapshot
 * @param {string} guideId - Guide ID
 * @param {string} filename - Filename
 * @param {string} newHash - New hash of source file
 */
function updateSourceHash(snapshot, guideId, filename, newHash) {
    const key = getSourceKey(guideId, filename);
    const entry = snapshot[key];

    if (entry && entry.hash !== newHash) {
        // Source changed - clear translated locales
        snapshot[key] = {
            hash: newHash,
            translatedLocales: []
        };
    } else if (!entry) {
        snapshot[key] = {
            hash: newHash,
            translatedLocales: []
        };
    }
}

/**
 * Remove a source file from the snapshot
 * @param {Object} snapshot - Current snapshot
 * @param {string} guideId - Guide ID
 * @param {string} filename - Filename
 */
function removeSource(snapshot, guideId, filename) {
    const key = getSourceKey(guideId, filename);
    delete snapshot[key];
}

module.exports = {
    loadSnapshot,
    saveSnapshot,
    hashFile,
    hashContent,
    getSourceKey,
    needsTranslation,
    markTranslated,
    updateSourceHash,
    removeSource,
    SNAPSHOT_FILE
};
