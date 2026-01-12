const fs = require('fs');
const path = require('path');
const BaseDocGenerator = require('./base-generator');
const RustParser = require('./rust-parser');
const OpenAIClient = require('./openai-client');

/**
 * Documentation generator that creates API reference from Rust SDK
 * using AI to generate code examples
 */
class RustAIGenerator extends BaseDocGenerator {
    /**
     * Generate API reference documentation using AI
     * @returns {Promise<GeneratedDocs>}
     */
    async generate() {
        const config = this.sdk.rustAiConfig;
        if (!config) {
            throw new Error(`No rustAiConfig found for ${this.sdk.id}`);
        }

        // Parse OpenAPI spec for HTTP method, path, and tag information
        const spec = this.parseOpenAPISpec(config.specPath);
        if (!spec) {
            console.warn(`Could not parse OpenAPI spec for ${this.sdk.id}`);
            return { intro: '', conclusion: '', sections: [] };
        }

        // Create parser and OpenAI client
        const parser = new RustParser(this.repoPath, config.modelsPath);
        // Use cache path outside of sdks-checkout so it can be checked into git
        const aiCachePath = path.join(__dirname, '..', 'sdk-ai-cache', this.sdk.id);
        const aiClient = new OpenAIClient(aiCachePath, 'rust');

        // Build operation map from OpenAPI spec
        const operationMap = this.buildOperationMap(spec);

        const allMethods = [];

        // Process each API file
        for (const apiFile of config.apiFiles) {
            console.log(`Processing API file: ${apiFile}`);

            const methods = parser.extractApiMethods(apiFile);
            console.log(`Found ${methods.length} methods in ${apiFile}`);

            // Enrich methods with OpenAPI info
            for (const method of methods) {
                // Convert snake_case to camelCase for matching with OpenAPI
                const camelCaseName = this.snakeToCamelCase(method.name);

                // Try exact match first, then case-insensitive match
                let opInfo = operationMap.get(camelCaseName);

                if (!opInfo) {
                    // Try with capitalized first letter
                    const capitalizedName = camelCaseName.charAt(0).toUpperCase() + camelCaseName.slice(1);
                    opInfo = operationMap.get(capitalizedName);
                }

                if (opInfo) {
                    method.httpMethod = opInfo.method.toUpperCase();
                    method.path = opInfo.path;
                    method.tag = opInfo.tag || 'api';
                    method.authType = opInfo.tag === 'Public' ? 'none' : 'x-api-key';
                    method.description = opInfo.description || '';
                } else {
                    console.warn(`No OpenAPI operation found for ${method.name}`);
                }

                allMethods.push(method);
            }
        }

        console.log(`Total methods to process: ${allMethods.length}`);

        // Generate code examples using AI and write files as we go
        console.log('Generating code examples using AI...');
        const sections = await this.generateCodeExamplesAndWriteFiles(allMethods, aiClient);

        return {
            intro: '',
            conclusion: '',
            sections
        };
    }

    /**
     * Generate code examples and write markdown files as each one completes
     * @param {Array<Object>} allMethods - All methods to process
     * @param {OpenAIClient} aiClient - OpenAI client
     * @returns {Promise<Array>} - Generated sections
     */
    async generateCodeExamplesAndWriteFiles(allMethods, aiClient) {
        const sections = [];
        let currentIndex = 0;
        const concurrency = 10;

        // Create generated items directory
        const generatedDir = path.join(__dirname, '..', 'content', 'guides', this.sdk.id, 'items', 'generated');
        console.log(`Creating generated items directory: ${generatedDir}`);
        if (!fs.existsSync(generatedDir)) {
            fs.mkdirSync(generatedDir, { recursive: true });
        }

        // Function to get next method
        const next = () => {
            if (currentIndex < allMethods.length) {
                const method = allMethods[currentIndex];
                currentIndex++;
                return method;
            }
            return null;
        };

        // Worker function that processes methods and writes files immediately
        const worker = async () => {
            while (true) {
                const method = next();
                if (!method) break;

                // Generate code example (uses cache if signature unchanged)
                const codeExample = await aiClient.generateCodeExample(method);

                // Determine resource for categorization
                let resource = method.tag || 'api';

                // Try to infer resource from path for default, api, or Public tags
                if (!method.tag || method.tag === 'api' || resource === 'Public') {
                    const inferredResource = this.inferResourceFromPath(method.path);
                    // Only use inferred resource if it's meaningful
                    if (inferredResource && inferredResource !== 'api') {
                        resource = inferredResource;
                    } else if (resource === 'Public') {
                        resource = 'Misc Apis';
                    }
                }

                // Generate section
                const section = this.generateMethodSection(method, codeExample, resource);
                if (section) {
                    // Write file immediately with -generated suffix
                    const filePath = path.join(generatedDir, section.file);
                    console.log(`Writing file: ${filePath}`);
                    fs.writeFileSync(filePath, section.content, 'utf8');

                    // Update section.file to include the generated/ prefix for meta.json
                    section.file = 'generated/' + section.file;
                    sections.push(section);
                }
            }
        };

        // Start concurrent workers
        const workers = [];
        for (let i = 0; i < concurrency; i++) {
            workers.push(worker());
        }

        // Wait for all workers to complete
        await Promise.all(workers);

        return sections;
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
     * Build map of operationId to operation info from OpenAPI spec
     * @param {Object} spec - OpenAPI specification
     * @returns {Map<string, Object>} - Map of operationId to info
     */
    buildOperationMap(spec) {
        const map = new Map();

        for (const [pathStr, pathItem] of Object.entries(spec.paths || {})) {
            for (const [method, operation] of Object.entries(pathItem)) {
                if (!['get', 'post', 'put', 'patch', 'delete'].includes(method)) {
                    continue;
                }

                // Skip Hidden API
                const tag = operation.tags?.[0];
                if (tag === 'Hidden') {
                    continue;
                }

                if (operation.operationId) {
                    map.set(operation.operationId, {
                        method,
                        path: pathStr,
                        tag,
                        description: operation.summary || operation.description
                    });
                }
            }
        }

        return map;
    }

    /**
     * Group methods by resource (tag)
     * @param {Array<Object>} methods - Array of methods
     * @returns {Object<string, Array>} - Methods grouped by resource
     */
    groupMethodsByResource(methods) {
        const grouped = {};

        for (const method of methods) {
            // Map Public to Misc Apis
            let resource = method.tag || 'api';
            if (resource === 'Public') {
                resource = 'Misc Apis';
            }

            if (!grouped[resource]) {
                grouped[resource] = [];
            }

            grouped[resource].push(method);
        }

        return grouped;
    }

    /**
     * Generate GitHub URL for a type definition
     * @param {string} fileName - File name relative to modelsPath
     * @returns {string} - GitHub URL
     */
    generateTypeGitHubUrl(fileName) {
        const config = this.sdk.rustAiConfig;
        const baseUrl = this.sdk.repo.replace(/\.git$/, '');
        const branch = this.sdk.branch;
        const modelsPath = config.modelsPath;

        return `${baseUrl}/blob/${branch}/${modelsPath}${fileName}`;
    }

    /**
     * Generate section for a single method
     * @param {Object} method - Method metadata
     * @param {string|null} codeExample - Generated code example
     * @param {string} resource - Resource name
     * @returns {Object|null} - Doc section
     */
    generateMethodSection(method, codeExample, resource) {
        // Skip methods without a name
        if (!method || !method.name) {
            console.warn('Skipping method with undefined name');
            return null;
        }

        const lines = [];

        // Description
        if (method.description) {
            lines.push(method.description);
            lines.push('');
        }

        // Parameters
        if (method.parameters && Object.keys(method.parameters).length > 0) {
            lines.push('## Parameters');
            lines.push('');
            lines.push('| Name | Type | Required | Description |');
            lines.push('|------|------|----------|-------------|');

            for (const [paramName, paramInfo] of Object.entries(method.parameters)) {
                const required = paramInfo.required ? 'Yes' : 'No';
                const type = paramInfo.type.replace(/\|/g, '\\|');
                lines.push(`| ${paramName} | ${type} | ${required} |  |`);
            }
            lines.push('');
        }

        // Response Type
        if (method.responseType) {
            lines.push('## Response');
            lines.push('');

            // Generate GitHub link if we have type info with filePath
            const typeInfo = method.nestedTypes ? method.nestedTypes[method.responseType] : null;
            if (typeInfo && typeInfo.filePath) {
                const githubUrl = this.generateTypeGitHubUrl(typeInfo.filePath);
                lines.push(`Returns: [\`${method.responseType}\`](${githubUrl})`);
            } else {
                lines.push(`Returns: \`${method.responseType}\``);
            }
            lines.push('');
        }

        // Code Example
        if (codeExample) {
            lines.push('## Example');
            lines.push('');

            const title = `${method.name} Example`;

            // Wrap in special format for syntax highlighting and copy button
            lines.push(`[inline-code-attrs-start title = '${title}'; type = 'rust'; isFunctional = false; inline-code-attrs-end]`);
            lines.push('[inline-code-start]');
            lines.push(codeExample);
            lines.push('[inline-code-end]');
            lines.push('');
        }

        const content = lines.join('\n');

        // Categorize by resource for meta.json (no "API Reference -" prefix)
        const subCat = this.formatResourceName(resource);

        // Generate filename with -api-generated suffix
        const filename = this.sanitizeFilename(method.name) + '-api-generated.md';

        return {
            name: method.name,
            file: filename,
            content,
            subCat,
            type: 'api'
        };
    }

    /**
     * Convert snake_case to camelCase
     * @param {string} str - snake_case string
     * @returns {string} - camelCase string
     */
    snakeToCamelCase(str) {
        // List of common acronyms that should stay uppercase
        const acronyms = ['sso', 'api', 'url', 'uri', 'http', 'https', 'html', 'json', 'xml'];

        const parts = str.split('_');

        return parts.map((part, index) => {
            const lowerPart = part.toLowerCase();

            // Special handling for 'id' - use 'Id' instead of 'ID' to match OpenAPI convention
            if (lowerPart === 'id') {
                return 'Id';
            }

            // Check if this part is an acronym
            if (acronyms.includes(lowerPart)) {
                return part.toUpperCase();
            }

            // Check for compound acronyms (e.g., 'urlid' -> 'URLId')
            for (const acronym of acronyms) {
                if (lowerPart.endsWith(acronym) && lowerPart !== acronym) {
                    const prefix = lowerPart.slice(0, -acronym.length);
                    // Check if the prefix is also an acronym
                    if (acronyms.includes(prefix)) {
                        return prefix.toUpperCase() + acronym.charAt(0).toUpperCase() + acronym.slice(1);
                    }
                }
            }

            // Handle compound with 'id' at the end (e.g., 'urlid' -> 'URLId')
            if (lowerPart.endsWith('id') && lowerPart !== 'id') {
                const prefix = lowerPart.slice(0, -2);
                if (acronyms.includes(prefix)) {
                    return prefix.toUpperCase() + 'Id';
                }
            }

            // First part stays lowercase (for camelCase), rest get capitalized
            if (index === 0) {
                return part;
            }
            return part.charAt(0).toUpperCase() + part.slice(1);
        }).join('');
    }
}

module.exports = RustAIGenerator;
