/**
 * Utility functions for converting between naming conventions
 */

const fs = require('fs');
const path = require('path');

/**
 * Convert PascalCase to camelCase
 * @param {string} name - PascalCase string (e.g., "GetComments")
 * @returns {string} - camelCase string (e.g., "getComments")
 */
function pascalToCamel(name) {
    if (!name || name.length === 0) return name;
    return name.charAt(0).toLowerCase() + name.slice(1);
}

/**
 * Convert PascalCase to snake_case
 * @param {string} name - PascalCase string (e.g., "GetComments")
 * @returns {string} - snake_case string (e.g., "get_comments")
 */
function pascalToSnake(name) {
    if (!name || name.length === 0) return name;

    return name
        .replace(/([A-Z])/g, '_$1')  // Add underscore before capitals
        .toLowerCase()                // Convert to lowercase
        .replace(/^_/, '');          // Remove leading underscore
}

/**
 * Convert PascalCase to the specified naming convention
 * @param {string} name - PascalCase string (e.g., "GetComments")
 * @param {string} convention - Target naming convention ("camelCase", "snake_case", "PascalCase")
 * @returns {string} - Converted string
 */
function convertFromPascal(name, convention) {
    if (!convention) return name;

    switch (convention) {
        case 'camelCase':
            return pascalToCamel(name);
        case 'snake_case':
            return pascalToSnake(name);
        case 'PascalCase':
            return name;
        default:
            console.warn(`Unknown naming convention: ${convention}, returning original name`);
            return name;
    }
}

/**
 * Parse SDK documentation table to extract actual method names
 * @param {string} repoPath - Path to SDK repository
 * @param {Object} config - OpenAPI config with generatedDocsPath and docFilePattern
 * @param {string} apiClass - API class name (e.g., 'DefaultApi', 'PublicApi')
 * @returns {Map<string, string>} - Map from "METHOD /path" to SDK method name
 */
function parseSDKDocTable(repoPath, config, apiClass = 'DefaultApi') {
    const methodMap = new Map();

    if (!config || !config.generatedDocsPath || !config.docFilePattern) {
        return methodMap;
    }

    const docsPath = path.join(repoPath, config.generatedDocsPath);
    const docFile = config.docFilePattern.replace('{ApiClass}', apiClass);
    const docPath = path.join(docsPath, docFile);

    if (!fs.existsSync(docPath)) {
        console.warn(`SDK doc file not found: ${docPath}`);
        return methodMap;
    }

    try {
        const content = fs.readFileSync(docPath, 'utf8');

        // Find the table section
        // Table format: | Method | HTTP request | Description | (Java/PHP)
        //           or: Method | HTTP request | Description (Python)
        // Followed by separator line: |------------- | ------------- | -------------| (with or without pipes)
        const tableRegex = /\|?\s*Method\s*\|\s*HTTP request\s*\|\s*Description\s*\|?\s*\n\|?[-\s|]+\|?\s*\n([\s\S]*?)(?=\n#{1,2}\s|$)/;
        const tableMatch = content.match(tableRegex);

        if (!tableMatch) {
            return methodMap;
        }

        const tableContent = tableMatch[1];

        // Parse each row in the table
        // Row format: | [**method_name**](link) | **METHOD** /path | description
        // Note: Leading pipe is optional (Python SDK doesn't have it)
        // HTTP methods can be all-caps (POST) or PascalCase (Post) depending on the SDK
        const rowRegex = /\|?\s*\[\*\*([^*]+)\*\*\][^\|]*\|\s*\*\*([A-Z][A-Za-z]*)\*\*\s+([^\s|]+)/g;

        let match;
        while ((match = rowRegex.exec(tableContent)) !== null) {
            const methodName = match[1].replace(/\(\)$/, ''); // Remove trailing () if present (PHP)
            const httpMethod = match[2].toUpperCase(); // Normalize to uppercase (Go uses "Post", others use "POST")
            const apiPath = match[3];

            // Create key as "METHOD /path" to match with OpenAPI spec
            const key = `${httpMethod} ${apiPath}`;
            methodMap.set(key, methodName);
        }

        console.log(`Parsed ${methodMap.size} methods from ${apiClass} table`);

    } catch (e) {
        console.error(`Error parsing SDK doc table: ${e.message}`);
    }

    return methodMap;
}

module.exports = {
    pascalToCamel,
    pascalToSnake,
    convertFromPascal,
    parseSDKDocTable
};
