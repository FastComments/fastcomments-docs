const fs = require('fs');
const path = require('path');
const marked = require('marked');
const hljs = require('highlight.js');
const handlebars = require('handlebars');
const {ExampleTenantId} = require('./utils');
const {getCompiledTemplate} = require('./utils');
const {processDynamicContent} = require('./guide-dynamic-content-transformer');
const snippetProcessor = require('./snippet-processor');
const {locales, defaultLocale} = require('./locales');

// Configure marked to use highlight.js for code blocks
marked.setOptions({
    highlight: function (code, lang) {
        if (lang && hljs.getLanguage(lang)) {
            return hljs.highlight(code.trim(), { language: lang }).value;
        }
        return hljs.highlightAuto(code.trim()).value;
    }
});

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
 * @param {string} locale
 * @return {Promise<GuideItem>}
 */
async function buildGuideItemForMeta(guide, metaItem, locale = defaultLocale) {
    console.log('buildGuideItemForMeta', metaItem.file, 'locale:', locale);
    const title = metaItem.name;
    const id = createGuideItemIdFromPath(metaItem.file);
    const urlId = metaItem.file.replace('md', 'html');

    // We add this guide metaItem to the index, but its url is an anchor to the element on the guide page. This way we
    // can have all the content on one page, but still deep link to it from search nicely.
    const guideUrl = createGuideLink(guide.id, locale);
    const fullUrl = `/${guideUrl}#${id}`;

    // Check for localized content first, then fall back to default locale
    let itemPath = path.join(GUIDES_DIR, guide.id, 'items', locale, metaItem.file);
    let isFallback = false;

    if (!fs.existsSync(itemPath)) {
        // Try default locale as fallback
        itemPath = path.join(GUIDES_DIR, guide.id, 'items', defaultLocale, metaItem.file);
        if (locale !== defaultLocale) {
            isFallback = true;
        }
    }

    // Also check for non-localized path (for backwards compatibility with guides that haven't been reorganized)
    if (!fs.existsSync(itemPath)) {
        itemPath = path.join(GUIDES_DIR, guide.id, 'items', metaItem.file);
        isFallback = false; // not a fallback if it's the original structure
    }

    // Skip if file doesn't exist (generated files may not be present yet)
    if (!fs.existsSync(itemPath)) {
        if (metaItem.file.endsWith('-generated.md')) {
            console.warn(`Skipping generated file (not yet created): ${metaItem.file}`);
            return null;
        }
        throw new Error(`Required file not found: ${itemPath}`);
    }

    const markdown = handlebars.compile(fs.readFileSync(itemPath, 'utf8'))({
        ExampleTenantId
    });

    // Determine the relative path for dynamic content processing
    const relativePath = itemPath.replace(path.join(__dirname, '..') + '/', '');
    let html = marked(await processDynamicContent(markdown, relativePath));

    // Apply snippet processor after markdown processing
    html = snippetProcessor(html, relativePath);

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
        isFallback,
    };
}

/**
 *
 * @param {Guide} guide
 * @param {Index} index
 * @param {string} locale
 * @return {Promise<GuideItem[]>}
 */
async function buildGuide(guide, index, locale = defaultLocale) {
    /** @type {Meta} **/
    const meta = JSON.parse(fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'meta.json'), 'utf8'));
    const items = [];
    for (const metaItem of meta.itemsOrdered) {
        const item = await buildGuideItemForMeta(guide, metaItem, locale); // this is done one at a time to be easier to understand
        if (item) {
            items.push(item);
        }
    }
    await buildGuideFromItems(guide, items, locale);

    return items;
}

async function buildGuideFromItems(guide, items, locale = defaultLocale) {
    const introPath = path.join(GUIDES_DIR, guide.id, GUIDE_INTRO_FILE_NAME);
    const guideIntroHTML = marked(fs.existsSync(introPath) ? fs.readFileSync(introPath, 'utf8') : '');
    const conclusionPath = path.join(GUIDES_DIR, guide.id, GUIDE_CONCLUSION_FILE_NAME);
    const guideConclusionHTML = marked(fs.existsSync(conclusionPath) ? fs.readFileSync(conclusionPath, 'utf8') : '');

    // Check if any items are using fallback content
    const hasFallbackContent = items.some(item => item.isFallback);

    // Build alternate locale links for hreflang tags
    const currentUrl = createGuideLink(guide.id, locale);
    const alternateLocales = Object.keys(locales).map(loc => ({
        hreflang: locales[loc].hreflang,
        url: createGuideLink(guide.id, loc),
        current: loc === locale
    }));

    // Build available locales for language selector
    const availableLocales = Object.keys(locales).map(loc => ({
        code: loc,
        name: locales[loc].name,
        nativeName: locales[loc].nativeName,
        flag: locales[loc].flag || '🌐',
        url: createGuideLink(guide.id, loc),
        current: loc === locale
    }));

    const guideContentHTML = handlebars.compile(fs.readFileSync(GUIDE_LAYOUT_PATH, 'utf8'))({
        intro: guideIntroHTML,
        items,
        itemsBySubCat: groupBy(items, 'subCat'),
        conclusion: guideConclusionHTML,
        isFallback: hasFallbackContent,
        locale,
        ...guide
    });
    const guideIndexPath = path.join(GUIDES_DIR, guide.id, 'index.md.html');
    const guideRootHTML = fs.existsSync(guideIndexPath) ? handlebars.compile(marked(fs.readFileSync(guideIndexPath, 'utf8')))({
        content: guideContentHTML
    }) : guideContentHTML;

    const outputUrl = createGuideLink(guide.id, locale);
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, outputUrl), getCompiledTemplate(path.join(TEMPLATE_DIR, 'page.html'), {
        title: guide.pageHeader ? guide.pageHeader : guide.name,
        content: guideRootHTML,
        ExampleTenantId: ExampleTenantId,
        lang: locales[locale].hreflang,
        locale,
        alternateLocales,
        availableLocales,
        defaultUrl: createGuideLink(guide.id, defaultLocale)
    }), 'utf8');
}

function createGuideLink(id, locale) {
    if (locale && locale !== defaultLocale) {
        return `guide-${id}-${locale}.html`;
    }
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
                icon: meta.icon ? `images/guide-icons/${meta.icon}` : null,
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
