const startTime = Date.now();

const fs = require('fs');
const path = require('path');
const marked = require('marked');
const shortid = require('shortid');
const {getGuides} = require('./guides');
const {persistIndex} = require('./index');
const {addContentToIndex} = require('./index');
const {buildGuide} = require('./guides');
const {getCompiledTemplate} = require('./utils');
const {dispose} = require('./guide-dynamic-content-transformer');

const CATEGORIES_DIR = path.join(__dirname, 'content', 'categories');
const GUIDES_DIR_NAME = 'guides';
const GUIDES_DIR = path.join(__dirname, 'content', GUIDES_DIR_NAME);
const TEMPLATE_DIR = path.join(__dirname, 'templates');
const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');

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
 * @property {string} subCat
 */

/**
 * @typedef {Object} Meta
 * @property {string} name
 * @property {Array.<MetaItem>} itemsOrdered
 */

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

            addContentToIndex(entry, index);

            content[`categories/${category}/${item.file}`] = entry;
        });
    });

    // Find guides.
    /** @type {Array.<Guide>} **/
    const guides = getGuides();

    // We'll periodically compare the build id on the page with the one from an API call to alert the user if the docs are out of date.
    const buildId = shortid.generate();

    // Create a page for each guide.
    await Promise.all(guides.map((guide) => {
        return buildGuide(guide, index);
    }))

    // Create the index.
    const indexRootJSON = persistIndex(index);

    // Store the build id.
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'build-id'), buildId, 'utf8');

    // Create the homepage.
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'index.html'), getCompiledTemplate(path.join(TEMPLATE_DIR, 'index.html'), {
        indexJSON: indexRootJSON,
        guides,
        buildId
    }), 'utf8');

    await dispose();

    console.log(`Execution Time: ${Date.now() - startTime}ms`);
    process.exit(0); // it seems like puppeteer is keeping the process alive...
})();

