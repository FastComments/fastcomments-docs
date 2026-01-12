const fs = require('fs');
const path = require('path');
const BaseDocGenerator = require('./base-generator');
const NimParser = require('./nim-parser');
const OpenAIClient = require('./openai-client');

/**
 * Documentation generator that creates API reference from Nim SDK
 * using AI to generate code examples
 */
class NimAIGenerator extends BaseDocGenerator {
    /**
     * Generate API reference documentation using AI
     * @returns {Promise<GeneratedDocs>}
     */
    async generate() {
        const config = this.sdk.nimAiConfig;
        if (!config) {
            throw new Error(`No nimAiConfig found for ${this.sdk.id}`);
        }

        // Parse OpenAPI spec for HTTP method, path, and tag information
        const spec = this.parseOpenAPISpec(config.specPath);
        if (!spec) {
            console.warn(`Could not parse OpenAPI spec for ${this.sdk.id}`);
            return { intro: '', conclusion: '', sections: [] };
        }

        // Create parser and OpenAI client
        const parser = new NimParser(this.repoPath, config.modelsPath);
        // Use cache path outside of sdks-checkout so it can be checked into git
        const aiCachePath = path.join(__dirname, '..', 'sdk-ai-cache', this.sdk.id);
        const aiClient = new OpenAIClient(aiCachePath, 'nim');

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
                // Try exact match first
                let opInfo = operationMap.get(method.name);

                // Try with lowercase first letter
                if (!opInfo) {
                    const lowerFirst = method.name.charAt(0).toLowerCase() + method.name.slice(1);
                    opInfo = operationMap.get(lowerFirst);
                }

                // Try with uppercase first letter (OpenAPI uses PascalCase, SDK may use camelCase)
                if (!opInfo) {
                    const upperFirst = method.name.charAt(0).toUpperCase() + method.name.slice(1);
                    opInfo = operationMap.get(upperFirst);
                }

                // Try converting from snake_case to camelCase
                if (!opInfo) {
                    const camelCase = method.name.replace(/_([a-z])/g, (m, p1) => p1.toUpperCase());
                    opInfo = operationMap.get(camelCase);
                }

                if (opInfo) {
                    method.httpMethod = opInfo.method.toUpperCase();
                    method.path = opInfo.path;
                    method.tag = opInfo.tag || 'api';
                    method.authType = opInfo.tag === 'Public' ? 'none' : 'x-api-key';
                    method.description = opInfo.description || method.description;
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

        console.log(`Generated ${sections.length} sections`);
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
     * Build operation map from OpenAPI spec
     * @param {Object} spec - OpenAPI specification
     * @returns {Map} - Map from operationId to operation info
     */
    buildOperationMap(spec) {
        const map = new Map();

        for (const [pathStr, pathItem] of Object.entries(spec.paths || {})) {
            for (const [method, operation] of Object.entries(pathItem)) {
                if (!['get', 'post', 'put', 'patch', 'delete', 'options', 'head'].includes(method)) {
                    continue;
                }

                const tag = operation.tags?.[0] || 'api';

                // Skip Hidden API operations
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
     * Generate GitHub URL for a type definition
     * @param {string} fileName - File path relative to repo root
     * @returns {string} - GitHub URL
     */
    generateTypeGitHubUrl(fileName) {
        const baseUrl = this.sdk.repo.replace(/\.git$/, '');
        const branch = this.sdk.branch;

        return `${baseUrl}/blob/${branch}/${fileName}`;
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
                // Skip HttpClient parameter (it's not user-facing)
                if (paramName === 'httpClient') continue;

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
                lines.push(`Returns: [\`Option[${method.responseType}]\`](${githubUrl})`);
            } else {
                lines.push(`Returns: \`Option[${method.responseType}]\``);
            }
            lines.push('');
        }

        // Code Example
        if (codeExample) {
            lines.push('## Example');
            lines.push('');

            const title = `${method.name} Example`;

            // Wrap in special format for syntax highlighting and copy button
            lines.push(`[inline-code-attrs-start title = '${title}'; type = 'nim'; isFunctional = false; inline-code-attrs-end]`);
            lines.push('[inline-code-start]');
            lines.push(codeExample);
            lines.push('[inline-code-end]');
            lines.push('');
        }

        const content = lines.join('\n');

        // Categorize by resource for meta.json (no "API Reference -" prefix)
        const subCat = this.formatResourceName(resource);

        // Generate filename - convert to kebab-case and add -api-generated suffix
        const filename = this.sanitizeFilename(method.name) + '-api-generated.md';

        return {
            name: method.name,
            file: filename,
            content,
            subCat,
            type: 'api'
        };
    }
}

module.exports = NimAIGenerator;
