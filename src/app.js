const startTime = Date.now();

const fs = require('fs');
const path = require('path');
const shortid = require('shortid');
const {getGuides, buildGuide, createGuideLink} = require('./guides');
const {getCompiledTemplate} = require('./utils');
const {dispose} = require('./guide-dynamic-content-transformer');
const guideOrder = require('./content/guides/guide-order.json');
const {locales, defaultLocale} = require('./locales');

const TEMPLATE_DIR = path.join(__dirname, 'templates');
const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');
const TRANSLATIONS_FILE = path.join(__dirname, 'translations.json');

/**
 * Load translations for a specific locale, falling back to default locale
 * @param {string} locale - Target locale
 * @returns {Object} - Translations object
 */
function getTranslations(locale) {
    const translations = JSON.parse(fs.readFileSync(TRANSLATIONS_FILE, 'utf8'));
    return translations[locale] || translations[defaultLocale] || {};
}

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
 * @property {string} [sidebarItemClasses]
 */

/**
 * @typedef {Object} Meta
 * @property {string} name
 * @property {Array.<MetaItem>} itemsOrdered
 */

(async function main() {
    // Find guides for default locale (for building guide pages).
    /** @type {Array.<Guide>} **/
    const guides = getGuides(defaultLocale);

    const gettingStartedGuides = guides
        .filter((guide) => guideOrder.includes(guide.id))
        .sort((a, b) => guideOrder.indexOf(a.id) - guideOrder.indexOf(b.id));

    // We'll periodically compare the build id on the page with the one from an API call to alert the user if the docs are out of date.
    const buildId = shortid.generate();

    // Create a page for each guide in each locale.
    for (const guide of guides) {
        // Build default locale first
        await buildGuide(guide, index, defaultLocale);

        // Build other locales
        for (const locale of Object.keys(locales)) {
            if (locale !== defaultLocale) {
                await buildGuide(guide, index, locale);
            }
        }
    }

    // Store the build id.
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'build-id'), buildId, 'utf8');

    // Helper to localize guide URLs
    function localizeGuides(guides, locale) {
        if (locale === defaultLocale) {
            return guides;
        }
        return guides.map(guide => ({
            ...guide,
            url: guide.url.replace('.html', `-${locale}.html`)
        }));
    }

    // Create the homepage for each locale.
    for (const locale of Object.keys(locales)) {
        const t = getTranslations(locale);
        const localeInfo = locales[locale];
        const isDefault = locale === defaultLocale;
        const filename = isDefault ? 'index.html' : `index-${locale}.html`;

        // Get guides with locale-specific names for this locale
        const localeGuides = getGuides(locale);
        const localeGettingStartedGuides = localeGuides
            .filter((guide) => guideOrder.includes(guide.id))
            .sort((a, b) => guideOrder.indexOf(a.id) - guideOrder.indexOf(b.id));
        const guidesNotInstallation = localeGuides.filter((guide) => !guide.id.startsWith('installation-') && !guide.id.startsWith('sdk-') && !guide.id.startsWith('lib-'));
        const installationGuides = localeGuides.filter((guide) => guide.id.startsWith('installation-'));
        const sdkGuides = localeGuides.filter((guide) => guide.id.startsWith('sdk-') || guide.id.startsWith('lib-'));

        // Build available locales for language selector
        const availableLocales = Object.keys(locales).map(loc => ({
            code: loc,
            name: locales[loc].name,
            nativeName: locales[loc].nativeName,
            flag: locales[loc].flag || '🌐',
            url: loc === defaultLocale ? 'index.html' : `index-${loc}.html`,
            current: loc === locale
        }));

        fs.writeFileSync(path.join(STATIC_GENERATED_DIR, filename), getCompiledTemplate(path.join(TEMPLATE_DIR, 'index.html'), {
            guides: localizeGuides(guidesNotInstallation, locale),
            installationGuides: localizeGuides(installationGuides, locale),
            sdkGuides: localizeGuides(sdkGuides, locale),
            gettingStartedGuides: localizeGuides(localeGettingStartedGuides, locale),
            lastUpdateDate: new Date().toLocaleString(),
            buildId,
            locale,
            lang: localeInfo.hreflang,
            availableLocales,
            t
        }), 'utf8');
    }

    // Generate sitemap.xml
    console.log('Generating sitemap.xml...');
    const sitemapStartTime = Date.now();
    const BASE_URL = 'https://docs.fastcomments.com/';
    const allLocaleKeys = Object.keys(locales);

    let sitemapXml = '<?xml version="1.0" encoding="UTF-8"?>\n';
    sitemapXml += '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"\n';
    sitemapXml += '        xmlns:xhtml="http://www.w3.org/1999/xhtml">\n';

    // Homepage entries
    for (const locale of allLocaleKeys) {
        const isDefault = locale === defaultLocale;
        const filename = isDefault ? 'index.html' : `index-${locale}.html`;
        sitemapXml += '  <url>\n';
        sitemapXml += `    <loc>${BASE_URL}${filename}</loc>\n`;
        for (const altLocale of allLocaleKeys) {
            const altIsDefault = altLocale === defaultLocale;
            const altFilename = altIsDefault ? 'index.html' : `index-${altLocale}.html`;
            sitemapXml += `    <xhtml:link rel="alternate" hreflang="${locales[altLocale].hreflang}" href="${BASE_URL}${altFilename}"/>\n`;
        }
        sitemapXml += '  </url>\n';
    }

    // Guide page entries
    for (const guide of guides) {
        if (guide.id.startsWith('code-')) {
            continue;
        }
        for (const locale of allLocaleKeys) {
            const guideUrl = createGuideLink(guide.id, locale);
            sitemapXml += '  <url>\n';
            sitemapXml += `    <loc>${BASE_URL}${guideUrl}</loc>\n`;
            for (const altLocale of allLocaleKeys) {
                const altUrl = createGuideLink(guide.id, altLocale);
                sitemapXml += `    <xhtml:link rel="alternate" hreflang="${locales[altLocale].hreflang}" href="${BASE_URL}${altUrl}"/>\n`;
            }
            sitemapXml += '  </url>\n';
        }
    }

    sitemapXml += '</urlset>\n';

    const sitemapUrlCount = allLocaleKeys.length + guides.filter(g => !g.id.startsWith('code-')).length * allLocaleKeys.length;
    const sitemapBytes = Buffer.byteLength(sitemapXml, 'utf8');
    const MAX_SITEMAP_URLS = 50000;
    const MAX_SITEMAP_BYTES = 50 * 1024 * 1024; // 50MB
    if (sitemapUrlCount > MAX_SITEMAP_URLS) {
        throw new Error(`Sitemap exceeds max URL count: ${sitemapUrlCount} > ${MAX_SITEMAP_URLS}`);
    }
    if (sitemapBytes > MAX_SITEMAP_BYTES) {
        throw new Error(`Sitemap exceeds max file size: ${(sitemapBytes / 1024 / 1024).toFixed(1)}MB > 50MB`);
    }

    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'sitemap.xml'), sitemapXml, 'utf8');
    console.log(`Sitemap generated in ${Date.now() - sitemapStartTime}ms with ${sitemapUrlCount} URLs (${(sitemapBytes / 1024 / 1024).toFixed(1)}MB)`);

    await dispose();

    console.log(`Execution Time: ${Date.now() - startTime}ms`);
    process.exit(0); // it seems like puppeteer is keeping the process alive...
})();

