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
const OPENAI_MODEL = process.env.OPENAI_MODEL || 'gpt-5-mini';

// Prompt injection detection patterns
const INJECTION_PATTERNS = [
    /ignore\s+(all\s+)?(previous|prior|above|earlier)\s+(instructions?|prompts?|rules?|context)/i,
    /disregard\s+(all\s+)?(previous|prior|above|earlier)\s+(instructions?|prompts?|rules?|context)/i,
    /forget\s+(all\s+)?(previous|prior|above|earlier)\s+(instructions?|prompts?|rules?|context)/i,
    /override\s+(all\s+)?(previous|prior|above|earlier)\s+(instructions?|prompts?|rules?|context)/i,
    /you\s+are\s+(now\s+)?(a|an)\s+/i,
    /new\s+instructions?:/i,
    /system\s*prompt/i,
    /\bact\s+as\b/i,
    /\brole\s*play\b/i,
    /pretend\s+(you('re|are)|to\s+be)/i,
    /do\s+not\s+follow/i,
    /bypass\s+(the\s+)?(rules?|restrictions?|filters?)/i,
];

function containsPromptInjection(text) {
    if (!text || typeof text !== 'string') return false;
    return INJECTION_PATTERNS.some(pattern => pattern.test(text));
}

async function reorderResultsWithOpenAI(query, results) {
    if (!process.env.OPENAI_API_KEY) {
        console.log('OpenAI reranking skipped: no API key configured');
        return results;
    }
    if (results.length <= 1) {
        console.log('OpenAI reranking skipped: 0-1 results');
        return results;
    }

    // Check for prompt injection in query
    if (containsPromptInjection(query)) {
        console.log('Prompt injection detected in query, skipping OpenAI reranking');
        return results;
    }

    try {
        const resultsList = results.map(r => `[${r.id}] "${r.title}" (parent: "${r.parentTitle || 'none'}")`).join('\n');

        console.log(`OpenAI reranking: calling ${OPENAI_MODEL} for ${results.length} results`);
        const response = await axios.post('https://api.openai.com/v1/chat/completions', {
            model: OPENAI_MODEL,
            messages: [
                {
                    role: 'system',
                    content: 'You are a search result ranker for FastComments documentation. Given a search query and a list of results with IDs in brackets, return ONLY the IDs in order of relevance (most relevant first). Output only comma-separated IDs, nothing else. Example output: guide-auth,api-sso,ref-users'
                },
                {
                    role: 'user',
                    content: `Query: "${query}"\n\nResults:\n${resultsList}`
                }
            ],
            max_tokens: 200,
            temperature: 0
        }, {
            headers: {
                'Authorization': `Bearer ${process.env.OPENAI_API_KEY}`,
                'Content-Type': 'application/json'
            },
            timeout: 5000
        });
        console.log('OpenAI reranking: response received');

        const rankingText = response.data.choices?.[0]?.message?.content?.trim();
        if (!rankingText) {
            console.log('OpenAI reranking: no ranking text in response, returning original order');
            return results;
        }

        // Parse the ranking (expecting comma-separated IDs)
        const rankedIds = rankingText.split(',').map(s => s.trim()).filter(Boolean);

        if (rankedIds.length === 0) {
            console.log('OpenAI reranking: could not parse any IDs, returning original order');
            return results;
        }

        // Build a map for quick lookup
        const resultsById = new Map(results.map(r => [r.id, r]));

        // Reorder results based on ranking
        const reordered = [];
        const used = new Set();
        for (const id of rankedIds) {
            if (!used.has(id) && resultsById.has(id)) {
                reordered.push(resultsById.get(id));
                used.add(id);
            }
        }

        // Add any results not included in the ranking at the end
        for (const r of results) {
            if (!used.has(r.id)) {
                reordered.push(r);
            }
        }

        console.log(`OpenAI reranking: successfully reordered ${reordered.length} results`);
        return reordered;
    } catch (e) {
        console.error('OpenAI reranking failed:', e.message);
        if (e.response?.data) {
            console.error('OpenAI error response:', JSON.stringify(e.response.data));
        }
        console.log('OpenAI reranking: returning original order due to error');
        return results;
    }
}

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

app.get('/search', async (req, res) => {
    try {
        res.set('Access-Control-Allow-Origin', '*');
        res.set('Access-Control-Allow-Credentials', 'true');
        res.set('Access-Control-Allow-Headers', 'content-type');

        const locale = req.query.locale || defaultLocale;
        const query = req.query.query || '';

        console.log('Searching for', query, 'in locale', locale);

        let results = search(locale, query);

        console.log(results.length, 'results for', query);

        // Reorder results using OpenAI for better relevance
        results = await reorderResultsWithOpenAI(query, results);

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
