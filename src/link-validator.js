const MARKDOWN_LINK_REGEX = /\[([^\]]*)\]\(([^)]+)\)/g;
const CODE_BLOCK_REGEX = /```[\s\S]*?```|`[^`]+`/g;
const IMAGE_EXTENSIONS = ['.png', '.jpg', '.jpeg', '.gif', '.svg', '.webp', '.ico'];
const EXTERNAL_PREFIXES = ['http://', 'https://', 'mailto:', 'tel:'];

class LinkValidator {
    constructor() {
        this.guideItems = {}; // { guideId: Set<itemId> }
        this.errors = [];
    }

    /**
     * Register items for a guide (call once per guide when meta is loaded)
     */
    registerGuideItems(guideId, metaItemsOrdered) {
        const items = new Set();
        for (const item of metaItemsOrdered) {
            items.add(item.file.replace('.md', ''));
        }
        this.guideItems[guideId] = items;
    }

    /**
     * Validate links in already-read markdown content
     */
    validateContent(content, filePath, guideId) {
        const contentWithoutCode = content.replace(CODE_BLOCK_REGEX, '');
        const lines = content.split('\n');

        let match;
        while ((match = MARKDOWN_LINK_REGEX.exec(contentWithoutCode)) !== null) {
            const linkText = match[1];
            const link = match[2];

            if (this.shouldSkipLink(link)) continue;

            const lineNumber = this.findLineNumber(lines, match[0]);
            const { path: linkPath, anchor } = this.parseLink(link);

            if (linkPath === null && anchor !== null) {
                this.validateAnchor(filePath, guideId, anchor, linkText, link, lineNumber);
            } else if (linkPath && (linkPath.startsWith('./') || linkPath.startsWith('../'))) {
                this.validateRelativeLink(filePath, guideId, linkPath, anchor, linkText, link, lineNumber);
            } else if (linkPath && linkPath.startsWith('/guide-')) {
                this.validateAbsoluteGuideLink(filePath, linkPath, anchor, linkText, link, lineNumber);
            }
        }
    }

    parseLink(link) {
        const hashIndex = link.indexOf('#');
        if (hashIndex === -1) return { path: link, anchor: null };
        if (hashIndex === 0) return { path: null, anchor: link.substring(1) };
        return { path: link.substring(0, hashIndex), anchor: link.substring(hashIndex + 1) };
    }

    shouldSkipLink(link) {
        const lower = link.toLowerCase();
        for (const prefix of EXTERNAL_PREFIXES) {
            if (lower.startsWith(prefix)) return true;
        }
        for (const ext of IMAGE_EXTENSIONS) {
            if (lower.endsWith(ext)) return true;
        }
        return false;
    }

    findLineNumber(lines, matchText) {
        for (let i = 0; i < lines.length; i++) {
            if (lines[i].includes(matchText)) return i + 1;
        }
        return 1;
    }

    validateAnchor(filePath, guideId, anchor, linkText, link, lineNumber) {
        const items = this.guideItems[guideId];
        if (!items || !items.has(anchor)) {
            const available = items ? Array.from(items).slice(0, 5).join(', ') : 'none';
            this.errors.push({
                filePath, lineNumber, linkText, link,
                issue: `Anchor '${anchor}' not found in guide '${guideId}'`,
                fix: `Available: ${available}${items && items.size > 5 ? ', ...' : ''}`
            });
        }
    }

    validateRelativeLink(filePath, guideId, linkPath, anchor, linkText, link, lineNumber) {
        const targetItemId = linkPath.replace(/^\.\//, '').replace(/^\.\.\//, '').replace(/\.md$/, '');
        const items = this.guideItems[guideId];

        if (!items || !items.has(targetItemId)) {
            const available = items ? Array.from(items).slice(0, 5).join(', ') : 'none';
            this.errors.push({
                filePath, lineNumber, linkText, link,
                issue: `Item '${targetItemId}' not found in guide '${guideId}'`,
                fix: `Use anchor: (#${targetItemId})\n       Available: ${available}${items && items.size > 5 ? ', ...' : ''}`
            });
        } else if (anchor && !items.has(anchor)) {
            const available = Array.from(items).slice(0, 5).join(', ');
            this.errors.push({
                filePath, lineNumber, linkText, link,
                issue: `Anchor '${anchor}' not found in guide '${guideId}'`,
                fix: `Available: ${available}${items.size > 5 ? ', ...' : ''}`
            });
        }
    }

    validateAbsoluteGuideLink(filePath, linkPath, anchor, linkText, link, lineNumber) {
        let guideId = linkPath.replace(/^\/guide-/, '').replace(/\.html$/, '');
        const localeMatch = guideId.match(/^(.+)-([a-z]{2})$/);
        if (localeMatch) guideId = localeMatch[1];

        const items = this.guideItems[guideId];
        if (!items) {
            const available = Object.keys(this.guideItems).slice(0, 5).join(', ');
            this.errors.push({
                filePath, lineNumber, linkText, link,
                issue: `Guide '${guideId}' not found`,
                fix: `Available: ${available}${Object.keys(this.guideItems).length > 5 ? ', ...' : ''}`
            });
        } else if (anchor && !items.has(anchor)) {
            const available = Array.from(items).slice(0, 5).join(', ');
            this.errors.push({
                filePath, lineNumber, linkText, link,
                issue: `Anchor '${anchor}' not found in guide '${guideId}'`,
                fix: `Available: ${available}${items.size > 5 ? ', ...' : ''}`
            });
        }
    }

    hasErrors() {
        return this.errors.length > 0;
    }

    printErrors() {
        if (this.errors.length === 0) return;

        console.log('\n================================================================================');
        console.log(`LINK VALIDATION FAILED - Found ${this.errors.length} invalid link${this.errors.length > 1 ? 's' : ''}`);
        console.log('================================================================================\n');

        for (const err of this.errors) {
            console.log(`File:   ${err.filePath}:${err.lineNumber}`);
            console.log(`Link:   [${err.linkText}](${err.link})`);
            console.log(`Issue:  ${err.issue}`);
            if (err.fix) console.log(`\n  ${err.fix}`);
            console.log('\n--------------------------------------------------------------------------------\n');
        }

        console.log('================================================================================');
    }
}

// Singleton instance
const linkValidator = new LinkValidator();

module.exports = { linkValidator };
