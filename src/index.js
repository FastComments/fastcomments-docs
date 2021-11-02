const path = require('path');
const fs = require('fs');
const axios = require('axios');
const md5 = require('md5');
const {htmlToText} = require('html-to-text');

const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');
const DOC_HASH_DIR = path.join(STATIC_GENERATED_DIR, 'doc-hashes');

fs.mkdirSync(DOC_HASH_DIR, { recursive: true });

/**
 * @typedef {Object} IndexEntry
 * @property {string} url - URL to the index entry.
 * @property {string} title - Title of the target entry.
 * @property {string} aroundText - Text around the searched word.
 */

/**
 * @typedef {Object.<string, Array.<IndexEntry>>} Index
 */

const CleanDescriptionRegex = /\[.+?\]/g;

function cleanDescription(text) {
    return text.replaceAll(CleanDescriptionRegex, ' ... ')
}

/**
 * @param {Content} content
 * @param {Index} index
 */
async function addContentToIndex(content, index) {
    if (!process.env.DOCS_SEARCH_INDEX_API_KEY) {
        console.warn('No API key - not indexing.', content.urlId);
        return;
    }
    const hashPath = path.join(DOC_HASH_DIR, content.urlId + '.hash');

    const contentSanitized = cleanDescription(htmlToText(content.html, {
        wordwrap: 9001,
        tags: {
            'h1': {format: 'heading', options: {uppercase: false}},
            'h2': {format: 'heading', options: {uppercase: false}},
            'h3': {format: 'heading', options: {uppercase: false}},
            'h4': {format: 'heading', options: {uppercase: false}},
            'h5': {format: 'heading', options: {uppercase: false}},
            'h6': {format: 'heading', options: {uppercase: false}},
            'hr': {format: 'horizontalLine', options: { length: undefined}},
            'p': {format: 'paragraph', options: {}},
            'ul': {format: 'unorderedList'},
            'br': {format: 'lineBreak'},
            'a': {format: 'inline'},
        }
    }));

    const newHash = md5(content.title + contentSanitized + content.fullUrl)
    if (fs.existsSync(hashPath)) {
        const existingHash = fs.readFileSync(hashPath, 'utf8');
        if (newHash === existingHash) {
            return;
        }
    }
    await axios.post(`https://fastcomments.com/docs-search-index/${content.urlId}?API_KEY=${process.env.DOCS_SEARCH_INDEX_API_KEY}`, {
        contentHash: newHash,
        contentTitle: content.title,
        content: contentSanitized,
        url: content.fullUrl
    });
    fs.writeFileSync(hashPath, newHash, 'utf8');
}

module.exports = {addContentToIndex};
