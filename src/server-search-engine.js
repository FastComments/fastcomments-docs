/**
 * Search server using SQLite FTS5 indexes.
 * Queries pre-built database files from the db/ folder.
 */

const Database = require('better-sqlite3');
const express = require('express');
const fs = require('fs');
const path = require('path');
const axios = require('axios');
const { locales, defaultLocale } = require('./locales');

const DB_DIR = path.join(__dirname, '..', 'db');

const AXIOS_CONFIG_NO_THROW = {
    validateStatus: function () {
        return true;
    }
};

// Search tracking collection
const searchCollection = new Map();

// Function to filter out prefix searches
function filterPrefixSearches(searches) {
    const searchArray = Array.from(searches.keys());
    const filtered = [];

    for (let i = 0; i < searchArray.length; i++) {
        let isPrefix = false;
        for (let j = 0; j < searchArray.length; j++) {
            if (i !== j && searchArray[j].startsWith(searchArray[i]) && searchArray[j] !== searchArray[i]) {
                isPrefix = true;
                break;
            }
        }
        if (!isPrefix) {
            filtered.push(searchArray[i]);
        }
    }

    return filtered;
}

// Process and track collected searches every 10 seconds
setInterval(async () => {
    if (searchCollection.size === 0) {
        return;
    }

    console.log('Processing collected searches...');
    const currentCollection = new Map(searchCollection);
    searchCollection.clear();

    for (const [tenantId, searches] of currentCollection) {
        const filteredSearches = filterPrefixSearches(searches);

        for (const searchInput of filteredSearches) {
            if (searchInput.includes('e2e-test')) {
                console.log('Skipping e2e test search:', searchInput);
                continue;
            }

            try {
                const response = await axios.post('https://fastcomments.com/docs-search/track-search-event?API_KEY='
                    + encodeURIComponent(process.env.SEARCH_API_KEY)
                    + '&tenantId='
                    + encodeURIComponent(tenantId)
                    + '&searchInput=' + encodeURIComponent(searchInput), AXIOS_CONFIG_NO_THROW
                );
                console.log('Tracked search event for:', searchInput, 'Response:', response.status);
            } catch (e) {
                console.error('Failed to track search:', searchInput, e);
            }
        }
    }
}, 10000);

// Cache database connections
const dbConnections = new Map();

function getDatabase(locale) {
    if (dbConnections.has(locale)) {
        return dbConnections.get(locale);
    }

    const dbPath = path.join(DB_DIR, `search-${locale}.db`);
    if (!fs.existsSync(dbPath)) {
        // Fall back to default locale if requested locale doesn't exist
        const fallbackPath = path.join(DB_DIR, `search-${defaultLocale}.db`);
        if (!fs.existsSync(fallbackPath)) {
            throw new Error(`No search database found for locale ${locale} or default ${defaultLocale}`);
        }
        console.log(`Using fallback database for locale ${locale}`);
        const db = new Database(fallbackPath, { readonly: true });
        dbConnections.set(locale, db);
        return db;
    }

    const db = new Database(dbPath, { readonly: true });
    dbConnections.set(locale, db);
    return db;
}

function search(locale, query, limit = 15) {
    const db = getDatabase(locale);

    // Escape special FTS5 characters and prepare query
    const sanitizedQuery = query
        .replace(/['"]/g, '') // Remove quotes
        .replace(/[-+*(){}[\]^~:]/g, ' ') // Replace special chars with spaces
        .trim()
        .split(/\s+/)
        .filter(term => term.length > 0)
        .map(term => `"${term}"*`) // Prefix matching
        .join(' OR ');

    if (!sanitizedQuery) {
        return [];
    }

    try {
        const stmt = db.prepare(`
            SELECT
                doc_id,
                title,
                parent_title,
                url,
                parent_url,
                icon,
                search_text,
                bm25(search_index) as score
            FROM search_index
            WHERE search_index MATCH ?
            ORDER BY bm25(search_index)
            LIMIT ?
        `);

        const results = stmt.all(sanitizedQuery, limit);

        return results.map(row => ({
            id: row.doc_id,
            title: row.title,
            parentTitle: row.parent_title,
            url: row.url,
            parentUrl: row.parent_url,
            icon: row.icon,
            searchText: row.search_text,
            score: -row.score // bm25 returns negative scores, lower is better
        }));
    } catch (e) {
        console.error('Search error:', e.message);
        return [];
    }
}

// Check if databases exist
const availableLocales = Object.keys(locales).filter(locale => {
    const dbPath = path.join(DB_DIR, `search-${locale}.db`);
    return fs.existsSync(dbPath);
});

if (availableLocales.length === 0) {
    console.error('No search databases found in', DB_DIR);
    console.error('Run "npm run build-search-index" first to create the databases.');
    process.exit(1);
}

console.log(`Found ${availableLocales.length} search databases`);

const app = express();
const port = 5001;

app.get('/search', (req, res) => {
    try {
        res.set('Access-Control-Allow-Origin', '*');
        res.set('Access-Control-Allow-Credentials', 'true');
        res.set('Access-Control-Allow-Headers', 'content-type');

        const locale = req.query.locale || defaultLocale;
        const query = req.query.query || '';
        const full = req.query.full === 'true';

        console.log('Searching for', query, 'in locale', locale);

        let results = search(locale, query);

        console.log(results.length, 'results for', query);

        // Only include body when full=true
        if (full) {
            results = results.map(({ searchText, ...rest }) => ({ ...rest, body: searchText }));
        } else {
            results = results.map(({ searchText, ...rest }) => rest);
        }

        res.send({
            status: 'success',
            results: results
        });

        // Collect search for batch processing
        const tenantId = req.query.tenantId || 'default';
        if (!searchCollection.has(tenantId)) {
            searchCollection.set(tenantId, new Map());
        }
        searchCollection.get(tenantId).set(query, Date.now());
    } catch (e) {
        console.error('Failed to handle search request', req.query.query, e);
        res.status(500).send({
            status: 'failed',
            error: e.message
        });
    }
});

app.options('/search', function (req, res) {
    res.set('Access-Control-Allow-Origin', '*');
    res.set('Access-Control-Allow-Credentials', 'true');
    res.set('Access-Control-Allow-Headers', 'content-type');
    res.status(200).end();
});

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('\nClosing database connections...');
    for (const [locale, db] of dbConnections) {
        db.close();
    }
    process.exit(0);
});

app.listen(port, () => {
    console.log(`Search server listening on port ${port}`);
    console.log(`Available locales: ${availableLocales.join(', ')}`);
});
