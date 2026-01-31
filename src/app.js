const startTime = Date.now();

const fs = require('fs');
const path = require('path');
const shortid = require('shortid');
const {getGuides} = require('./guides');
const {buildGuide} = require('./guides');
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

    await dispose();

    console.log(`Execution Time: ${Date.now() - startTime}ms`);
    process.exit(0); // it seems like puppeteer is keeping the process alive...
})();

