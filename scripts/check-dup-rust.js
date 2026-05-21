#!/usr/bin/env node
// Run jscpd against the Rust workspace and fail non-zero if duplication
// exceeds the configured threshold. jscpd 4.2.3's own `--exitCode 1`
// flag is broken (the threshold breach raises an exception that the
// async wrapper in `jscpd/dist/bin/jscpd.js` swallows, exiting 0). We
// drive jscpd ourselves, parse the JSON report, and gate on the
// percentage so the build actually fails when it should.

const { spawnSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const REPO_ROOT = path.resolve(__dirname, '..');
const CONFIG = path.join(REPO_ROOT, '.jscpd.json');
const OUT_DIR = path.join(REPO_ROOT, '.jscpd-report');

if (!fs.existsSync(CONFIG)) {
    console.error(`missing config: ${CONFIG}`);
    process.exit(2);
}
const cfg = JSON.parse(fs.readFileSync(CONFIG, 'utf8'));
const threshold = typeof cfg.threshold === 'number' ? cfg.threshold : 3;

fs.mkdirSync(OUT_DIR, { recursive: true });

// `--reporters json,console` lets the developer see the per-clone
// listing in their terminal AND gives us a machine-readable report
// to gate on. We honor the rest of .jscpd.json (pattern, ignore,
// threshold, etc.) by passing nothing else — jscpd auto-loads it.
const result = spawnSync(
    'npx',
    ['--no-install', 'jscpd', '--reporters', 'json,console', '--output', OUT_DIR],
    { cwd: REPO_ROOT, stdio: 'inherit' }
);
if (result.error) {
    console.error(`jscpd spawn failed: ${result.error.message}`);
    process.exit(2);
}

const reportPath = path.join(OUT_DIR, 'jscpd-report.json');
if (!fs.existsSync(reportPath)) {
    console.error(`jscpd produced no report at ${reportPath}`);
    process.exit(2);
}
const report = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
const stats = report.statistics && report.statistics.total;
if (!stats) {
    console.error('jscpd report missing statistics.total');
    process.exit(2);
}
const pct = stats.percentage;
const lines = stats.duplicatedLines;
const clones = stats.clones;
console.log(
    `\njscpd: ${pct.toFixed(2)}% duplicated (${lines} lines across ${clones} clones); threshold=${threshold}%`
);
if (pct > threshold) {
    console.error(
        `FAIL: rust duplication ${pct.toFixed(2)}% exceeds threshold ${threshold}%`
    );
    process.exit(1);
}
console.log('OK: rust duplication within threshold');
