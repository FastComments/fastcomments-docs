/**
 * Parallel build script for SQLite FTS5 search indexes.
 * Uses worker threads to build multiple locale indexes concurrently.
 */

const { Worker } = require('worker_threads');
const os = require('os');
const path = require('path');
const fs = require('fs');
const { locales } = require('./locales');

const DB_DIR = path.join(__dirname, '..', 'db');
const WORKER_PATH = path.join(__dirname, 'build-search-index-worker.js');
// Each worker won't max out a core
const MAX_WORKERS = Math.ceil(os.cpus().length * 1.5);

// Ensure db directory exists
if (!fs.existsSync(DB_DIR)) {
    fs.mkdirSync(DB_DIR, { recursive: true });
}

const localeKeys = Object.keys(locales);
let completed = 0;
let totalIndexed = 0;
let totalSkipped = 0;
let activeWorkers = 0;
let failedCount = 0;
const results = [];
const queue = [...localeKeys];

console.log(`Building search indexes for ${localeKeys.length} locales using ${MAX_WORKERS} workers...`);
console.log('');

function startWorker(locale) {
    activeWorkers++;

    const worker = new Worker(WORKER_PATH, {
        workerData: { locale }
    });

    worker.on('message', (result) => {
        completed++;
        activeWorkers--;

        if (result.success) {
            totalIndexed += result.indexed;
            totalSkipped += result.skipped;
            const sizeMB = (result.size / 1024 / 1024).toFixed(2);
            console.log(`[${completed}/${localeKeys.length}] ${result.locale}: ${result.indexed} indexed, ${result.skipped} skipped, ${sizeMB} MB`);
            results.push(result);
        } else {
            failedCount++;
            console.error(`[${completed}/${localeKeys.length}] ${result.locale}: FAILED - ${result.error}`);
        }

        // Start next worker if there are more locales
        processQueue();
    });

    worker.on('error', (err) => {
        completed++;
        activeWorkers--;
        failedCount++;
        console.error(`[${completed}/${localeKeys.length}] ${locale}: WORKER ERROR - ${err.message}`);
        processQueue();
    });
}

function processQueue() {
    // Start workers up to MAX_WORKERS
    while (queue.length > 0 && activeWorkers < MAX_WORKERS) {
        const locale = queue.shift();
        startWorker(locale);
    }

    // Check if all done
    if (completed === localeKeys.length) {
        finishBuild();
    }
}

function finishBuild() {
    console.log('');
    console.log('='.repeat(50));
    console.log(`Done! Total indexed: ${totalIndexed}, Total skipped: ${totalSkipped}`);
    console.log('');

    // Show database sizes
    const files = fs.readdirSync(DB_DIR).filter(f => f.endsWith('.db'));
    let totalSize = 0;
    for (const file of files) {
        const stats = fs.statSync(path.join(DB_DIR, file));
        totalSize += stats.size;
    }
    console.log(`Total: ${(totalSize / 1024 / 1024).toFixed(2)} MB across ${files.length} databases`);

    if (failedCount > 0) {
        console.error(`\nERROR: ${failedCount} locale(s) failed to build.`);
        process.exit(1);
    }
}

// Start processing
processQueue();
