const startTime = Date.now();

const fs = require('fs');
const path = require('path');
const marked = require('marked');
const handlebars = require('handlebars');
const shortid = require('shortid');
const {htmlToText} = require('html-to-text');
const {processDynamicContent} = require('./guide-dynamic-content-transformer');

const CONTENT_DIR = path.join(__dirname, 'content');
const CATEGORIES_DIR = path.join(__dirname, 'content', 'categories');
const GUIDES_DIR = path.join(__dirname, 'content', 'guides');
const TEMPLATE_DIR = path.join(__dirname, 'templates');
const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');

/**
 * @typedef {Object} IndexEntry
 * @property {string} url - URL to the index entry.
 * @property {string} title - Title of the target entry.
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
 * @typedef {Object} MetaItem
 * @property {string} file
 * @property {string} name
 */

/**
 * @typedef {Object} Meta
 * @property {string} name
 * @property {Array.<MetaItem>} itemsOrdered
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
        const wordPosition = text.indexOf(word);
        const wordClean = word.replace(/(,|\\.)/g, '').toLowerCase();
        if (index[wordClean] === undefined) {
            index[wordClean] = [];
        }

        index[wordClean].push({
            url: content.fullUrl,
            title: content.title,
            // TODO contain all instances of the word
            aroundText: text.substring(wordPosition - 50, wordPosition) + '...' + text.substring(wordPosition, wordPosition + 50)
        });
    }
}

(async function main() {
    const content = {}; // full path like "categories/custom-styles/custom-css.md" to
    fs.readdirSync(CATEGORIES_DIR).forEach(function (category) {
        /** @type {Meta} **/
        const meta = JSON.parse(fs.readFileSync(path.join(CATEGORIES_DIR, category, 'meta.json'), 'utf8'));
        meta.itemsOrdered.forEach((item) => {
            const title = item.name;
            const urlId = item.file.replace('md', 'html');
            const fullUrl = '/' + urlId;

            const fileContent = fs.readFileSync(path.join(CATEGORIES_DIR, category, 'items', item.file), 'utf8');
            const html = marked(fileContent);

            const entry = {
                html,
                title,
                urlId,
                fullUrl,
            };

            addContentToIndex(entry);

            content[`categories/${category}/${item.file}`] = entry;
        });
    });

    // Find guides.
    const guides = [];
    fs.readdirSync(GUIDES_DIR).forEach((guide) => {
        const meta = JSON.parse(fs.readFileSync(path.join(GUIDES_DIR, guide, 'meta.json'), 'utf8'));
        const hasItems = meta.itemsOrdered.length > 0;
        guides.push({
            id: guide,
            url: hasItems ? `guide-${guide}.html` : '#',
            icon: `images/guide-icons/${meta.icon}`,
            name: meta.name,
            hasItems
        });
    });

    // Create a page for each guide.
    for (const guide of guides) {
        /** @type {Meta} **/
        const meta = JSON.parse(fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'meta.json'), 'utf8'));
        const items = [];
        for (const item of meta.itemsOrdered) {
            const title = item.name;
            const id = item.file.replace('.md', '');
            const urlId = item.file.replace('md', 'html');

            // We add this guide item to the index, but its url is an anchor to the element on the guide page. This way we
            // can have all the content on one page, but still deep link to it from search nicely.
            const fullUrl = `/${guide.url}#${id}`;

            const markdown = fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'items', item.file), 'utf8');
            const html = marked(await processDynamicContent(markdown));

            items.push({
                title,
                id,
                content: html
            });

            const entry = {
                html,
                title,
                urlId,
                fullUrl,
            };

            addContentToIndex(entry);
        }
        const guideIndexPath = path.join(GUIDES_DIR, guide.id, 'index.md.html');
        if (fs.existsSync(guideIndexPath)) {
            const guideContentHTML = handlebars.compile(marked(fs.readFileSync(guideIndexPath, 'utf8')))({
                items
            });
            fs.writeFileSync(path.join(STATIC_GENERATED_DIR, guide.url), getCompiledTemplate(path.join(TEMPLATE_DIR, 'page.html'), {
                title: meta.name,
                content: guideContentHTML
            }), 'utf8');
        }
    }

    // Create the index.
    const indexRoot = {};
    for (const word in index) {
        const id = shortid.generate();
        indexRoot[word] = id;
        fs.writeFileSync(path.join(STATIC_GENERATED_DIR, `index-${id}.json`), JSON.stringify(index[word]), 'utf8');
    }
    const indexRootJSON = JSON.stringify(indexRoot);
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'index.json'), indexRootJSON, 'utf8'); // Currently, this is only written to disk for debugging.

    // Create the homepage.
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'index.html'), getCompiledTemplate(path.join(TEMPLATE_DIR, 'index.html'), {
        indexJSON: indexRootJSON,
        guides: guides
    }), 'utf8');

    console.log(`Execution Time: ${Date.now() - startTime}ms`);
})();

function getCompiledTemplate(templatePath, data) {
    return handlebars.compile(fs.readFileSync(templatePath, 'utf8'))(data);
}

