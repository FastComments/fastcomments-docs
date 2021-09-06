const fs = require('fs');
const path = require('path');
const marked = require('marked');
const {groupBy} = require('lodash');
const handlebars = require('handlebars');
const {addContentToIndex} = require('./index');
const {ExampleTenantId} = require('./utils');
const {getCompiledTemplate} = require('./utils');
const {processDynamicContent} = require('./guide-dynamic-content-transformer');

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
 * @property {string} [layoutFile]
 * @property {string} itemsPath
 * @property {string} metaJSONPath
 * @property {string} indexTemplatePath
 * @property {string} conclusionPath
 * @property {string} introPath
 */

/**
 * @typedef {Object} GuideItem
 * @property {string} id
 * @property {string} title
 * @property {string} content
 * @property {string} name
 * @property {string} file
 * @property {string} subCat
 * @property {string} [sidebarItemClasses]
 */

function createGuideItemIdFromPath(filePath) {
    return filePath.replace('.md', '');
}

/**
 *
 * @param {Guide} guide
 * @param {MetaItem} metaItem
 * @param {Index} index
 * @return {Promise<GuideItem>}
 */
async function buildGuideItemForMeta(guide, metaItem, index) {
    console.log('buildGuideItemForMeta', metaItem.file);
    const title = metaItem.name;
    const id = createGuideItemIdFromPath(metaItem.file);
    const urlId = metaItem.file.replace('md', 'html');

    // We add this guide metaItem to the index, but its url is an anchor to the element on the guide page. This way we
    // can have all the content on one page, but still deep link to it from search nicely.
    const fullUrl = `/${guide.url}#${id}`;

    let html = '';
    if (metaItem.file.endsWith('.md')) {
        const metaItemAbsPath = fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'items', metaItem.file), 'utf8');
        const markdown = handlebars.compile(metaItemAbsPath)({
            ExampleTenantId
        });
        html = marked(await processDynamicContent(markdown, path.join('src', 'content', GUIDES_DIR_NAME, guide.id, 'items', metaItem.file)));
    } else if(metaItem.file.endsWith('.json')) {
        // TODO
    } else {
        throw new Error(`Don't know how to handle meta item file=[${metaItem.file}]`);
    }

    addContentToIndex({
        html,
        title,
        urlId,
        fullUrl
    }, index);

    html += '<style>' + fs.readFileSync(path.join(__dirname, './../node_modules/highlight.js/styles/monokai-sublime.css'), 'utf8') + '</style>';

    return {
        title,
        id,
        name: metaItem.name,
        description: metaItem.description,
        file: metaItem.file,
        icon: metaItem.icon,
        subCat: metaItem.subCat,
        sidebarItemClasses: metaItem.sidebarItemClasses,
        content: html
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
    const items = await Promise.all(meta.itemsOrdered.map((metaItem) => {
        return buildGuideItemForMeta(guide, metaItem, index);
    }));
    guide.layoutFile = meta.layoutFile;
    await buildGuideFromItems(guide, items);

    return items;
}

async function buildGuideFromItems(guide, items) {
    const guideIndexPath = path.join(GUIDES_DIR, guide.id, 'index.md.html');
    if (fs.existsSync(guideIndexPath)) {
        const guideIntroHTML = marked(fs.readFileSync(path.join(GUIDES_DIR, guide.id, GUIDE_INTRO_FILE_NAME), 'utf8'));
        const guideConclusionHTML = marked(fs.readFileSync(path.join(GUIDES_DIR, guide.id, GUIDE_CONCLUSION_FILE_NAME), 'utf8'));

        const layoutFile = guide.layoutFile ? path.join(__dirname, 'templates', guide.layoutFile) : GUIDE_LAYOUT_PATH;
        const guideContentHTML = handlebars.compile(fs.readFileSync(layoutFile, 'utf8'))({
            intro: guideIntroHTML,
            items,
            itemsBySubCat: groupBy(items, 'subCat'),
            conclusion: guideConclusionHTML,
            ...guide
        });
        const guideRootHTML = handlebars.compile(marked(fs.readFileSync(guideIndexPath, 'utf8')))({
            content: guideContentHTML
        });
        fs.writeFileSync(path.join(STATIC_GENERATED_DIR, guide.url), getCompiledTemplate(path.join(TEMPLATE_DIR, 'page.html'), {
            title: guide.name,
            content: guideRootHTML,
            ExampleTenantId: ExampleTenantId
        }), 'utf8');
    }
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
        const meta = JSON.parse(fs.readFileSync(metaJSONPath, 'utf8'));
        const hasItems = meta.itemsOrdered.length > 0 || meta.url;
        if (hasItems) {
            /** @type {Array.<string>} **/
            const guideOrder = JSON.parse(fs.readFileSync(GUIDE_ORDER_PATH, 'utf8'));
            const guideIndex = guideOrder.indexOf(guide);
            console.log('guideIndex', guide, guideOrder, guideIndex, guideIndex > -1 ? createGuideLink(guideOrder[guideIndex - 1]) : null, guideIndex > -1 && guideIndex < guideOrder.length - 1 ? createGuideLink(guideOrder[guideIndex + 1]) : null)

            result.push({
                id: guide,
                prevGuideUrl: guideIndex > 0 ? createGuideLink(guideOrder[guideIndex - 1]) : null,
                nextGuideUrl: guideIndex > -1 && guideIndex < guideOrder.length - 1 ? createGuideLink(guideOrder[guideIndex + 1]) : null,
                url: meta.itemsOrdered.length > 0 ? createGuideLink(guide) : (meta.url ? meta.url : '#'),
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
    buildGuideItemForMeta,
    buildGuideFromItems,
    createGuideItemIdFromPath,
    getGuides
};
