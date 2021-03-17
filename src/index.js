const fs = require('fs');
const path = require('path');
const shortid = require('shortid');
const {htmlToText} = require('html-to-text');

const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');

/**
 * @typedef {Object} IndexEntry
 * @property {string} url - URL to the index entry.
 * @property {string} title - Title of the target entry.
 * @property {string} aroundText - Text around the searched word.
 */

/**
 * @typedef {Object.<string, Array.<IndexEntry>>} Index
 */

/**
 * @param {Content} content
 * @param {Index} index
 */
function addContentToIndex(content, index) {
    const text = htmlToText(content.html, {
        wordwrap: 9001,
        tags: {
            'h1': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h2': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h3': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h4': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h5': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h6': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'hr': {format: 'horizontalLine', options: {leadingLineBreaks: 0, length: undefined, trailingLineBreaks: 0}},
            'p': {format: 'paragraph', options: {leadingLineBreaks: 0, trailingLineBreaks: 0}},
        }
    }).replace(/\n\n/g, '...');
    const words = text.replace(/\n/g, '').split(' ');
    for (const word of words) {
        if (word.length < 4) {
            continue;
        }
        if (word.includes('...')) {
            continue;
        }
        if (word.includes('<') || word.includes('>')) {
            continue;
        }
        if (word.includes('[') || word.includes(']')) {
            continue;
        }
        const wordPosition = text.indexOf(word);
        const wordClean = word.replace(/[,\\.<>"]/g, '').toLowerCase();
        if (index[wordClean] === undefined) {
            index[wordClean] = [];
        }

        index[wordClean].push({
            url: content.fullUrl,
            title: content.title,
            // TODO contain all instances of the word
            aroundText: text.substring(wordPosition - 50, wordPosition + 50)
        });
    }
}

/**
 *
 * @param {Index} index
 * @return {string} index root JSON
 */
function persistIndex(index) {
    const indexRoot = {};
    for (const word in index) {
        const id = shortid.generate();
        indexRoot[word] = id;
        fs.writeFileSync(path.join(STATIC_GENERATED_DIR, `index-${id}.json`), JSON.stringify(index[word]), 'utf8');
    }
    const indexRootJSON = JSON.stringify(indexRoot);
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'index.json'), indexRootJSON, 'utf8'); // Currently, this is only written to disk for debugging.

    return indexRootJSON;
}

module.exports = {addContentToIndex, persistIndex};
