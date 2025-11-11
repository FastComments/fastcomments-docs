const fs = require('fs');
const path = require('path');

/**
 * Parser for C++ API files to extract method signatures and type information
 */
class CppParser {
    constructor(repoPath, modelsPath) {
        this.repoPath = repoPath;
        this.modelsPath = path.join(repoPath, modelsPath);
        this.typeCache = new Map(); // Cache loaded type definitions
    }

    /**
     * Extract all API methods from a C++ API header file
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
        // Pattern: pplx::task<std::shared_ptr<ResponseType>> methodName(...params...) const;
        const methodRegex = /\/\/\/ <summary>([\s\S]*?)\/\/\/ <\/summary>[\s\S]*?pplx::task<std::shared_ptr<(\w+)>>\s+(\w+)\s*\(([\s\S]*?)\)\s*const;/g;

        let match;
        while ((match = methodRegex.exec(content)) !== null) {
            const [, summary, responseType, methodName, paramsStr] = match;

            // Parse parameters
            const parameters = this.parseParameters(paramsStr);

            const method = {
                name: methodName,
                responseType: responseType.trim(),
                parameters: parameters,
                description: summary.trim().replace(/\/\/\//g, '').trim(),
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
     * Parse C++ method parameters
     * @param {string} paramsStr - Parameter string from method signature
     * @returns {Object} - Map of parameter names to metadata
     */
    parseParameters(paramsStr) {
        const params = {};

        if (!paramsStr || paramsStr.trim() === '') {
            return params;
        }

        // Split by commas, but be careful of template parameters
        const parts = [];
        let current = '';
        let depth = 0;

        for (let i = 0; i < paramsStr.length; i++) {
            const char = paramsStr[i];
            if (char === '<') depth++;
            if (char === '>') depth--;

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
            // Match: Type paramName or boost::optional<Type> paramName
            const paramMatch = part.match(/(?:boost::optional<(.+?)>|std::shared_ptr<(.+?)>|(.+?))\s+(\w+)(?:\s*=\s*.+)?$/);

            if (paramMatch) {
                const [, optionalType, sharedPtrType, directType, paramName] = paramMatch;
                const isOptional = !!optionalType;
                let actualType = optionalType || sharedPtrType || directType;

                // Clean up the type
                actualType = actualType
                    .replace(/utility::string_t/g, 'string')
                    .replace(/utility::datetime/g, 'datetime')
                    .replace(/std::vector</g, 'vector<')
                    .replace(/std::shared_ptr</g, '')
                    .replace(/>/g, '')
                    .trim();

                params[paramName] = {
                    type: actualType,
                    required: !isOptional
                };
            }
        }

        return params;
    }

    /**
     * Load type definition from model file
     * @param {string} typeName - Name of the type
     * @returns {Object|null} - Type definition metadata
     */
    loadTypeDefinition(typeName) {
        // Check cache first
        if (this.typeCache.has(typeName)) {
            return this.typeCache.get(typeName);
        }

        // Construct path to model header file
        const fileName = `${typeName}.h`;
        const filePath = path.join(this.modelsPath, fileName);

        if (!fs.existsSync(filePath)) {
            return null;
        }

        const relativeFilePath = path.relative(this.repoPath, filePath);

        const result = {
            name: typeName,
            filePath: relativeFilePath,
            summary: `Type definition for ${typeName}`
        };

        this.typeCache.set(typeName, result);
        return result;
    }

    /**
     * Convert camelCase to snake_case
     * @param {string} str - camelCase string
     * @returns {string} - snake_case string
     */
    camelToSnakeCase(str) {
        return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
    }

    /**
     * Convert PascalCase to Title Case
     * @param {string} str - PascalCase string
     * @returns {string} - Title Case string
     */
    pascalToTitleCase(str) {
        return str.replace(/([A-Z])/g, ' $1').trim();
    }
}

module.exports = CppParser;
