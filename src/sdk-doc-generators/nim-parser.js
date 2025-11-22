const fs = require('fs');
const path = require('path');

/**
 * Parser for Nim API files to extract method signatures and type information
 */
class NimParser {
    constructor(repoPath, modelsPath) {
        this.repoPath = repoPath;
        this.modelsPath = path.join(repoPath, modelsPath);
        this.typeCache = new Map(); // Cache loaded type definitions
    }

    /**
     * Extract all API methods from a Nim API file
     * @param {string} apiFilePath - Relative path to API file
     * @returns {Array<Object>} - Array of method metadata
     */
    extractApiMethods(apiFilePath) {
        const fullPath = path.join(this.repoPath, apiFilePath);

        if (!fs.existsSync(fullPath)) {
            console.warn(`API file not found: ${fullPath}`);
            return [];
        }

        const content = fs.readFileSync(fullPath, 'utf8');
        const methods = [];

        // Extract method declarations
        // Pattern: proc methodName*(httpClient: HttpClient, param1: Type1, ...): (Option[ResponseType], Response) =
        const methodRegex = /proc\s+(\w+)\*\(([\s\S]*?)\):\s*\(Option\[(\w+)\],\s*Response\)\s*=/g;

        let match;
        while ((match = methodRegex.exec(content)) !== null) {
            const [, methodName, paramsStr, responseType] = match;

            // Parse parameters
            const parameters = this.parseParameters(paramsStr);

            const method = {
                name: methodName,
                responseType: responseType.trim(),
                parameters: parameters,
                description: '', // Nim generated files don't have doc comments
                nestedTypes: {}
            };

            // Load response type definition
            const responseTypeDef = this.loadTypeDefinition(method.responseType);
            if (responseTypeDef) {
                method.nestedTypes[method.responseType] = {
                    summary: responseTypeDef.summary,
                    filePath: responseTypeDef.filePath
                };
            }

            methods.push(method);
        }

        return methods;
    }

    /**
     * Parse Nim method parameters
     * @param {string} paramsStr - Parameter string from method signature
     * @returns {Object} - Map of parameter names to metadata
     */
    parseParameters(paramsStr) {
        const params = {};

        if (!paramsStr || paramsStr.trim() === '') {
            return params;
        }

        // Split by commas, being careful of nested brackets
        const parts = [];
        let current = '';
        let depth = 0;

        for (let i = 0; i < paramsStr.length; i++) {
            const char = paramsStr[i];
            if (char === '[') depth++;
            if (char === ']') depth--;

            if (char === ',' && depth === 0) {
                parts.push(current.trim());
                current = '';
            } else {
                current += char;
            }
        }
        if (current.trim()) {
            parts.push(current.trim());
        }

        // Parse each parameter
        for (const part of parts) {
            // Match: paramName: Type or paramName: seq[Type]
            const paramMatch = part.match(/(\w+):\s*(.+?)$/);

            if (paramMatch) {
                const paramName = paramMatch[1];
                const paramType = paramMatch[2].trim();

                // Determine if it's optional based on type
                const isOptional = paramType !== 'HttpClient' &&
                                 paramName !== 'tenantId' &&
                                 paramName !== 'urlId' &&
                                 paramName !== 'commentId';

                params[paramName] = {
                    type: paramType,
                    required: !isOptional,
                    isArray: paramType.startsWith('seq[')
                };
            }
        }

        return params;
    }

    /**
     * Load type definition from models directory
     * @param {string} typeName - Name of the type
     * @returns {Object|null} - Type definition with summary and filePath
     */
    loadTypeDefinition(typeName) {
        // Check cache
        if (this.typeCache.has(typeName)) {
            return this.typeCache.get(typeName);
        }

        // Convert PascalCase to snake_case with model_ prefix
        // GetCommentsPublic_200_response -> model_get_comments_public200response.nim
        // Handles acronyms properly: AddSSOUserAPIResponse -> add_sso_user_api_response
        const snakeCase = 'model_' + typeName
            .replace(/_/g, '')
            .replace(/([a-z0-9])([A-Z])/g, '$1_$2')  // lowercase/digit followed by uppercase
            .replace(/([A-Z])([A-Z][a-z])/g, '$1_$2')  // uppercase followed by uppercase+lowercase (handles acronyms)
            .toLowerCase()
            .replace(/url_id/g, 'urlid');  // Special case: URLId becomes urlid not url_id

        const typeFilePath = path.join(this.modelsPath, `${snakeCase}.nim`);

        if (!fs.existsSync(typeFilePath)) {
            console.warn(`Type definition not found: ${typeFilePath}`);
            return null;
        }

        try {
            const content = fs.readFileSync(typeFilePath, 'utf8');

            // Extract type definition from Nim code
            const typeDef = {
                summary: `Type definition for ${typeName}`,
                filePath: typeFilePath.replace(this.repoPath + '/', '')
            };

            this.typeCache.set(typeName, typeDef);
            return typeDef;
        } catch (e) {
            console.error(`Error loading type definition for ${typeName}: ${e.message}`);
            return null;
        }
    }
}

module.exports = NimParser;
