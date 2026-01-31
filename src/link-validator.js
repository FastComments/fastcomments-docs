const MARKDOWN_LINK_REGEX = /\[([^\]]*)\]\(([^)]+)\)/g;
const CODE_BLOCK_REGEX = /```[\s\S]*?```|`[^`]+`/g;
const IMAGE_EXTENSIONS = ['.png', '.jpg', '.jpeg', '.gif', '.svg', '.webp', '.ico'];
const EXTERNAL_PREFIXES = ['http://', 'https://', 'mailto:', 'tel:'];

class LinkValidator {
    constructor() {
        this.guideItems = {}; // { guideId: Set<itemId> }
    }

    registerGuideItems(guideId, metaItemsOrdered) {
        const items = new Set();
        for (const item of metaItemsOrdered) {
            items.add(item.file.replace('.md', ''));
        }
        this.guideItems[guideId] = items;
    }

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

    fail(filePath, lineNumber, linkText, link, issue, available) {
        let msg = `\nInvalid link at ${filePath}:${lineNumber}\n`;
        msg += `  Link: [${linkText}](${link})\n`;
        msg += `  ${issue}\n`;
        if (available) msg += `  Available: ${available}\n`;
        throw new Error(msg);
    }

    validateAnchor(filePath, guideId, anchor, linkText, link, lineNumber) {
        const items = this.guideItems[guideId];
        if (!items || !items.has(anchor)) {
            const available = items ? Array.from(items).slice(0, 5).join(', ') + (items.size > 5 ? ', ...' : '') : 'none';
            this.fail(filePath, lineNumber, linkText, link, `Anchor '${anchor}' not found in guide '${guideId}'`, available);
        }
    }

    validateRelativeLink(filePath, guideId, linkPath, anchor, linkText, link, lineNumber) {
        const targetItemId = linkPath.replace(/^\.\//, '').replace(/^\.\.\//, '').replace(/\.md$/, '');
        const items = this.guideItems[guideId];

        if (!items || !items.has(targetItemId)) {
            const available = items ? Array.from(items).slice(0, 5).join(', ') + (items.size > 5 ? ', ...' : '') : 'none';
            this.fail(filePath, lineNumber, linkText, link, `Item '${targetItemId}' not found in guide '${guideId}'`, available);
        } else if (anchor && !items.has(anchor)) {
            const available = Array.from(items).slice(0, 5).join(', ') + (items.size > 5 ? ', ...' : '');
            this.fail(filePath, lineNumber, linkText, link, `Anchor '${anchor}' not found in guide '${guideId}'`, available);
        }
    }

    validateAbsoluteGuideLink(filePath, linkPath, anchor, linkText, link, lineNumber) {
        let guideId = linkPath.replace(/^\/guide-/, '').replace(/\.html$/, '');
        const localeMatch = guideId.match(/^(.+)-([a-z]{2})$/);
        if (localeMatch) guideId = localeMatch[1];

        const items = this.guideItems[guideId];
        if (!items) {
            const available = Object.keys(this.guideItems).slice(0, 5).join(', ') + (Object.keys(this.guideItems).length > 5 ? ', ...' : '');
            this.fail(filePath, lineNumber, linkText, link, `Guide '${guideId}' not found`, available);
        } else if (anchor && !items.has(anchor)) {
            const available = Array.from(items).slice(0, 5).join(', ') + (items.size > 5 ? ', ...' : '');
            this.fail(filePath, lineNumber, linkText, link, `Anchor '${anchor}' not found in guide '${guideId}'`, available);
        }
    }
}

const linkValidator = new LinkValidator();

module.exports = { linkValidator };
