const fs = require('fs');
const path = require('path');
const marked = require('marked');
const handlebars = require('handlebars');
const {addContentToIndex} = require('./index');
const {ExampleTenantId} = require('./utils');
const {getCompiledTemplate} = require('./utils');
const {processDynamicContent} = require('./guide-dynamic-content-transformer');

const GUIDES_DIR_NAME = 'guides';
const ITEMS_DIR_NAME = 'items';
const INDEX_NAME = 'index.md.html';
const GUIDES_DIR = path.join(__dirname, 'content', GUIDES_DIR_NAME);
const TEMPLATE_DIR = path.join(__dirname, 'templates');
const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');
const GUIDE_LAYOUT_PATH = path.join(__dirname, 'templates', 'guide-layout.html');
const GUIDE_INTRO_FILE_NAME = 'intro.md';
const GUIDE_CONCLUSION_FILE_NAME = 'conclusion.md';

/**
 * @typedef {Object} Guide
 * @property {string} id
 * @property {string} url
 * @property {string} icon
 * @property {string} name
 * @property {string} itemsPath
 * @property {string} metaJSONPath
 * @property {string} indexTemplatePath
 * @property {string} conclusionPath
 * @property {string} introPath
 */

async function buildGuideItem(guide, item, index) {
    const title = item.name;
    const id = item.file.replace('.md', '');
    const urlId = item.file.replace('md', 'html');

    // We add this guide item to the index, but its url is an anchor to the element on the guide page. This way we
    // can have all the content on one page, but still deep link to it from search nicely.
    const fullUrl = `/${guide.url}#${id}`;

    const markdown = handlebars.compile(fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'items', item.file), 'utf8'))({
        ExampleTenantId
    });
    let html = marked(await processDynamicContent(markdown, path.join('src', 'content', GUIDES_DIR_NAME, guide.id, 'items', item.file)));

    html += '<style>' + fs.readFileSync(path.join(__dirname, './../node_modules/highlight.js/styles/monokai-sublime.css'), 'utf8') + '</style>';

    addContentToIndex({
        html,
        title,
        urlId,
        fullUrl
    }, index);

    return {
        title,
        id,
        content: html
    };
}

/**
 *
 * @param {Guide} guide
 * @param {Index} index
 * @return {Promise<void>}
 */
async function buildGuide(guide, index) {
    /** @type {Meta} **/
    const meta = JSON.parse(fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'meta.json'), 'utf8'));
    const items = await Promise.all(meta.itemsOrdered.map((item) => {
        return buildGuideItem(guide, item, index);
    }));
    const guideIndexPath = path.join(GUIDES_DIR, guide.id, 'index.md.html');
    if (fs.existsSync(guideIndexPath)) {
        const guideIntroHTML = marked(fs.readFileSync(path.join(GUIDES_DIR, guide.id, GUIDE_INTRO_FILE_NAME), 'utf8'));
        const guideConclusionHTML = marked(fs.readFileSync(path.join(GUIDES_DIR, guide.id, GUIDE_CONCLUSION_FILE_NAME), 'utf8'));
        const guideContentHTML = handlebars.compile(fs.readFileSync(GUIDE_LAYOUT_PATH, 'utf8'))({
            intro: guideIntroHTML,
            items,
            conclusion: guideConclusionHTML
        });
        const guideRootHTML = handlebars.compile(marked(fs.readFileSync(guideIndexPath, 'utf8')))({
            content: guideContentHTML
        });
        fs.writeFileSync(path.join(STATIC_GENERATED_DIR, guide.url), getCompiledTemplate(path.join(TEMPLATE_DIR, 'page.html'), {
            title: meta.name,
            content: guideRootHTML,
            ExampleTenantId: ExampleTenantId
        }), 'utf8');
    }
}

/**
 *
 * @return {Array.<Guide>}
 */
function getGuides() {
    const result = [];
    fs.readdirSync(GUIDES_DIR).forEach((guide) => {
        const metaJSONPath = path.join(GUIDES_DIR, guide, 'meta.json');
        const meta = JSON.parse(fs.readFileSync(metaJSONPath, 'utf8'));
        const hasItems = meta.itemsOrdered.length > 0 || meta.url;
        if (hasItems) {
            result.push({
                id: guide,
                url: meta.itemsOrdered.length > 0 ? `guide-${guide}.html` : (meta.url ? meta.url : '#'),
                icon: `images/guide-icons/${meta.icon}`,
                name: meta.name,
                metaJSONPath,
                itemsPath: path.join(GUIDES_DIR, guide, ITEMS_DIR_NAME),
                indexTemplatePath: path.join(GUIDES_DIR, guide, INDEX_NAME),
                conclusionPath: path.join(GUIDES_DIR, guide, GUIDE_CONCLUSION_FILE_NAME),
                introPath: path.join(GUIDES_DIR, guide, GUIDE_INTRO_FILE_NAME)
            });
        }
    });
    return result;
}

module.exports = {
    TEMPLATE_DIR,
    GUIDE_LAYOUT_PATH,
    buildGuide,
    getGuides
};
