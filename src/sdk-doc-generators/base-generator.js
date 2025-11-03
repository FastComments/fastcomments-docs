const fs = require('fs');
const path = require('path');

/**
 * Base class for SDK documentation generators
 *
 * Each language-specific generator should extend this class
 * and implement the generate() method
 */

/**
 * @typedef {Object} DocSection
 * @property {string} name - Section name (used for meta.json)
 * @property {string} content - Markdown content
 * @property {string} subCat - Subcategory for grouping
 * @property {string} [sidebarItemClasses] - Optional CSS classes for sidebar
 */

/**
 * @typedef {Object} GeneratedDocs
 * @property {string} intro - Introduction markdown
 * @property {string} conclusion - Conclusion markdown
 * @property {Array<DocSection>} sections - Documentation sections
 */

class BaseDocGenerator {
    /**
     * @param {Object} sdk - SDK configuration
     * @param {string} repoPath - Path to checked out repository
     */
    constructor(sdk, repoPath) {
        this.sdk = sdk;
        this.repoPath = repoPath;
    }

    /**
     * Generate documentation from the SDK repository
     * This method should be overridden by subclasses
     *
     * @returns {Promise<GeneratedDocs>}
     */
    async generate() {
        throw new Error('generate() must be implemented by subclass');
    }

    /**
     * Helper to sanitize filenames
     * @param {string} name - Original name
     * @returns {string} - Sanitized filename
     */
    sanitizeFilename(name) {
        return name
            .toLowerCase()
            .replace(/[^a-z0-9]+/g, '-')
            .replace(/^-+|-+$/g, '');
    }

    /**
     * Helper to extract title from markdown heading
     * @param {string} markdown - Markdown content
     * @returns {string|null} - Extracted title or null
     */
    extractTitle(markdown) {
        const match = markdown.match(/^#\s+(.+)$/m);
        return match ? match[1] : null;
    }

    /**
     * Helper to remove front matter from markdown
     * @param {string} markdown - Markdown content
     * @returns {string} - Markdown without front matter
     */
    removeFrontMatter(markdown) {
        const frontMatterRegex = /^---\n[\s\S]*?\n---\n/;
        return markdown.replace(frontMatterRegex, '');
    }

    /**
     * Helper to read file safely
     * @param {string} filePath - Path to file
     * @returns {string|null} - File contents or null if not found
     */
    readFileIfExists(filePath) {
        try {
            if (fs.existsSync(filePath)) {
                return fs.readFileSync(filePath, 'utf8');
            }
        } catch (e) {
            console.error(`Error reading file ${filePath}:`, e.message);
        }
        return null;
    }

    /**
     * Convert relative links in markdown to absolute repository URLs
     * @param {string} markdown - Markdown content
     * @param {string} repoUrl - Repository URL (e.g., https://github.com/FastComments/fastcomments-sdk-js)
     * @param {string} branch - Branch name (e.g., main)
     * @param {string} basePath - Base path relative to repo root (e.g., '' for root, 'docs/' for docs dir)
     * @returns {string} - Markdown with converted links
     */
    convertRelativeLinks(markdown, repoUrl, branch, basePath = '') {
        // Regex to match markdown links: [text](url)
        const linkRegex = /\[([^\]]+)\]\(([^)]+)\)/g;

        return markdown.replace(linkRegex, (match, text, url) => {
            // Skip absolute URLs, anchors, and root-relative paths
            if (url.startsWith('http://') ||
                url.startsWith('https://') ||
                url.startsWith('#') ||
                url.startsWith('/')) {
                return match;
            }

            // This is a relative link - convert it
            // Resolve the path relative to the base path
            let resolvedPath;
            if (url.startsWith('./') || url.startsWith('../')) {
                // Use path.join to properly resolve relative paths
                resolvedPath = path.posix.join(basePath, url);
            } else {
                // No ./ or ../ prefix, treat as relative to base path
                resolvedPath = path.posix.join(basePath, url);
            }

            // Normalize the path (remove ./ and resolve ../)
            const normalizedPath = path.posix.normalize(resolvedPath);

            // Build the absolute GitHub URL
            // Remove trailing .git from repo URL if present
            const cleanRepoUrl = repoUrl.replace(/\.git$/, '');
            const absoluteUrl = `${cleanRepoUrl}/blob/${branch}/${normalizedPath}`;

            return `[${text}](${absoluteUrl})`;
        });
    }
}

module.exports = BaseDocGenerator;
