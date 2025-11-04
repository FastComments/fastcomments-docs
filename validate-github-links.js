#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');

/**
 * Validate GitHub links in generated SDK documentation
 * Fetches URLs with concurrency of 10 and checks for 404s
 */
class GitHubLinkValidator {
    constructor() {
        this.concurrency = 10;
        this.activeRequests = 0;
        this.queue = [];
        this.results = {
            valid: [],
            invalid: [],
            errors: []
        };
    }

    /**
     * Extract all GitHub URLs from markdown file
     * @param {string} filePath - Path to markdown file
     * @returns {Array<{url: string, file: string, line: number}>}
     */
    extractGitHubUrls(filePath) {
        const content = fs.readFileSync(filePath, 'utf8');
        const lines = content.split('\n');
        const urls = [];

        // Match GitHub URLs for source files
        const githubUrlRegex = /https:\/\/github\.com\/[^\s)]+\.(java|php|py|rs)/g;

        lines.forEach((line, index) => {
            let match;
            while ((match = githubUrlRegex.exec(line)) !== null) {
                urls.push({
                    url: match[0],
                    file: filePath,
                    line: index + 1
                });
            }
        });

        return urls;
    }

    /**
     * Find all generated markdown files
     * @returns {Array<string>}
     */
    findGeneratedMarkdownFiles() {
        const guidesDir = path.join(__dirname, 'src/content/guides');
        const files = [];

        const walkDir = (dir) => {
            const entries = fs.readdirSync(dir, { withFileTypes: true });

            for (const entry of entries) {
                const fullPath = path.join(dir, entry.name);

                if (entry.isDirectory()) {
                    walkDir(fullPath);
                } else if (entry.isFile() && entry.name.endsWith('-generated.md')) {
                    files.push(fullPath);
                }
            }
        };

        walkDir(guidesDir);
        return files;
    }

    /**
     * Fetch URL and check status code
     * @param {string} url - URL to check
     * @returns {Promise<{url: string, statusCode: number}>}
     */
    fetchUrl(url) {
        return new Promise((resolve) => {
            const request = https.get(url, {
                headers: {
                    'User-Agent': 'FastComments-Link-Validator/1.0'
                },
                timeout: 10000
            }, (res) => {
                // Drain the response to free up memory
                res.resume();

                resolve({
                    url,
                    statusCode: res.statusCode
                });
            });

            request.on('error', (err) => {
                resolve({
                    url,
                    statusCode: null,
                    error: err.message
                });
            });

            request.on('timeout', () => {
                request.destroy();
                resolve({
                    url,
                    statusCode: null,
                    error: 'Request timeout'
                });
            });
        });
    }

    /**
     * Process next item in queue
     */
    async processNext() {
        if (this.queue.length === 0 || this.activeRequests >= this.concurrency) {
            return;
        }

        const item = this.queue.shift();
        this.activeRequests++;

        try {
            const result = await this.fetchUrl(item.url);

            if (result.error) {
                this.results.errors.push({
                    ...item,
                    error: result.error
                });
                console.error(`❌ ERROR: ${item.url}`);
                console.error(`   File: ${item.file}:${item.line}`);
                console.error(`   Error: ${result.error}`);
            } else if (result.statusCode === 404) {
                this.results.invalid.push({
                    ...item,
                    statusCode: result.statusCode
                });
                console.error(`❌ 404: ${item.url}`);
                console.error(`   File: ${item.file}:${item.line}`);
            } else if (result.statusCode === 200) {
                this.results.valid.push({
                    ...item,
                    statusCode: result.statusCode
                });
                console.log(`✅ ${item.url}`);
            } else {
                // Other status codes (301, 302, etc.) - might want to track these
                this.results.valid.push({
                    ...item,
                    statusCode: result.statusCode
                });
                console.log(`⚠️  ${result.statusCode}: ${item.url}`);
            }
        } catch (error) {
            this.results.errors.push({
                ...item,
                error: error.message
            });
            console.error(`❌ ERROR: ${item.url}`);
            console.error(`   File: ${item.file}:${item.line}`);
            console.error(`   Error: ${error.message}`);
        } finally {
            this.activeRequests--;
            // Process next item
            setImmediate(() => this.processNext());
        }
    }

    /**
     * Validate all URLs with controlled concurrency
     * @param {Array<{url: string, file: string, line: number}>} urls
     */
    async validateUrls(urls) {
        // Deduplicate URLs (same URL may appear multiple times)
        const uniqueUrls = new Map();
        for (const item of urls) {
            if (!uniqueUrls.has(item.url)) {
                uniqueUrls.set(item.url, item);
            }
        }

        this.queue = Array.from(uniqueUrls.values());
        const totalUrls = this.queue.length;

        console.log(`\n🔍 Validating ${totalUrls} unique GitHub URLs with concurrency=${this.concurrency}...\n`);

        // Start initial batch of requests
        for (let i = 0; i < this.concurrency && this.queue.length > 0; i++) {
            this.processNext();
        }

        // Wait for all requests to complete
        while (this.activeRequests > 0 || this.queue.length > 0) {
            await new Promise(resolve => setTimeout(resolve, 100));
        }
    }

    /**
     * Print summary report
     */
    printSummary() {
        console.log('\n' + '='.repeat(80));
        console.log('VALIDATION SUMMARY');
        console.log('='.repeat(80));
        console.log(`✅ Valid URLs: ${this.results.valid.length}`);
        console.log(`❌ Invalid URLs (404): ${this.results.invalid.length}`);
        console.log(`❌ Errors: ${this.results.errors.length}`);
        console.log('='.repeat(80));

        if (this.results.invalid.length > 0) {
            console.log('\n❌ INVALID URLs (404):');
            for (const item of this.results.invalid) {
                console.log(`\n  ${item.url}`);
                console.log(`  File: ${item.file}:${item.line}`);
            }
        }

        if (this.results.errors.length > 0) {
            console.log('\n❌ ERRORS:');
            for (const item of this.results.errors) {
                console.log(`\n  ${item.url}`);
                console.log(`  File: ${item.file}:${item.line}`);
                console.log(`  Error: ${item.error}`);
            }
        }

        console.log('\n');
    }

    /**
     * Run validation
     */
    async run() {
        console.log('FastComments SDK Documentation Link Validator');
        console.log('='.repeat(80));

        // Find all generated markdown files
        const markdownFiles = this.findGeneratedMarkdownFiles();
        console.log(`Found ${markdownFiles.length} generated markdown files`);

        // Extract all GitHub URLs
        const allUrls = [];
        for (const file of markdownFiles) {
            const urls = this.extractGitHubUrls(file);
            allUrls.push(...urls);
        }

        console.log(`Found ${allUrls.length} total GitHub URLs`);

        // Validate URLs
        await this.validateUrls(allUrls);

        // Print summary
        this.printSummary();

        // Exit with error code if there are invalid URLs or errors
        if (this.results.invalid.length > 0 || this.results.errors.length > 0) {
            process.exit(1);
        }
    }
}

// Run if called directly
if (require.main === module) {
    const validator = new GitHubLinkValidator();
    validator.run().catch(err => {
        console.error('Fatal error:', err);
        process.exit(1);
    });
}

module.exports = GitHubLinkValidator;
