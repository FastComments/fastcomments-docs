const fs = require('fs');
const path = require('path');
const BaseDocGenerator = require('./base-generator');
const { convertFromPascal, parseSDKDocTable } = require('./naming-utils');

/**
 * Documentation generator that creates API reference from OpenAPI spec
 * and extracts code examples from generated markdown docs
 */
class OpenAPIDocGenerator extends BaseDocGenerator {
    constructor(sdk, repoPath) {
        super(sdk, repoPath);
    }

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
        // Will throw immediately if any operation fails
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
        // Use configured apiClasses if available, otherwise default to standard naming
        const apiClasses = config.apiClasses || ['DefaultApi', 'PublicApi'];

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

                // Try to infer resource from path for default or Public tags
                let resource;
                if (tag === 'default' || tag === 'Public') {
                    resource = this.inferResourceFromPath(pathStr);
                    // Only fall back to 'Misc Apis' if we can't infer anything useful
                    if (!resource || resource === 'api') {
                        resource = 'Misc Apis';
                    }
                } else {
                    resource = tag;
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

        // FAIL FAST if method name not found
        if (!name) {
            throw new Error(`Method not found in SDK docs for ${lookupKey}. Operation: ${operation.operationId || operation.summary}`);
        }

        // Extract code example and return type from generated docs
        const { codeExample, returnType } = this.extractCodeExample(operation, config, name);

        // FAIL FAST if return type not found in markdown
        if (!returnType) {
            throw new Error(`Failed to extract return type from markdown for method: ${name} (${lookupKey})`);
        }

        // Generate markdown content
        const content = this.generateOperationMarkdown(operation, codeExample, name, returnType);

        // Categorize by resource for meta.json (no "API Reference -" prefix)
        const subCat = this.formatResourceName(resource);

        // Generate filename with -api-generated suffix
        const filename = this.sanitizeFilename(name) + '-api-generated.md';

        return {
            name,
            file: filename,
            content,
            subCat,
            type: 'api'
        };
    }

    /**
     * Extract code example and return type from generated markdown docs
     * @param {Object} operation - Operation details
     * @param {Object} config - OpenAPI config
     * @param {string} methodName - Actual SDK method name to search for
     * @returns {{codeExample: string|null, returnType: string|null}}
     */
    extractCodeExample(operation, config, methodName) {
        const docsPath = path.join(this.repoPath, config.generatedDocsPath);

        if (!fs.existsSync(docsPath)) {
            console.warn(`Generated docs path not found: ${docsPath}`);
            return { codeExample: null, returnType: null };
        }

        // Find the API class file (e.g., DefaultApi.md, PublicApi.md)
        // Determine which file based on operation tags
        const apiClass = this.determineApiClass(operation);
        const docFile = config.docFilePattern.replace('{ApiClass}', apiClass);
        const docPath = path.join(docsPath, docFile);

        if (!fs.existsSync(docPath)) {
            console.warn(`Doc file not found: ${docPath}`);
            return { codeExample: null, returnType: null };
        }

        try {
            const docContent = fs.readFileSync(docPath, 'utf8');
            return this.parseCodeExampleAndReturnType(docContent, methodName);
        } catch (e) {
            console.error(`Error reading doc file: ${e.message}`);
            return { codeExample: null, returnType: null };
        }
    }

    /**
     * Determine which API class the operation belongs to
     * @param {Object} operation - Operation details
     * @returns {string}
     */
    determineApiClass(operation) {
        const tags = operation.tags || [];
        const config = this.sdk.openApiConfig;
        const apiClasses = config?.apiClasses || ['DefaultApi', 'PublicApi'];

        // Map tags to API classes
        // Use the configured class names to determine the pattern
        const defaultClass = apiClasses[0] || 'DefaultApi';
        const publicClass = apiClasses[1] || 'PublicApi';

        if (tags.includes('Public')) return publicClass;
        if (tags.includes('Hidden')) return 'HiddenApi';

        return defaultClass; // Default for secured/authenticated endpoints
    }

    /**
     * Parse code example and return type from generated markdown
     * @param {string} markdown - Markdown content
     * @param {string} methodName - Actual SDK method name to search for
     * @returns {{codeExample: string|null, returnType: string|null}}
     */
    parseCodeExampleAndReturnType(markdown, methodName) {
        if (!methodName) return { codeExample: null, returnType: null };

        const escapedMethodName = methodName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

        // Try multiple heading formats:
        // 1. Most SDKs: # **methodName**
        // 2. With backticks and parens: ## `methodName()`
        // 3. Go SDK: ## methodName (plain)
        const patterns = [
            new RegExp(`^#+\\s*\\*\\*${escapedMethodName}\\*\\*\\s*$`, 'm'),
            new RegExp(`^#+\\s*\`${escapedMethodName}\\(\\)\`\\s*$`, 'm'),
            new RegExp(`^#+\\s*${escapedMethodName}\\s*$`, 'm')
        ];

        let match = null;
        for (const pattern of patterns) {
            match = markdown.match(pattern);
            if (match) break;
        }

        if (!match) {
            return { codeExample: null, returnType: null };
        }

        // Extract content after the heading until next H1/H2 heading or end
        // Use \n#{1,2}\s\*\* to match only markdown headings (with bold), not code comments
        const startIndex = match.index + match[0].length;
        const nextHeadingMatch = markdown.substring(startIndex).match(/\n#{1,2}\s+\*\*/);
        const endIndex = nextHeadingMatch
            ? startIndex + nextHeadingMatch.index
            : markdown.length;

        const sectionContent = markdown.substring(startIndex, endIndex);

        // Extract code example
        let codeExample = null;
        const exampleSectionMatch = sectionContent.match(/###\s*Example[\s\S]*?```[\w]*\n([\s\S]*?)```/);
        if (exampleSectionMatch) {
            codeExample = exampleSectionMatch[1].trim();
        } else {
            const codeBlockMatch = sectionContent.match(/```[\w]*\n([\s\S]*?)```/);
            if (codeBlockMatch) {
                codeExample = codeBlockMatch[1].trim();
            }
        }

        // Extract return type from ### Return type section
        let returnType = null;
        // Match pattern: [**Type**](link) where Type can include brackets like List[InnerType]
        const returnTypeMatch = sectionContent.match(/###\s*Return type\s*\n+\[\*\*(.+?)\*\*\]\(/);
        if (returnTypeMatch) {
            // Clean up HTML entities and extract the main type name
            let fullType = returnTypeMatch[1].trim();

            // Decode HTML entities (Ruby's OpenAPI generator encodes < and > as &lt; and &gt;)
            fullType = this.decodeHtmlEntities(fullType);

            // Handle generic types first before namespace stripping:
            // Java: List<Type> or Optional<Type>
            // Ruby: Array<Type> or Hash<K, V>
            // Python: List[Type] or Optional[Type]
            // PHP: \Namespace\Type[] (array syntax with namespace)
            // Use .+? for non-greedy matching to get content between < and last >
            const genericMatch = fullType.match(/^(?:List|Optional|Set|Array|Hash)<(.+)>$/);
            const pythonGenericMatch = fullType.match(/^(?:List|Optional|Set)\[([^\]]+)\]$/);
            const phpArrayMatch = fullType.match(/^(.+?)\[\]$/);

            if (genericMatch) {
                // For simple generics like Array<Type>, extract the inner type
                // For complex generics like Hash<K, V> or nested types, keep the full inner content
                const innerContent = genericMatch[1];

                // If it's a simple type (no comma or nested brackets), extract it
                // Otherwise, keep it as-is for complex types
                if (!innerContent.includes(',') && !innerContent.includes('<')) {
                    returnType = innerContent;
                } else {
                    // For complex types, just use the whole type as-is
                    returnType = fullType;
                }
            } else if (pythonGenericMatch) {
                returnType = pythonGenericMatch[1];
            } else if (phpArrayMatch) {
                // PHP array syntax - strip the [] and extract just class name
                fullType = phpArrayMatch[1];
            }

            // Handle PHP namespaces: \FastComments\Client\Model\TypeName -> TypeName
            // Must come after array handling
            if (!returnType) {
                returnType = fullType.replace(/^\\?(?:[A-Za-z]+\\)+([A-Za-z0-9_]+)$/, '$1');
            }
        }

        return { codeExample, returnType };
    }

    /**
     * Decode HTML entities in a string
     * @param {string} str - String with HTML entities
     * @returns {string} - Decoded string
     */
    decodeHtmlEntities(str) {
        const entities = {
            '&lt;': '<',
            '&gt;': '>',
            '&amp;': '&',
            '&quot;': '"',
            '&#39;': "'",
            '&nbsp;': ' '
        };

        return str.replace(/&[#\w]+;/g, (match) => entities[match] || match);
    }

    /**
     * Generate markdown content for an operation
     * @param {Object} operation - Operation details
     * @param {string|null} codeExample - Extracted code example
     * @param {string} methodName - Actual SDK method name
     * @returns {string}
     */
    /**
     * Extract response type name from OpenAPI operation
     * @param {Object} operation - Operation details
     * @returns {string|null} - Response type name
     */
    extractResponseType(operation) {
        if (!operation.responses || !operation.responses['200']) {
            return null;
        }

        const response200 = operation.responses['200'];
        const content = response200.content;
        if (!content || !content['application/json']) {
            return null;
        }

        const schema = content['application/json'].schema;
        if (!schema) {
            return null;
        }

        // Handle direct $ref
        if (schema.$ref) {
            return this.extractTypeNameFromRef(schema.$ref);
        }

        // Handle anyOf/oneOf unions - take the first non-error type
        if (schema.anyOf || schema.oneOf) {
            const options = schema.anyOf || schema.oneOf;
            for (const option of options) {
                if (option.$ref) {
                    const typeName = this.extractTypeNameFromRef(option.$ref);
                    // Skip error types
                    if (typeName && !typeName.includes('Error')) {
                        return typeName;
                    }
                }
            }
            // If all are errors, just return the first one
            if (options[0].$ref) {
                return this.extractTypeNameFromRef(options[0].$ref);
            }
        }

        return null;
    }

    /**
     * Extract type name from $ref string
     * @param {string} ref - Reference string like "#/components/schemas/TypeName"
     * @returns {string} - Type name with underscores removed (to match SDK generator behavior)
     */
    extractTypeNameFromRef(ref) {
        const parts = ref.split('/');
        const typeName = parts[parts.length - 1];
        // Remove underscores to match how SDK generators (Java, PHP, Python) sanitize class names
        return typeName.replace(/_/g, '');
    }

    /**
     * Get the file path for a type in this SDK
     * Language-specific implementations should override this for complex cases
     * @param {string} typeName - Type name from markdown
     * @returns {string|null} - File path relative to repo root
     */
    getTypeFilePath(typeName) {
        const language = this.sdk.language;

        switch (language) {
            case 'java':
                return `client/src/main/java/com/fastcomments/model/${typeName}.java`;
            case 'php':
                return `lib/Model/${typeName}.php`;
            case 'javascript':
                return `src/generated/src/models/${typeName}.ts`;
            case 'python':
                return this.getPythonTypeFilePath(typeName);
            case 'rust':
                return this.getRustTypeFilePath(typeName);
            case 'go':
                return this.getGoTypeFilePath(typeName);
            case 'cpp':
                return `client/include/FastCommentsClient/model/${typeName}.h`;
            case 'swift':
                return `client/FastCommentsSwift/Models/${typeName}.swift`;
            case 'ruby':
                return this.getRubyTypeFilePath(typeName);
            case 'nim':
                return this.getNimTypeFilePath(typeName);
            default:
                return null;
        }
    }

    /**
     * Get Python type file path by extracting from import in model .md file
     * @param {string} typeName - Type name (e.g., "GetUserPresenceStatusesResponse")
     * @returns {string|null} - File path (e.g., "client/models/get_user_presence_statuses_response.py")
     */
    getPythonTypeFilePath(typeName) {
        const config = this.sdk.openApiConfig;
        if (!config) {
            throw new Error(`No openApiConfig for Python SDK`);
        }

        const modelDocPath = path.join(this.repoPath, config.generatedDocsPath, `${typeName}.md`);

        if (!fs.existsSync(modelDocPath)) {
            throw new Error(`Python model doc not found: ${modelDocPath}`);
        }

        const content = fs.readFileSync(modelDocPath, 'utf8');

        // Extract import line: "from client.models.get_user_presence_statuses_response import GetUserPresenceStatusesResponse"
        const importMatch = content.match(/from\s+([\w.]+)\s+import\s+\w+/);

        if (importMatch) {
            const modulePath = importMatch[1].replace(/\./g, '/');
            return `${modulePath}.py`;
        }

        throw new Error(`Could not extract import from ${modelDocPath}`);
    }

    /**
     * Get Rust type file path by searching for struct/enum definition
     * @param {string} typeName - Type name (e.g., "ImportedApiStatusSuccess")
     * @returns {string|null} - File path (e.g., "client/src/models/imported_api_status_success.rs")
     */
    getRustTypeFilePath(typeName) {
        const modelsDir = path.join(this.repoPath, 'client/src/models');

        if (!fs.existsSync(modelsDir)) {
            throw new Error(`Rust models directory not found: ${modelsDir}`);
        }

        // Search for struct or enum definition
        const files = fs.readdirSync(modelsDir);

        for (const file of files) {
            if (!file.endsWith('.rs')) continue;

            const filePath = path.join(modelsDir, file);
            const content = fs.readFileSync(filePath, 'utf8');

            // Look for "pub struct TypeName" or "pub enum TypeName"
            const structRegex = new RegExp(`pub\\s+(?:struct|enum)\\s+${typeName}\\b`);

            if (structRegex.test(content)) {
                return `client/src/models/${file}`;
            }
        }

        throw new Error(`Could not find Rust file for type: ${typeName}`);
    }

    /**
     * Get Go type file path by searching for struct/interface definition
     * @param {string} typeName - Type name (e.g., "AddDomainConfig200ResponseAnyOf")
     * @returns {string|null} - File path (e.g., "client/model_add_domain_config_200_response_any_of.go")
     */
    getGoTypeFilePath(typeName) {
        const modelsDir = path.join(this.repoPath, 'client');

        if (!fs.existsSync(modelsDir)) {
            throw new Error(`Go models directory not found: ${modelsDir}`);
        }

        // Strip slice prefix [] if present (e.g., []SaveComment200Response -> SaveComment200Response)
        if (typeName.startsWith('[]')) {
            typeName = typeName.slice(2);
        }

        // Search for struct or interface definition
        const files = fs.readdirSync(modelsDir);

        for (const file of files) {
            if (!file.startsWith('model_') || !file.endsWith('.go')) continue;

            const filePath = path.join(modelsDir, file);
            const content = fs.readFileSync(filePath, 'utf8');

            // Look for "type TypeName struct" or "type TypeName interface"
            const typeRegex = new RegExp(`type\\s+${typeName}\\s+(?:struct|interface)`);

            if (typeRegex.test(content)) {
                return `client/${file}`;
            }
        }

        throw new Error(`Could not find Go file for type: ${typeName}`);
    }

    /**
     * Get Ruby type file path by converting PascalCase to snake_case
     * @param {string} typeName - Type name (e.g., "GetCommentsPublic200Response")
     * @returns {string|null} - File path (e.g., "client/lib/fastcomments-client/models/get_comments_public200_response.rb")
     */
    getRubyTypeFilePath(typeName) {
        // Convert PascalCase to snake_case
        // GetCommentsPublic200Response -> get_comments_public200_response
        const snakeCase = typeName
            .replace(/([A-Z])/g, '_$1')
            .toLowerCase()
            .replace(/^_/, '');

        return `client/lib/fastcomments-client/models/${snakeCase}.rb`;
    }

    /**
     * Get Nim type file path by converting PascalCase to snake_case with model_ prefix
     * @param {string} typeName - Type name (e.g., "GetCommentsPublic_200_response")
     * @returns {string|null} - File path (e.g., "client/fastcomments/models/model_get_comments_public200response.nim")
     */
    getNimTypeFilePath(typeName) {
        // Convert PascalCase to snake_case with model_ prefix and remove underscores from the type name
        // GetCommentsPublic_200_response -> model_get_comments_public200response
        const snakeCase = 'model_' + typeName
            .replace(/_/g, '')
            .replace(/([A-Z])/g, '_$1')
            .toLowerCase()
            .replace(/^_/, '');

        return `client/fastcomments/models/${snakeCase}.nim`;
    }

    /**
     * Generate GitHub URL for a type definition
     * @param {string} typeName - Actual type name extracted from markdown
     * @returns {string} - GitHub URL
     */
    generateTypeGitHubUrl(typeName) {
        const baseUrl = this.sdk.repo.replace(/\.git$/, '');
        const branch = this.sdk.branch;

        const filePath = this.getTypeFilePath(typeName);

        if (!filePath) {
            return null;
        }

        return `${baseUrl}/blob/${branch}/${filePath}`;
    }

    generateOperationMarkdown(operation, codeExample, methodName, returnType) {
        const lines = [];

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

        // Response Type (extracted from markdown, required)
        lines.push('## Response');
        lines.push('');

        const githubUrl = this.generateTypeGitHubUrl(returnType);
        if (githubUrl) {
            lines.push(`Returns: [\`${returnType}\`](${githubUrl})`);
        } else {
            lines.push(`Returns: \`${returnType}\``);
        }
        lines.push('');

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
        if (sdkId.includes('go')) {
            return 'go';
        }
        if (sdkId.includes('rust')) {
            return 'rust';
        }
        if (sdkId.includes('cpp') || sdkId.includes('c++')) {
            return 'cpp';
        }
        if (sdkId.includes('swift')) {
            return 'swift';
        }
        if (sdkId.includes('ruby')) {
            return 'ruby';
        }
        if (sdkId.includes('nim')) {
            return 'nim';
        }

        return '';
    }

}

module.exports = OpenAPIDocGenerator;
