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
const GUIDES_DIR = path.join(__dirname, 'content', GUIDES_DIR_NAME);
const GUIDE_ORDER_PATH = path.join(__dirname, 'content', GUIDES_DIR_NAME, 'guide-order.json');
const TEMPLATE_DIR = path.join(__dirname, 'templates');
const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');
const GUIDE_LAYOUT_PATH = path.join(__dirname, 'templates', 'guide-layout.html');
const NODE_MODULES_PATH = path.join(__dirname, '..', 'node_modules');

/**
 * @typedef {Object} GuidePage
 * @property {string} id
 * @property {string} prevGuideUrl
 * @property {string} nextGuideUrl
 * @property {string} url
 * @property {string} name
 * @property {string} itemsPath
 * @property {string} metaJSONPath
 */

/**
 * @typedef {Object} Guide
 * @property {string} id
 * @property {string} icon
 * @property {string} name
 * @property {string} metaJSONPath
 * @property {GuidePage[]} pages
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

/**
 * @param {string} filePath
 * @return {string}
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
    const fullUrl = `/${guide.url}#${id}`; // TODO BROKEN

    let html = '';
    if (metaItem.file.endsWith('.md')) {
        const metaItemAbsPath = fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'items', metaItem.file), 'utf8');
        const markdown = handlebars.compile(metaItemAbsPath)({
            ExampleTenantId
        });
        html = marked(await processDynamicContent(markdown, path.join('src', 'content', GUIDES_DIR_NAME, guide.id, 'items', metaItem.file)));
    } else if (metaItem.file.endsWith('.json')) {
        // TODO
    } else {
        throw new Error(`Don't know how to handle meta item file=[${metaItem.file}]`);
    }

    addContentToIndex({ // TODO DO NOT DO THIS HERE
        html,
        title,
        urlId,
        fullUrl
    }, index);

    html += '<style>' + fs.readFileSync(path.join(NODE_MODULES_PATH, '/highlight.js/styles/monokai-sublime.css'), 'utf8') + '</style>';

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
    await buildGuideFromItems(guide, null, {});

    // return items;
}

async function buildGuideFromItems(guide, items, index) {
    for (const page of guide.pages) {
        await buildGuidePage(guide, page, index);
    }
}

async function buildGuidePage(guide, page, index) {
    if (page.pageLayoutPath === GUIDE_LAYOUT_PATH && page.data && page.data.items) { // TODO HACK SHOULD BE DETERMINED VIA PAGE TYPE
        page.data.items = await Promise.all(page.data.items.map((metaItem) => {
            return buildGuideItemForMeta(guide, metaItem, index);
        }));
        page.data.itemsBySubCat = groupBy(page.data.items, 'subCat');
    }
    const guideContentHTML = handlebars.compile(fs.readFileSync(page.pageLayoutPath, 'utf8'))(page.data);
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, page.url), getCompiledTemplate(path.join(TEMPLATE_DIR, 'page.html'), {
        title: guide.name,
        content: guideContentHTML,
        ExampleTenantId: ExampleTenantId
    }), 'utf8');
}

function createPageLink(id) {
    return `guide-${id}.html`;
}

/**
 *
 * @param {string} guideId
 * @param {Meta} meta
 * @return {GuidePage[]}
 */
function getGuidePages(guideId, meta) {
    if (meta.scriptFile) {
        return require(path.join(GUIDES_DIR, guideId, meta.scriptFile))(meta);
    } else {
        const hasItems = meta.itemsOrdered.length > 0 || meta.url;
        if (hasItems) {
            /** @type {Array.<string>} **/
            const guideOrder = JSON.parse(fs.readFileSync(GUIDE_ORDER_PATH, 'utf8'));
            const guideIndex = guideOrder.indexOf(guideId);
            console.log('guideIndex', guideId, guideOrder, guideIndex, guideIndex > -1 ? createPageLink(guideOrder[guideIndex - 1]) : null, guideIndex > -1 && guideIndex < guideOrder.length - 1 ? createPageLink(guideOrder[guideIndex + 1]) : null)
            return [
                {
                    id: guideId,
                    prevGuideUrl: guideIndex > 0 ? createPageLink(guideOrder[guideIndex - 1]) : null,
                    nextGuideUrl: guideIndex > -1 && guideIndex < guideOrder.length - 1 ? createPageLink(guideOrder[guideIndex + 1]) : null,
                    url: meta.itemsOrdered.length > 0 ? createPageLink(guideId) : (meta.url ? meta.url : '#'),
                    icon: `images/guide-icons/${meta.icon}`,
                    name: meta.name,
                    itemsPath: path.join(GUIDES_DIR, guideId, ITEMS_DIR_NAME),
                    pageLayoutPath: GUIDE_LAYOUT_PATH,
                    data: {
                        items: meta.itemsOrdered,
                        itemsBySubCat: groupBy(meta.itemsOrdered, 'subCat')
                    }
                }
            ]
        }
    }
    return [];
}

/**
 *
 * @return {Array.<Guide>}
 */
function getGuides() {
    const result = [];
    fs.readdirSync(GUIDES_DIR).forEach((guideId) => {
        if (guideId === 'guide-order.json') {
            return;
        }
        const metaJSONPath = path.join(GUIDES_DIR, guideId, 'meta.json');
        /** @type {Meta} */
        const meta = JSON.parse(fs.readFileSync(metaJSONPath, 'utf8'));

        const pages = getGuidePages(guideId, meta);

        if (pages.length > 0) {
            result.push({
                id: guideId,
                icon: `images/guide-icons/${meta.icon}`,
                name: meta.name,
                pages
            });
        }
    });
    return result;
}

module.exports = {
    GUIDES_DIR_NAME,
    GUIDES_DIR,
    TEMPLATE_DIR,
    GUIDE_LAYOUT_PATH,
    STATIC_GENERATED_DIR,
    NODE_MODULES_PATH,
    ITEMS_DIR_NAME,
    buildGuide,
    buildGuideItemForMeta,
    buildGuideFromItems,
    createPageLink,
    createGuideItemIdFromPath,
    getGuides
};
