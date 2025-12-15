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
    // Find guides.
    /** @type {Array.<Guide>} **/
    const guides = getGuides();

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

    const guidesNotInstallation = guides.filter((guide) => !guide.id.startsWith('installation-') && !guide.id.startsWith('sdk-'));
    const installationGuides = guides.filter((guide) => guide.id.startsWith('installation-'));
    const sdkGuides = guides.filter((guide) => guide.id.startsWith('sdk-'));

    // Store the build id.
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'build-id'), buildId, 'utf8');

    // Create the homepage.
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, 'index.html'), getCompiledTemplate(path.join(TEMPLATE_DIR, 'index.html'), {
        guides: guidesNotInstallation,
        installationGuides,
        sdkGuides,
        gettingStartedGuides,
        lastUpdateDate: new Date().toLocaleString(),
        buildId
    }), 'utf8');

    await dispose();

    console.log(`Execution Time: ${Date.now() - startTime}ms`);
    process.exit(0); // it seems like puppeteer is keeping the process alive...
})();

