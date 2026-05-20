#!/usr/bin/env node
/**
 * Regression harness: hit two /search endpoints with a canned query set,
 * compare top-15 result-set membership and top-3 ordering.
 *
 * Intended use during the Node -> Rust migration:
 *   * baseline = current production (Node + SQLite FTS5 on port 5001)
 *   * candidate = Rust + Tantivy server on a different port
 *
 * Run both servers concurrently, then:
 *   node scripts/search-regression.js \
 *     --baseline http://127.0.0.1:5001 \
 *     --candidate http://127.0.0.1:5002 \
 *     --queries scripts/search-regression-queries.json \
 *     --min-overlap 0.6
 *
 * Each query is sent with ?nollm=true so OpenAI reranking doesn't inject
 * non-determinism. Exits non-zero if any query falls below --min-overlap on
 * top-15 membership.
 */

const fs = require('fs');
const path = require('path');
const http = require('http');
const https = require('https');
const { URL } = require('url');

function parseArgs(argv) {
    const args = {
        baseline: 'http://127.0.0.1:5001',
        candidate: 'http://127.0.0.1:5002',
        queries: path.join(__dirname, 'search-regression-queries.json'),
        minOverlap: 0.6,
        verbose: false,
    };
    for (let i = 0; i < argv.length; i++) {
        const a = argv[i];
        if (a === '--baseline') args.baseline = argv[++i];
        else if (a === '--candidate') args.candidate = argv[++i];
        else if (a === '--queries') args.queries = argv[++i];
        else if (a === '--min-overlap') args.minOverlap = parseFloat(argv[++i]);
        else if (a === '--verbose' || a === '-v') args.verbose = true;
        else if (a === '--help' || a === '-h') {
            console.error('Usage: search-regression.js [--baseline URL] [--candidate URL] [--queries JSON] [--min-overlap 0-1] [-v]');
            process.exit(2);
        }
    }
    return args;
}

function get(baseUrl, query, locale) {
    return new Promise((resolve, reject) => {
        const url = new URL('/search', baseUrl);
        url.searchParams.set('query', query);
        url.searchParams.set('locale', locale);
        url.searchParams.set('nollm', 'true');
        const mod = url.protocol === 'https:' ? https : http;
        const req = mod.get(url, (res) => {
            let buf = '';
            res.on('data', (chunk) => { buf += chunk; });
            res.on('end', () => {
                try {
                    resolve(JSON.parse(buf));
                } catch (e) {
                    reject(new Error(`Failed to parse ${url}: ${e.message}\n${buf.slice(0, 200)}`));
                }
            });
        });
        req.on('error', reject);
        req.setTimeout(15000, () => req.destroy(new Error('timeout')));
    });
}

function overlap(a, b) {
    const sa = new Set(a);
    const sb = new Set(b);
    if (sa.size === 0 && sb.size === 0) return 1;
    if (sa.size === 0 || sb.size === 0) return 0;
    let inter = 0;
    for (const x of sa) if (sb.has(x)) inter++;
    return inter / Math.max(sa.size, sb.size);
}

function topNIds(results, n) {
    return (results || []).slice(0, n).map((r) => r.id);
}

async function main() {
    const args = parseArgs(process.argv.slice(2));
    const cfg = JSON.parse(fs.readFileSync(args.queries, 'utf8'));
    const queries = cfg.queries || [];
    console.log(`Comparing ${queries.length} queries`);
    console.log(`  baseline:  ${args.baseline}`);
    console.log(`  candidate: ${args.candidate}`);
    console.log(`  min-overlap (top-15): ${args.minOverlap}`);
    console.log('');

    const failures = [];
    let totalOverlap = 0;
    let totalTop3Match = 0;

    for (const entry of queries) {
        const q = entry.query;
        const loc = entry.locale || 'en';
        let baseline, candidate;
        try {
            baseline = await get(args.baseline, q, loc);
            candidate = await get(args.candidate, q, loc);
        } catch (e) {
            console.log(`[ERR] ${loc} ${q}: ${e.message}`);
            failures.push({ query: q, locale: loc, error: e.message });
            continue;
        }
        const bIds = topNIds(baseline.results, 15);
        const cIds = topNIds(candidate.results, 15);
        const ov = overlap(bIds, cIds);
        const top3 = JSON.stringify(bIds.slice(0, 3)) === JSON.stringify(cIds.slice(0, 3));
        totalOverlap += ov;
        if (top3) totalTop3Match += 1;
        const flag = ov < args.minOverlap ? 'FAIL' : 'OK  ';
        const top3Flag = top3 ? 'top3=' : 'top3*';
        console.log(`${flag} ov=${ov.toFixed(2)} ${top3Flag} ${loc.padEnd(6)} ${q}`);
        if (args.verbose || ov < args.minOverlap) {
            console.log('     baseline:', bIds.slice(0, 5));
            console.log('     candidate:', cIds.slice(0, 5));
        }
        if (ov < args.minOverlap) {
            failures.push({ query: q, locale: loc, overlap: ov, baseline: bIds, candidate: cIds });
        }
        // Expected IDs check (optional per query entry).
        if (entry.expected_top_ids) {
            for (const expected of entry.expected_top_ids) {
                if (!cIds.includes(expected)) {
                    console.log(`     MISSING expected id: ${expected}`);
                    failures.push({ query: q, locale: loc, missingExpected: expected });
                }
            }
        }
    }

    const avgOv = totalOverlap / Math.max(queries.length, 1);
    const top3Rate = totalTop3Match / Math.max(queries.length, 1);
    console.log('');
    console.log(`=== overall ===`);
    console.log(`avg overlap (top-15): ${avgOv.toFixed(3)}`);
    console.log(`top-3 exact match rate: ${(top3Rate * 100).toFixed(1)}%`);
    console.log(`failures: ${failures.length}`);

    process.exit(failures.length > 0 ? 1 : 0);
}

main().catch((e) => {
    console.error(e);
    process.exit(2);
});
