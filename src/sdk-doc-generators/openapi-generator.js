const fs = require('fs');
const path = require('path');
const BaseDocGenerator = require('./base-generator');
const { convertFromPascal, parseSDKDocTable } = require('./naming-utils');

/**
 * Documentation generator that creates API reference from OpenAPI spec
 * and extracts code examples from generated markdown docs
 */
class OpenAPIDocGenerator extends BaseDocGenerator {
    /**
     * Generate API reference documentation from OpenAPI spec
     * @returns {Promise<GeneratedDocs>}
     */
    async generate() {
        const config = this.sdk.openApiConfig;
        if (!config) {
            throw new Error(`No openApiConfig found for ${this.sdk.id}`);
        }

        // Parse OpenAPI spec
        const spec = this.parseOpenAPISpec(config.specPath);
        if (!spec) {
            console.warn(`Could not parse OpenAPI spec for ${this.sdk.id}`);
            return { intro: '', conclusion: '', sections: [] };
        }

        // Parse SDK doc tables to build lookup maps for method names
        const methodLookup = this.buildMethodLookup(config);

        // Group operations by resource (tag)
        const operationsByResource = this.groupOperationsByResource(spec);

        // Generate sections for each operation
        const sections = [];
        for (const [resource, operations] of Object.entries(operationsByResource)) {
            for (const operation of operations) {
                const section = this.generateOperationSection(operation, resource, config, methodLookup);
                if (section) {
                    sections.push(section);
                }
            }
        }

        return {
            intro: '', // OpenAPI generator doesn't provide intro/conclusion
            conclusion: '',
            sections
        };
    }

    /**
     * Build method name lookup from SDK documentation tables
     * @param {Object} config - OpenAPI config
     * @returns {Map<string, string>} - Map from "METHOD /path" to SDK method name
     */
    buildMethodLookup(config) {
        const methodLookup = new Map();

        // Parse tables from all API class files
        const apiClasses = ['DefaultApi', 'PublicApi', 'HiddenApi'];

        for (const apiClass of apiClasses) {
            const classLookup = parseSDKDocTable(this.repoPath, config, apiClass);
            // Merge into main lookup
            for (const [key, value] of classLookup.entries()) {
                methodLookup.set(key, value);
            }
        }

        return methodLookup;
    }

    /**
     * Parse OpenAPI spec file
     * @param {string} specPath - Relative path to OpenAPI spec
     * @returns {Object|null}
     */
    parseOpenAPISpec(specPath) {
        const fullPath = path.join(this.repoPath, specPath);

        if (!fs.existsSync(fullPath)) {
            console.error(`OpenAPI spec not found: ${fullPath}`);
            return null;
        }

        try {
            const content = fs.readFileSync(fullPath, 'utf8');
            return JSON.parse(content);
        } catch (e) {
            console.error(`Error parsing OpenAPI spec: ${e.message}`);
            return null;
        }
    }

    /**
     * Group operations by resource (using tags)
     * @param {Object} spec - OpenAPI specification
     * @returns {Object<string, Array>} - Operations grouped by resource
     */
    groupOperationsByResource(spec) {
        const grouped = {};

        for (const [pathStr, pathItem] of Object.entries(spec.paths || {})) {
            for (const [method, operation] of Object.entries(pathItem)) {
                // Skip non-method keys like 'parameters', 'summary', etc.
                if (!['get', 'post', 'put', 'patch', 'delete', 'options', 'head'].includes(method)) {
                    continue;
                }

                // Get primary tag as resource name
                const tag = operation.tags?.[0] || 'default';

                // Skip Hidden API operations
                if (tag === 'Hidden') {
                    continue;
                }

                // Map Public to Misc Apis
                let resource = tag === 'default' ? this.inferResourceFromPath(pathStr) : tag;
                if (resource === 'Public') {
                    resource = 'Misc Apis';
                }

                if (!grouped[resource]) {
                    grouped[resource] = [];
                }

                grouped[resource].push({
                    operationId: operation.operationId,
                    summary: operation.summary,
                    description: operation.description,
                    method: method.toUpperCase(),
                    path: pathStr,
                    parameters: operation.parameters || [],
                    requestBody: operation.requestBody,
                    responses: operation.responses,
                    tags: operation.tags || []
                });
            }
        }

        return grouped;
    }

    /**
     * Infer resource name from API path
     * @param {string} path - API path
     * @returns {string}
     */
    inferResourceFromPath(path) {
        // Extract resource from path like /api/v1/comments -> comments
        const match = path.match(/\/api\/v\d+\/([^\/]+)/);
        if (match) {
            return match[1];
        }
        return 'api';
    }

    /**
     * Generate section for a single operation
     * @param {Object} operation - Operation details
     * @param {string} resource - Resource name
     * @param {Object} config - OpenAPI config
     * @param {Map<string, string>} methodLookup - Map from "METHOD /path" to SDK method name
     * @returns {DocSection|null}
     */
    generateOperationSection(operation, resource, config, methodLookup) {
        // Try to get actual method name from SDK docs first
        const lookupKey = `${operation.method} ${operation.path}`;
        let name = methodLookup.get(lookupKey);

        // Fallback to conversion if not found in lookup
        if (!name) {
            const namingConvention = this.sdk.namingConvention || 'camelCase';
            const operationId = operation.operationId || this.sanitizeFilename(operation.summary);
            name = convertFromPascal(operationId, namingConvention);
            console.warn(`Method not found in SDK docs for ${lookupKey}, using fallback: ${name}`);
        }

        // Extract code example from generated docs
        const codeExample = this.extractCodeExample(operation, config, name);

        // Generate markdown content
        const content = this.generateOperationMarkdown(operation, codeExample, name);

        // Categorize by resource for meta.json (no "API Reference -" prefix)
        const subCat = this.formatResourceName(resource);

        // Generate filename with -generated suffix
        const filename = this.sanitizeFilename(name) + '-generated.md';

        return {
            name,
            file: filename,
            content,
            subCat
        };
    }

    /**
     * Extract code example from generated markdown docs
     * @param {Object} operation - Operation details
     * @param {Object} config - OpenAPI config
     * @param {string} methodName - Actual SDK method name to search for
     * @returns {string|null}
     */
    extractCodeExample(operation, config, methodName) {
        const docsPath = path.join(this.repoPath, config.generatedDocsPath);

        if (!fs.existsSync(docsPath)) {
            console.warn(`Generated docs path not found: ${docsPath}`);
            return null;
        }

        // Find the API class file (e.g., DefaultApi.md, PublicApi.md)
        // Determine which file based on operation tags
        const apiClass = this.determineApiClass(operation);
        const docFile = config.docFilePattern.replace('{ApiClass}', apiClass);
        const docPath = path.join(docsPath, docFile);

        if (!fs.existsSync(docPath)) {
            console.warn(`Doc file not found: ${docPath}`);
            return null;
        }

        try {
            const docContent = fs.readFileSync(docPath, 'utf8');
            return this.parseCodeExampleFromMarkdown(docContent, methodName);
        } catch (e) {
            console.error(`Error reading doc file: ${e.message}`);
            return null;
        }
    }

    /**
     * Determine which API class the operation belongs to
     * @param {Object} operation - Operation details
     * @returns {string}
     */
    determineApiClass(operation) {
        const tags = operation.tags || [];

        // Map tags to API classes
        if (tags.includes('Public')) return 'PublicApi';
        if (tags.includes('Hidden')) return 'HiddenApi';

        return 'DefaultApi'; // Default for secured/authenticated endpoints
    }

    /**
     * Parse code example from generated markdown
     * @param {string} markdown - Markdown content
     * @param {string} methodName - Actual SDK method name to search for
     * @returns {string|null}
     */
    parseCodeExampleFromMarkdown(markdown, methodName) {
        if (!methodName) return null;

        const escapedMethodName = methodName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

        // Try multiple heading formats:
        // 1. Most SDKs: # **methodName**
        // 2. With backticks and parens: ## `methodName()`
        const patterns = [
            new RegExp(`^#+\\s*\\*\\*${escapedMethodName}\\*\\*\\s*$`, 'm'),
            new RegExp(`^#+\\s*\`${escapedMethodName}\\(\\)\`\\s*$`, 'm')
        ];

        let match = null;
        for (const pattern of patterns) {
            match = markdown.match(pattern);
            if (match) break;
        }

        if (!match) {
            return null;
        }

        // Extract content after the heading until next H1/H2 heading or end
        // Use \n#{1,2}\s\*\* to match only markdown headings (with bold), not code comments
        const startIndex = match.index + match[0].length;
        const nextHeadingMatch = markdown.substring(startIndex).match(/\n#{1,2}\s+\*\*/);
        const endIndex = nextHeadingMatch
            ? startIndex + nextHeadingMatch.index
            : markdown.length;

        const sectionContent = markdown.substring(startIndex, endIndex);

        // Extract first code block (look for ### Example section first)
        const exampleSectionMatch = sectionContent.match(/###\s*Example[\s\S]*?```[\w]*\n([\s\S]*?)```/);
        if (exampleSectionMatch) {
            return exampleSectionMatch[1].trim();
        }

        // Fall back to first code block if no Example section
        const codeBlockMatch = sectionContent.match(/```[\w]*\n([\s\S]*?)```/);
        if (codeBlockMatch) {
            return codeBlockMatch[1].trim();
        }

        return null;
    }

    /**
     * Generate markdown content for an operation
     * @param {Object} operation - Operation details
     * @param {string|null} codeExample - Extracted code example
     * @param {string} methodName - Actual SDK method name
     * @returns {string}
     */
    generateOperationMarkdown(operation, codeExample, methodName) {
        const lines = [];

        // HTTP method and path
        lines.push(`\`${operation.method} ${operation.path}\``);
        lines.push('');

        // Description
        if (operation.description) {
            lines.push(operation.description);
            lines.push('');
        }

        // Parameters
        if (operation.parameters && operation.parameters.length > 0) {
            lines.push('## Parameters');
            lines.push('');
            lines.push('| Name | Type | Location | Required | Description |');
            lines.push('|------|------|----------|----------|-------------|');

            for (const param of operation.parameters) {
                const name = param.name || '';
                const type = param.schema?.type || param.type || 'string';
                const location = param.in || '';
                const required = param.required ? 'Yes' : 'No';
                const description = (param.description || '').replace(/\|/g, '\\|');

                lines.push(`| ${name} | ${type} | ${location} | ${required} | ${description} |`);
            }
            lines.push('');
        }

        // Request Body
        if (operation.requestBody) {
            lines.push('## Request Body');
            lines.push('');
            const description = operation.requestBody.description;
            if (description) {
                lines.push(description);
                lines.push('');
            }

            const content = operation.requestBody.content;
            if (content) {
                const contentTypes = Object.keys(content);
                if (contentTypes.length > 0) {
                    lines.push(`Content-Type: \`${contentTypes[0]}\``);
                    lines.push('');
                }
            }
        }

        // Responses
        if (operation.responses) {
            lines.push('## Responses');
            lines.push('');

            for (const [statusCode, response] of Object.entries(operation.responses)) {
                lines.push(`### ${statusCode}`);
                if (response.description) {
                    lines.push(response.description);
                }
                lines.push('');
            }
        }

        // Code Example
        if (codeExample) {
            lines.push('## Example');
            lines.push('');

            // Determine language based on SDK
            const language = this.getLanguageForSDK();

            // Use the actual method name for the title
            const title = `${methodName} Example`;

            // Wrap in special format for syntax highlighting and copy button
            lines.push(`[inline-code-attrs-start title = '${title}'; type = '${language}'; isFunctional = false; inline-code-attrs-end]`);
            lines.push('[inline-code-start]');
            lines.push(codeExample);
            lines.push('[inline-code-end]');
            lines.push('');
        }

        return lines.join('\n');
    }

    /**
     * Get programming language for code blocks
     * @returns {string}
     */
    getLanguageForSDK() {
        const sdkId = this.sdk.id.toLowerCase();

        if (sdkId.includes('javascript') || sdkId.includes('js')) {
            return 'javascript';
        }
        if (sdkId.includes('java')) {
            return 'java';
        }
        if (sdkId.includes('php')) {
            return 'php';
        }
        if (sdkId.includes('python')) {
            return 'python';
        }

        return '';
    }

    /**
     * Format resource name for display
     * @param {string} resource - Raw resource name
     * @returns {string}
     */
    formatResourceName(resource) {
        // Convert from kebab-case or snake_case to Title Case
        return resource
            .replace(/[-_]/g, ' ')
            .split(' ')
            .map(word => word.charAt(0).toUpperCase() + word.slice(1))
            .join(' ');
    }
}

module.exports = OpenAPIDocGenerator;
