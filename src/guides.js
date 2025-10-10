const fs = require('fs');
const path = require('path');
const marked = require('marked');
const handlebars = require('handlebars');
const {ExampleTenantId} = require('./utils');
const {getCompiledTemplate} = require('./utils');
const {processDynamicContent} = require('./guide-dynamic-content-transformer');
const snippetProcessor = require('./snippet-processor');

function groupBy(array, key) {
    return array.reduce((result, item) => {
        const group = item[key];
        if (!result[group]) {
            result[group] = [];
        }
        result[group].push(item);
        return result;
    }, {});
}

const GUIDES_DIR_NAME = 'guides';
const ITEMS_DIR_NAME = 'items';
const INDEX_NAME = 'index.md.html';
const GUIDES_DIR = path.join(__dirname, 'content', GUIDES_DIR_NAME);
const GUIDE_ORDER_PATH = path.join(__dirname, 'content', GUIDES_DIR_NAME, 'guide-order.json');
const TEMPLATE_DIR = path.join(__dirname, 'templates');
const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');
const GUIDE_LAYOUT_PATH = path.join(__dirname, 'templates', 'guide-layout.html');
const GUIDE_INTRO_FILE_NAME = 'intro.md';
const GUIDE_CONCLUSION_FILE_NAME = 'conclusion.md';

/**
 * @typedef {Object} Guide
 * @property {string} id
 * @property {string} prevGuideUrl
 * @property {string} nextGuideUrl
 * @property {string} url
 * @property {string} icon
 * @property {string} name
 * @property {string} itemsPath
 * @property {string} metaJSONPath
 * @property {string} indexTemplatePath
 * @property {string} conclusionPath
 * @property {string} introPath
 * @property {string} pageHeader
 */

/**
 * @typedef {Object} GuideItem
 * @property {string} id
 * @property {string} title
 * @property {string} content
 * @property {string} name
 * @property {string} file
 * @property {string} subCat
 * @property {string} fullUrl
 * @property {string} [sidebarItemClasses]
 */

function createGuideItemIdFromPath(filePath) {
    return filePath.replace('.md', '');
}

/**
 *
 * @param {Guide} guide
 * @param {MetaItem} metaItem
 * @return {Promise<GuideItem>}
 */
async function buildGuideItemForMeta(guide, metaItem) {
    console.log('buildGuideItemForMeta', metaItem.file);
    const title = metaItem.name;
    const id = createGuideItemIdFromPath(metaItem.file);
    const urlId = metaItem.file.replace('md', 'html');

    // We add this guide metaItem to the index, but its url is an anchor to the element on the guide page. This way we
    // can have all the content on one page, but still deep link to it from search nicely.
    const fullUrl = `/${guide.url}#${id}`;

    const markdown = handlebars.compile(fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'items', metaItem.file), 'utf8'))({
        ExampleTenantId
    });
    let html = marked(await processDynamicContent(markdown, path.join('src', 'content', GUIDES_DIR_NAME, guide.id, 'items', metaItem.file)));
    
    // Apply snippet processor after markdown processing
    html = snippetProcessor(html, path.join('src', 'content', GUIDES_DIR_NAME, guide.id, 'items', metaItem.file));

    html += '<style>' + fs.readFileSync(path.join(__dirname, './../node_modules/highlight.js/styles/monokai-sublime.css'), 'utf8') + '</style>';

    return {
        title,
        id,
        name: metaItem.name,
        file: metaItem.file,
        subCat: metaItem.subCat,
        sidebarItemClasses: metaItem.sidebarItemClasses,
        content: html,
        itemClasses: html.includes('https://fastcomments.com') ? 'has-site-link' : '', // determine this at build time, so we can use a hack to quickly replace all the fastcomments.com links to eu.fastcomments.com
        fullUrl,
    };
}

/**
 *
 * @param {Guide} guide
 * @param {Index} index
 * @return {Promise<GuideItem[]>}
 */
async function buildGuide(guide, index) {
    /** @type {Meta} **/
    const meta = JSON.parse(fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'meta.json'), 'utf8'));
    const items = [];
    for (const metaItem of meta.itemsOrdered) {
        items.push(await buildGuideItemForMeta(guide, metaItem, index)); // this is done one at a time to be easier to understand
    }
    await buildGuideFromItems(guide, items);

    return items;
}

async function buildGuideFromItems(guide, items) {
    const introPath = path.join(GUIDES_DIR, guide.id, GUIDE_INTRO_FILE_NAME);
    const guideIntroHTML = marked(fs.existsSync(introPath) ? fs.readFileSync(introPath, 'utf8') : '');
    const conclusionPath = path.join(GUIDES_DIR, guide.id, GUIDE_CONCLUSION_FILE_NAME);
    const guideConclusionHTML = marked(fs.existsSync(conclusionPath) ? fs.readFileSync(conclusionPath, 'utf8') : '');

    const guideContentHTML = handlebars.compile(fs.readFileSync(GUIDE_LAYOUT_PATH, 'utf8'))({
        intro: guideIntroHTML,
        items,
        itemsBySubCat: groupBy(items, 'subCat'),
        conclusion: guideConclusionHTML,
        ...guide
    });
    const guideIndexPath = path.join(GUIDES_DIR, guide.id, 'index.md.html');
    const guideRootHTML = fs.existsSync(guideIndexPath) ? handlebars.compile(marked(fs.readFileSync(guideIndexPath, 'utf8')))({
        content: guideContentHTML
    }) : guideContentHTML;
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, guide.url), getCompiledTemplate(path.join(TEMPLATE_DIR, 'page.html'), {
        title: guide.pageHeader ? guide.pageHeader : guide.name,
        content: guideRootHTML,
        ExampleTenantId: ExampleTenantId
    }), 'utf8');
}

function createGuideLink(id) {
    return `guide-${id}.html`;
}

/**
 *
 * @return {Array.<Guide>}
 */
function getGuides() {
    const result = [];
    fs.readdirSync(GUIDES_DIR).forEach((guide) => {
        if (guide === 'guide-order.json') {
            return;
        }
        const metaJSONPath = path.join(GUIDES_DIR, guide, 'meta.json');
        if (!fs.existsSync(metaJSONPath)) {
            return console.warn('Skipping', guide, 'as it does not have a meta.json');
        }
        const meta = JSON.parse(fs.readFileSync(metaJSONPath, 'utf8'));
        const hasItems = meta.itemsOrdered.length > 0 || meta.url;
        if (hasItems) {
            /** @type {Array.<string>} **/
            const guideOrder = JSON.parse(fs.readFileSync(GUIDE_ORDER_PATH, 'utf8'));
            const guideIndex = guideOrder.indexOf(guide);
            console.log('guideIndex', guide, guideOrder, guideIndex, guideIndex > -1 ? createGuideLink(guideOrder[guideIndex - 1]) : null, guideIndex > -1 && guideIndex < guideOrder.length - 1 ? createGuideLink(guideOrder[guideIndex + 1]) : null)

            result.push({
                id: guide,
                pageHeader: meta.pageHeader,
                prevGuideUrl: guideIndex > 0 ? createGuideLink(guideOrder[guideIndex - 1]) : null,
                nextGuideUrl: guideIndex > -1 && guideIndex < guideOrder.length - 1 ? createGuideLink(guideOrder[guideIndex + 1]) : null,
                url: meta.itemsOrdered.length > 0 ? createGuideLink(guide) : (meta.url ? meta.url : '#'),
                urlEncoded: encodeURIComponent(meta.itemsOrdered.length > 0 ? 'https://docs.fastcomments.com/' + createGuideLink(guide) : (meta.url ? meta.url : '#')), // is relative path
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

function getGuideMeta(id) {
    return JSON.parse(fs.readFileSync(path.join(GUIDES_DIR, id, 'meta.json'), 'utf8'));
}

module.exports = {
    GUIDES_DIR,
    TEMPLATE_DIR,
    GUIDE_LAYOUT_PATH,
    buildGuide,
    buildGuideItemForMeta,
    buildGuideFromItems,
    createGuideLink,
    createGuideItemIdFromPath,
    getGuides,
    getGuideMeta,
};
