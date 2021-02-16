const startTime = Date.now();

const fs = require('fs');
const path = require('path');
const marked = require('marked');
const handlebars = require('handlebars');
const shortid = require('shortid');
const {htmlToText} = require('html-to-text');

const CONTENT_DIR = path.join(__dirname, 'content');
const CATEGORIES_DIR = path.join(__dirname, 'content', 'categories');
const GUIDES_DIR = path.join(__dirname, 'content', 'guides');
const TEMPLATE_DIR = path.join(__dirname, 'templates');
const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');

/**
 * @typedef {Object} IndexEntry
 * @property {string} url - URL to the index entry.
 * @property {string} aroundText - Text around the searched word.
 */

/**
 * @type {Object.<string, Array.<IndexEntry>>}
 * A map of phrase to relevant results.
 */
const index = {};

/**
 * @typedef {Object} Content
 * @property {string} html
 * @property {string} title
 * @property {string} urlId
 * @property {string} fullUrl
 */

/**
 * @typedef {Object} Meta
 * @property {string} name
 * @property {Array.<string>} itemsOrdered
 */

/**
 * @param {Content} content
 */
function addContentToIndex(content) {
    const text = htmlToText(content.html, {
        wordwrap: 9001,
        tags: {
            'h1': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h2': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h3': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h4': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h5': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'h6': {format: 'heading', options: {leadingLineBreaks: 0, trailingLineBreaks: 0, uppercase: false}},
            'hr': { format: 'horizontalLine', options: { leadingLineBreaks: 0, length: undefined, trailingLineBreaks: 0 } },
            'p': { format: 'paragraph', options: { leadingLineBreaks: 0, trailingLineBreaks: 0 } },
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
        const wordPosition = text.indexOf(word);
        const wordClean = word.replace(/(,|\\.)/g, '').toLowerCase();
        if (index[wordClean] === undefined) {
            index[wordClean] = [];
        }

        index[wordClean].push({
            url: content.fullUrl,
            // TODO contain all instances of the word
            aroundText: text.substring(wordPosition - 50, wordPosition) + '...' + text.substring(wordPosition, wordPosition + 50)
        });
    }
}

const content = {}; // full path like "categories/custom-styles/custom-css.md" to
fs.readdirSync(CATEGORIES_DIR).forEach(function (category) {
    /** @type {Meta} **/
    const meta = JSON.parse(fs.readFileSync(path.join(CATEGORIES_DIR, category, 'meta.json'), 'utf8'));
    meta.itemsOrdered.forEach((item) => {
        const title = item.replace('\.md', '');
        const urlId = title.toLowerCase().replace(/ /g, '-') + '.html';
        const fullUrl = 'https://docs.fastcomments.com/' + urlId;

        const fileContent = fs.readFileSync(path.join(CATEGORIES_DIR, category, 'items', item), 'utf8');
        const html = marked(fileContent);

        const entry = {
            html,
            title,
            urlId,
            fullUrl,
        };

        addContentToIndex(entry);

        content[`categories/${category}/${item}`] = entry;
    });
});

const indexRoot = {};
for (const word in index) {
    const id = shortid.generate();
    indexRoot[word] = id;
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, `index-${id}.json`), JSON.stringify(index[word]), 'utf8');
}
fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'index.json'), JSON.stringify(indexRoot), 'utf8');

// fs.readdirSync(TEMPLATE_DIR).forEach(function (item) {
//     if (item === 'index.html') {
//         fs.writeFileSync(path.join(STATIC_GENERATED_DIR, item), getCompiledTemplate(item, {
//             posts: posts,
//             footerYears: footerYears
//         }), 'utf8');
//     } else if (item === 'page.html') {
//         posts.forEach(function (page) {
//             const html = getCompiledTemplate(item, {
//                 page: page,
//                 posts: posts,
//                 footerYears: footerYears
//             });
//             fs.writeFileSync(path.join(STATIC_GENERATED_DIR, page.urlId), html, 'utf8');
//         });
//     }
// });
//
// function getCompiledTemplate(templateName, data) {
//     return handlebars.compile(fs.readFileSync(path.join(TEMPLATE_DIR, templateName), 'utf8'))(data);
// }

console.log(`Execution Time: ${Date.now() - startTime}ms`);
