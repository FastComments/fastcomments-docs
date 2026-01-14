/**
 * Worker script for building a single locale's search index.
 * Called by build-search-index-parallel.js
 */

const { parentPort, workerData } = require('worker_threads');
const Database = require('better-sqlite3');
const fs = require('fs');
const path = require('path');
const { htmlToText } = require('html-to-text');
const marked = require('marked');
const {
    getGuides,
    buildGuideItemForMeta,
    getGuideMeta,
    createGuideLink
} = require('./guides');
const { defaultLocale } = require('./locales');

const DB_DIR = path.join(__dirname, '..', 'db');
const GUIDES_DIR = path.join(__dirname, 'content', 'guides');

const htmlToTextOptions = {
    selectors: [
        { selector: '.line-number', format: 'skip' },
        { selector: '.copy', format: 'skip' },
        { selector: '.top-right', format: 'skip' },
        { selector: 'img', format: 'skip' }
    ]
};

function cleanSearchText(html) {
    const withoutHljsStyles = html.replace(/<style>pre code\.hljs\{[\s\S]*?<\/style>/g, '');
    return htmlToText(withoutHljsStyles, htmlToTextOptions);
}

async function buildIndexForLocale(locale) {
    const dbPath = path.join(DB_DIR, `search-${locale}.db`);

    if (fs.existsSync(dbPath)) {
        fs.unlinkSync(dbPath);
    }

    const db = new Database(dbPath);
    db.exec(`
        CREATE VIRTUAL TABLE search_index USING fts5(
            doc_id,
            title,
            parent_title,
            url,
            parent_url,
            icon,
            search_text,
            tokenize='porter unicode61'
        );
    `);

    const insertStmt = db.prepare(`
        INSERT INTO search_index (doc_id, title, parent_title, url, parent_url, icon, search_text)
        VALUES (?, ?, ?, ?, ?, ?, ?)
    `);

    const guides = getGuides();
    let indexedCount = 0;
    let skippedCount = 0;

    for (const guide of guides) {
        const guideTitle = guide.pageHeader || guide.name;
        const meta = getGuideMeta(guide.id);

        // Check if this guide has content for this locale
        const localeItemsPath = path.join(GUIDES_DIR, guide.id, 'items', locale);
        const hasLocaleContent = fs.existsSync(localeItemsPath) && fs.readdirSync(localeItemsPath).length > 0;

        // Skip non-default locales that have no translated content
        if (locale !== defaultLocale && !hasLocaleContent) {
            continue;
        }

        if (guide.id.startsWith('installation-') && guide.id !== 'installation') {
            try {
                let introPath = path.join(GUIDES_DIR, guide.id, 'items', locale, 'intro.md');
                if (!fs.existsSync(introPath)) {
                    introPath = path.join(GUIDES_DIR, guide.id, 'items', defaultLocale, 'intro.md');
                }
                if (!fs.existsSync(introPath)) {
                    introPath = guide.introPath;
                }

                const preview = fs.existsSync(introPath) ? marked(fs.readFileSync(introPath, 'utf8')) : '';
                let bodyWithChildren = preview.trim();

                for (const item of meta.itemsOrdered) {
                    try {
                        const builtItem = await buildGuideItemForMeta(guide, item, locale);
                        if (builtItem) {
                            bodyWithChildren += builtItem.content;
                        }
                    } catch (e) {
                        skippedCount++;
                    }
                }

                insertStmt.run(
                    guide.id,
                    meta.pageHeader || guide.name,
                    null,
                    '/' + createGuideLink(guide.id, locale),
                    null,
                    meta.icon ? '/images/guide-icons/' + meta.icon : null,
                    cleanSearchText(bodyWithChildren)
                );
                indexedCount++;
            } catch (e) {
                skippedCount++;
            }
        } else if (meta.itemsOrdered) {
            for (const item of meta.itemsOrdered) {
                try {
                    const builtItem = await buildGuideItemForMeta(guide, item, locale);
                    if (!builtItem) continue;

                    insertStmt.run(
                        `${guide.id}>${builtItem.id}`,
                        builtItem.title,
                        guideTitle,
                        builtItem.fullUrl,
                        createGuideLink(guide.id, locale),
                        meta.icon ? '/images/guide-icons/' + meta.icon : null,
                        cleanSearchText(builtItem.content)
                    );
                    indexedCount++;
                } catch (e) {
                    skippedCount++;
                }
            }
        }
    }

    db.close();

    const stats = fs.statSync(dbPath);
    return {
        locale,
        indexed: indexedCount,
        skipped: skippedCount,
        size: stats.size
    };
}

// Run the build
buildIndexForLocale(workerData.locale)
    .then(result => parentPort.postMessage({ success: true, ...result }))
    .catch(err => parentPort.postMessage({ success: false, locale: workerData.locale, error: err.message }));
