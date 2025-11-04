const fs = require('fs');
const path = require('path');
const BaseDocGenerator = require('./base-generator');

/**
 * Documentation generator that parses README.md and other markdown files
 * This is a fallback generator that works for most SDKs
 */
class ReadmeDocGenerator extends BaseDocGenerator {
    /**
     * Generate documentation from README and other markdown files
     * @returns {Promise<GeneratedDocs>}
     */
    async generate() {
        const sections = [];

        // Try to find and parse README
        const readmePath = this.findReadme();
        if (readmePath) {
            const readmeContent = fs.readFileSync(readmePath, 'utf8');
            const parsedSections = this.parseReadme(readmeContent);
            sections.push(...parsedSections);
        } else {
            console.warn(`No README found for ${this.sdk.id}`);
        }

        // Look for additional documentation files
        const docsDir = path.join(this.repoPath, 'docs');
        if (fs.existsSync(docsDir)) {
            const docSections = this.parseDocsDirectory(docsDir);
            sections.push(...docSections);
        }

        // Create intro and conclusion
        const intro = this.generateIntro();
        const conclusion = this.generateConclusion();

        return {
            intro,
            conclusion,
            sections: sections.length > 0 ? sections : this.generateFallbackSections()
        };
    }

    /**
     * Find README file (case-insensitive)
     * @returns {string|null}
     */
    findReadme() {
        const possibleNames = ['README.md', 'Readme.md', 'readme.md', 'README.MD', 'README'];
        for (const name of possibleNames) {
            const filePath = path.join(this.repoPath, name);
            if (fs.existsSync(filePath)) {
                return filePath;
            }
        }
        return null;
    }

    /**
     * Parse README into sections
     * @param {string} content - README content
     * @returns {Array<DocSection>}
     */
    parseReadme(content) {
        // Remove front matter if present
        content = this.removeFrontMatter(content);

        // Split by h2 headers (##)
        const sections = [];
        const headerRegex = /^##\s+(.+)$/gm;
        const matches = [...content.matchAll(headerRegex)];

        if (matches.length === 0) {
            // No sections found, treat entire README as one section
            // Convert relative links to absolute repository URLs
            const convertedContent = this.convertRelativeLinks(
                content,
                this.sdk.repo,
                this.sdk.branch,
                '' // README is at the root of the repo
            );

            return [{
                name: 'Overview',
                file: 'overview-readme-generated.md',
                content: convertedContent,
                subCat: 'Getting Started',
                type: 'readme'
            }];
        }

        for (let i = 0; i < matches.length; i++) {
            const match = matches[i];
            const sectionTitle = match[1];
            const startIndex = match.index;
            const endIndex = i < matches.length - 1 ? matches[i + 1].index : content.length;

            // Extract section content (including the header)
            let sectionContent = content.substring(startIndex, endIndex).trim();

            // Remove the first H2 heading since it will be displayed by the frontend using the section name
            // This prevents duplicate headings
            sectionContent = sectionContent.replace(/^##\s+.+\n/, '').trim();

            // Convert relative links to absolute repository URLs
            sectionContent = this.convertRelativeLinks(
                sectionContent,
                this.sdk.repo,
                this.sdk.branch,
                '' // README is at the root of the repo
            );

            // Determine subcategory based on section title
            const subCat = this.categorizeSection(sectionTitle);

            // Skip certain sections that are not useful in docs
            if (this.shouldSkipSection(sectionTitle)) {
                continue;
            }

            sections.push({
                name: sectionTitle,
                file: this.sanitizeFilename(sectionTitle) + '-readme-generated.md',
                content: sectionContent,
                subCat,
                type: 'readme'
            });
        }

        return sections;
    }

    /**
     * Categorize section based on title
     * @param {string} title - Section title
     * @returns {string}
     */
    categorizeSection(title) {
        const lower = title.toLowerCase();

        if (lower.includes('install') || lower.includes('setup') || lower.includes('getting started')) {
            return 'Getting Started';
        }
        if (lower.includes('usage') || lower.includes('example') || lower.includes('quickstart')) {
            return 'Usage';
        }
        if (lower.includes('api') || lower.includes('reference') || lower.includes('method')) {
            return 'API Reference';
        }
        if (lower.includes('config') || lower.includes('option')) {
            return 'Configuration';
        }
        if (lower.includes('auth') || lower.includes('security')) {
            return 'Authentication';
        }
        if (lower.includes('contribut')) {
            return 'Contributing';
        }
        if (lower.includes('license')) {
            return 'License';
        }

        return 'Documentation';
    }

    /**
     * Check if section should be skipped
     * @param {string} title - Section title
     * @returns {boolean}
     */
    shouldSkipSection(title) {
        const lower = title.toLowerCase();
        const skipKeywords = ['license', 'contributing', 'changelog', 'contributors'];
        return skipKeywords.some(keyword => lower.includes(keyword));
    }

    /**
     * Parse additional documentation directory
     * @param {string} docsDir - Path to docs directory
     * @returns {Array<DocSection>}
     */
    parseDocsDirectory(docsDir) {
        const sections = [];

        try {
            const files = fs.readdirSync(docsDir);

            for (const file of files) {
                if (!file.endsWith('.md')) {
                    continue;
                }

                const filePath = path.join(docsDir, file);
                let content = fs.readFileSync(filePath, 'utf8');

                // Remove front matter
                content = this.removeFrontMatter(content);

                // Convert relative links to absolute repository URLs
                content = this.convertRelativeLinks(
                    content,
                    this.sdk.repo,
                    this.sdk.branch,
                    'docs/' // Files are in the docs/ directory
                );

                // Extract title from filename or content
                const title = this.extractTitle(content) ||
                             file.replace('.md', '').replace(/-/g, ' ');

                sections.push({
                    name: title,
                    file: this.sanitizeFilename(title) + '-readme-generated.md',
                    content: content,
                    subCat: 'Documentation',
                    type: 'readme'
                });
            }
        } catch (e) {
            console.error(`Error parsing docs directory:`, e.message);
        }

        return sections;
    }

    /**
     * Generate intro markdown
     * @returns {string}
     */
    generateIntro() {
        return `This is the official ${this.sdk.name} for FastComments.

${this.sdk.description || ''}

## Repository

[View on GitHub](${this.sdk.repo})
`;
    }

    /**
     * Generate conclusion markdown
     * @returns {string}
     */
    generateConclusion() {
        return `## Need Help?

If you encounter any issues or have questions about the ${this.sdk.name}, please:

- [Open an issue on GitHub](${this.sdk.repo}/issues)
- [Contact FastComments Support](https://fastcomments.com/auth/my-account/help)

## Contributing

Contributions are welcome! Please visit the [GitHub repository](${this.sdk.repo}) for contribution guidelines.
`;
    }

    /**
     * Generate fallback sections if no documentation found
     * @returns {Array<DocSection>}
     */
    generateFallbackSections() {
        return [{
            name: 'Overview',
            content: `# ${this.sdk.name}

${this.sdk.description || 'Official SDK for the FastComments API.'}

For more information, please visit the [GitHub repository](${this.sdk.repo}).
`,
            subCat: 'Getting Started'
        }];
    }
}

module.exports = ReadmeDocGenerator;
