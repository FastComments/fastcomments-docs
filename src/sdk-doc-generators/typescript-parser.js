const fs = require('fs');
const path = require('path');

/**
 * Parser for TypeScript API files to extract method signatures and type information
 */
class TypeScriptParser {
    constructor(repoPath, modelsPath) {
        this.repoPath = repoPath;
        this.modelsPath = path.join(repoPath, modelsPath);
        this.typeCache = new Map(); // Cache loaded type definitions
    }

    /**
     * Extract all API methods from a TypeScript API file
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

        // Match async method signatures
        // Pattern: async methodName(requestParameters: RequestType, ...): Promise<ResponseType>
        const methodRegex = /async\s+(\w+)\s*\(\s*requestParameters:\s*(\w+)(?:[^)]*)\):\s*Promise<([^>]+)>/g;

        let match;
        while ((match = methodRegex.exec(content)) !== null) {
            const [, methodName, requestType, responseType] = match;

            // Skip "Raw" methods - they're just unwrapped versions
            if (methodName.endsWith('Raw')) {
                continue;
            }

            // Extract the method details
            const method = {
                name: methodName,
                requestType: requestType.trim(),
                responseType: responseType.trim(),
                parameters: {},
                nestedTypes: {}
            };

            // Extract the request interface definition
            const requestInterface = this.extractInterface(requestType, content);
            if (requestInterface) {
                method.parameters = requestInterface.properties;

                // Resolve nested types in parameters
                this.resolveNestedTypes(requestInterface.properties, method.nestedTypes, 0, 3);
            }

            // Resolve the response type
            const responseTypeDef = this.loadTypeDefinition(responseType);
            if (responseTypeDef) {
                method.nestedTypes[responseType] = responseTypeDef.summary;

                // Resolve nested types in response (shallower depth to avoid bloat)
                if (responseTypeDef.properties) {
                    this.resolveNestedTypes(responseTypeDef.properties, method.nestedTypes, 0, 2);
                }
            }

            methods.push(method);
        }

        return methods;
    }

    /**
     * Extract interface definition from content
     * @param {string} interfaceName - Interface name to find
     * @param {string} content - File content to search in
     * @returns {Object|null} - Interface metadata
     */
    extractInterface(interfaceName, content) {
        // Match: export interface InterfaceName { ... }
        // Need to handle nested braces properly
        const interfaceStartRegex = new RegExp(
            `export\\s+interface\\s+${interfaceName}\\s*\\{`,
            ''
        );

        const startMatch = content.match(interfaceStartRegex);
        if (!startMatch) {
            return null;
        }

        // Find the matching closing brace
        const startIndex = startMatch.index + startMatch[0].length;
        let braceCount = 1;
        let endIndex = startIndex;

        for (let i = startIndex; i < content.length; i++) {
            if (content[i] === '{') braceCount++;
            if (content[i] === '}') braceCount--;

            if (braceCount === 0) {
                endIndex = i;
                break;
            }
        }

        const propertiesText = content.substring(startIndex, endIndex);
        const properties = {};

        // Match properties: propertyName?: Type;
        // Handle multi-line properties with comments
        const propertyRegex = /(\w+)(\?)?:\s*([^;]+);/g;

        let propMatch;
        while ((propMatch = propertyRegex.exec(propertiesText)) !== null) {
            const [, propName, optional, propType] = propMatch;

            properties[propName] = {
                type: propType.trim(),
                required: !optional
            };
        }

        return {
            name: interfaceName,
            properties
        };
    }

    /**
     * Resolve nested type definitions
     * @param {Object} properties - Properties to resolve
     * @param {Object} nestedTypes - Output object for nested types
     * @param {number} currentDepth - Current recursion depth
     * @param {number} maxDepth - Maximum recursion depth
     */
    resolveNestedTypes(properties, nestedTypes, currentDepth, maxDepth) {
        if (currentDepth >= maxDepth) {
            return;
        }

        for (const [propName, propInfo] of Object.entries(properties)) {
            const type = propInfo.type;

            // Extract type names (handle arrays, unions, etc.)
            const typeNames = this.extractTypeNames(type);

            for (const typeName of typeNames) {
                // Skip primitive types and already resolved types
                if (this.isPrimitiveType(typeName) || nestedTypes[typeName]) {
                    continue;
                }

                // Try to load type definition from models
                const typeDef = this.loadTypeDefinition(typeName);
                if (typeDef) {
                    nestedTypes[typeName] = typeDef.summary;

                    // Recursively resolve nested types
                    if (typeDef.properties) {
                        this.resolveNestedTypes(typeDef.properties, nestedTypes, currentDepth + 1, maxDepth);
                    }
                }
            }
        }
    }

    /**
     * Extract type names from a type expression
     * @param {string} typeExpr - Type expression (e.g., "Array<Type>", "Type | null")
     * @returns {Array<string>} - Array of type names
     */
    extractTypeNames(typeExpr) {
        const names = [];

        // Remove common wrappers
        typeExpr = typeExpr
            .replace(/\s*\|\s*null/g, '')
            .replace(/\s*\|\s*undefined/g, '')
            .trim();

        // Handle Array<Type>
        const arrayMatch = typeExpr.match(/Array<([^>]+)>/);
        if (arrayMatch) {
            names.push(...this.extractTypeNames(arrayMatch[1]));
            return names;
        }

        // Handle unions (Type1 | Type2)
        if (typeExpr.includes('|')) {
            const parts = typeExpr.split('|').map(p => p.trim());
            for (const part of parts) {
                names.push(...this.extractTypeNames(part));
            }
            return names;
        }

        // Handle dictionaries { [key: string]: ValueType }
        const dictMatch = typeExpr.match(/\{\s*\[.*\]:\s*([^}]+)\}/);
        if (dictMatch) {
            names.push(...this.extractTypeNames(dictMatch[1]));
            return names;
        }

        // Simple type name
        if (!this.isPrimitiveType(typeExpr)) {
            names.push(typeExpr);
        }

        return names;
    }

    /**
     * Check if type is a primitive
     * @param {string} typeName - Type name
     * @returns {boolean}
     */
    isPrimitiveType(typeName) {
        const primitives = ['string', 'number', 'boolean', 'Date', 'any', 'void', 'unknown'];
        return primitives.includes(typeName);
    }

    /**
     * Load type definition from models directory
     * @param {string} typeName - Type name to load
     * @returns {Object|null} - Type definition
     */
    loadTypeDefinition(typeName) {
        // Check cache first
        if (this.typeCache.has(typeName)) {
            return this.typeCache.get(typeName);
        }

        const typeFile = path.join(this.modelsPath, `${typeName}.ts`);

        if (!fs.existsSync(typeFile)) {
            return null;
        }

        const content = fs.readFileSync(typeFile, 'utf8');

        // Check if it's an enum
        const enumDef = this.extractEnum(typeName, content);
        if (enumDef) {
            this.typeCache.set(typeName, enumDef);
            return enumDef;
        }

        // Extract interface
        const interfaceDef = this.extractInterface(typeName, content);
        if (interfaceDef) {
            const typeDef = {
                name: typeName,
                kind: 'interface',
                properties: interfaceDef.properties,
                summary: this.summarizeInterface(interfaceDef)
            };
            this.typeCache.set(typeName, typeDef);
            return typeDef;
        }

        return null;
    }

    /**
     * Extract enum definition from content
     * @param {string} enumName - Enum name to find
     * @param {string} content - File content
     * @returns {Object|null} - Enum metadata
     */
    extractEnum(enumName, content) {
        // Match: export enum EnumName { ... }
        const enumRegex = new RegExp(
            `export\\s+enum\\s+${enumName}\\s*\\{([^}]+)\\}`,
            's'
        );

        const match = content.match(enumRegex);
        if (!match) {
            return null;
        }

        const valuesText = match[1];
        const values = [];

        // Match enum values: valueName = 'value'
        const valueRegex = /(\w+)\s*=\s*['"]([^'"]+)['"]/g;

        let valueMatch;
        while ((valueMatch = valueRegex.exec(valuesText)) !== null) {
            values.push(valueMatch[2]);
        }

        return {
            name: enumName,
            kind: 'enum',
            values,
            summary: `enum: ${values.map(v => `'${v}'`).join(' | ')}`
        };
    }

    /**
     * Summarize interface as a string
     * @param {Object} interfaceDef - Interface definition
     * @returns {string} - Summary string
     */
    summarizeInterface(interfaceDef) {
        const props = [];
        for (const [name, info] of Object.entries(interfaceDef.properties)) {
            const optional = info.required ? '' : '?';
            props.push(`${name}${optional}: ${info.type}`);
        }
        return `{ ${props.join(', ')} }`;
    }
}

module.exports = TypeScriptParser;
