const fs = require('fs');
const path = require('path');

/**
 * Parser for Rust API files to extract method signatures and type information
 */
class RustParser {
    constructor(repoPath, modelsPath) {
        this.repoPath = repoPath;
        this.modelsPath = path.join(repoPath, modelsPath);
        this.typeCache = new Map(); // Cache loaded type definitions
    }

    /**
     * Extract all API methods from a Rust API file
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

        // Match async function signatures
        // Pattern: pub async fn method_name(configuration: &configuration::Configuration, params: ParamsType) -> Result<ResponseType, Error<ErrorType>>
        const methodRegex = /pub\s+async\s+fn\s+(\w+)\s*\([^,]+,\s*params:\s*(\w+)\)\s*->\s*Result<([^,]+),\s*Error<[^>]+>>/g;

        let match;
        while ((match = methodRegex.exec(content)) !== null) {
            const [, methodName, paramsType, responseType] = match;

            // Extract the method details
            const method = {
                name: methodName,
                paramsType: paramsType.trim(),
                responseType: responseType.trim().replace(/^models::/, ''),
                parameters: {},
                nestedTypes: {}
            };

            // Extract the params struct definition from the same file
            const paramsStruct = this.extractParamsStruct(paramsType, content);
            if (paramsStruct) {
                method.parameters = paramsStruct.fields;

                // Resolve nested types in parameters
                this.resolveNestedTypes(paramsStruct.fields, method.nestedTypes, 0, 3);
            }

            // Resolve the response type (use method.responseType which has models:: stripped)
            const responseTypeDef = this.loadTypeDefinition(method.responseType);

            if (responseTypeDef) {
                method.nestedTypes[method.responseType] = {
                    summary: responseTypeDef.summary,
                    filePath: responseTypeDef.filePath
                };

                // Resolve nested types in response (shallower depth to avoid bloat)
                if (responseTypeDef.fields) {
                    this.resolveNestedTypes(responseTypeDef.fields, method.nestedTypes, 0, 2);
                }
            }

            methods.push(method);
        }

        return methods;
    }

    /**
     * Extract parameter struct definition from content
     * @param {string} structName - Struct name to find
     * @param {string} content - File content to search in
     * @returns {Object|null} - Struct metadata
     */
    extractParamsStruct(structName, content) {
        // Match: pub struct StructName { ... }
        // Need to handle nested braces properly
        const structStartRegex = new RegExp(
            `pub\\s+struct\\s+${structName}\\s*\\{`,
            ''
        );

        const startMatch = content.match(structStartRegex);
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

        const fieldsText = content.substring(startIndex, endIndex);
        const fields = {};

        // Match fields: pub field_name: Type
        // Handle Option<Type> for optional fields
        const fieldRegex = /pub\s+(\w+):\s*([^,\n]+)/g;

        let fieldMatch;
        while ((fieldMatch = fieldRegex.exec(fieldsText)) !== null) {
            const [, fieldName, fieldType] = fieldMatch;
            const cleanType = fieldType.trim();

            // Check if it's an Option<T> type
            const isOptional = cleanType.startsWith('Option<');
            let actualType = cleanType;
            if (isOptional) {
                const optionMatch = cleanType.match(/Option<(.+)>/);
                if (optionMatch) {
                    actualType = optionMatch[1];
                }
            }

            fields[fieldName] = {
                type: actualType,
                required: !isOptional
            };
        }

        return {
            name: structName,
            fields
        };
    }

    /**
     * Resolve nested type definitions
     * @param {Object} fields - Fields to resolve
     * @param {Object} nestedTypes - Output object for nested types
     * @param {number} currentDepth - Current recursion depth
     * @param {number} maxDepth - Maximum recursion depth
     */
    resolveNestedTypes(fields, nestedTypes, currentDepth, maxDepth) {
        if (currentDepth >= maxDepth) {
            return;
        }

        for (const [fieldName, fieldInfo] of Object.entries(fields)) {
            const type = fieldInfo.type;

            // Extract type names (handle Vec, Option, etc.)
            const typeNames = this.extractTypeNames(type);

            for (const typeName of typeNames) {
                // Skip primitive types and already resolved types
                if (this.isPrimitiveType(typeName) || nestedTypes[typeName]) {
                    continue;
                }

                // Try to load type definition from models
                const typeDef = this.loadTypeDefinition(typeName);
                if (typeDef) {
                    nestedTypes[typeName] = {
                        summary: typeDef.summary,
                        filePath: typeDef.filePath
                    };

                    // Recursively resolve nested types
                    if (typeDef.fields) {
                        this.resolveNestedTypes(typeDef.fields, nestedTypes, currentDepth + 1, maxDepth);
                    }
                }
            }
        }
    }

    /**
     * Extract type names from a type expression
     * @param {string} typeExpr - Type expression (e.g., "Vec<Type>", "Box<Type>")
     * @returns {Array<string>} - Array of type names
     */
    extractTypeNames(typeExpr) {
        const names = [];

        // Remove common wrappers and clean up
        typeExpr = typeExpr.trim();

        // Handle Vec<Type>
        const vecMatch = typeExpr.match(/Vec<(.+)>/);
        if (vecMatch) {
            names.push(...this.extractTypeNames(vecMatch[1]));
            return names;
        }

        // Handle Box<Type>
        const boxMatch = typeExpr.match(/Box<(.+)>/);
        if (boxMatch) {
            names.push(...this.extractTypeNames(boxMatch[1]));
            return names;
        }

        // Handle std::collections::HashMap<K, V> - extract V
        const hashMapMatch = typeExpr.match(/std::collections::HashMap<[^,]+,\s*(.+)>/);
        if (hashMapMatch) {
            names.push(...this.extractTypeNames(hashMapMatch[1]));
            return names;
        }

        // Remove models:: prefix if present
        typeExpr = typeExpr.replace(/^models::/, '');

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
        const primitives = ['String', 'str', 'i32', 'i64', 'u32', 'u64', 'f32', 'f64', 'bool', 'usize', 'isize'];
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

        // Convert CamelCase to snake_case for file names
        const fileName = this.toSnakeCase(typeName);
        const typeFile = path.join(this.modelsPath, `${fileName}.rs`);

        if (!fs.existsSync(typeFile)) {
            return null;
        }

        const content = fs.readFileSync(typeFile, 'utf8');

        // Check if it's an enum
        const enumDef = this.extractEnum(typeName, content);
        if (enumDef) {
            enumDef.filePath = `${fileName}.rs`;
            this.typeCache.set(typeName, enumDef);
            return enumDef;
        }

        // Extract struct
        const structDef = this.extractStruct(typeName, content);
        if (structDef) {
            const typeDef = {
                name: typeName,
                kind: 'struct',
                fields: structDef.fields,
                summary: this.summarizeStruct(structDef),
                filePath: `${fileName}.rs`
            };
            this.typeCache.set(typeName, typeDef);
            return typeDef;
        }

        return null;
    }

    /**
     * Extract struct definition from content
     * @param {string} structName - Struct name to find
     * @param {string} content - File content
     * @returns {Object|null} - Struct metadata
     */
    extractStruct(structName, content) {
        // Match: pub struct StructName { ... }
        const structStartRegex = new RegExp(
            `pub\\s+struct\\s+${structName}\\s*\\{`,
            ''
        );

        const startMatch = content.match(structStartRegex);
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

        const fieldsText = content.substring(startIndex, endIndex);
        const fields = {};

        // Match fields with serde annotations
        // #[serde(...)] pub field_name: Type,
        const lines = fieldsText.split('\n');
        let currentField = null;
        let isOptional = false;

        for (const line of lines) {
            // Check for skip_serializing_if = "Option::is_none" which indicates optional
            if (line.includes('skip_serializing_if = "Option::is_none"')) {
                isOptional = true;
            }

            // Match field definitions
            const fieldMatch = line.match(/pub\s+(\w+):\s*([^,]+)/);
            if (fieldMatch) {
                const [, fieldName, fieldType] = fieldMatch;
                const cleanType = fieldType.trim();

                // Check if it's an Option<T> type
                const typeIsOption = cleanType.startsWith('Option<');
                let actualType = cleanType;
                if (typeIsOption) {
                    const optionMatch = cleanType.match(/Option<(.+)>/);
                    if (optionMatch) {
                        actualType = optionMatch[1];
                    }
                }

                fields[fieldName] = {
                    type: actualType,
                    required: !(isOptional || typeIsOption)
                };

                isOptional = false; // Reset for next field
            }
        }

        return {
            name: structName,
            fields
        };
    }

    /**
     * Extract enum definition from content
     * @param {string} enumName - Enum name to find
     * @param {string} content - File content
     * @returns {Object|null} - Enum metadata
     */
    extractEnum(enumName, content) {
        // Match: pub enum EnumName { ... }
        const enumRegex = new RegExp(
            `pub\\s+enum\\s+${enumName}\\s*\\{([^}]+)\\}`,
            's'
        );

        const match = content.match(enumRegex);
        if (!match) {
            return null;
        }

        const variantsText = match[1];
        const variants = [];

        // Match enum variants - handle both simple and complex variants
        // Simple: VariantName,
        // With data: VariantName(Type),
        const lines = variantsText.split('\n');
        for (const line of lines) {
            const simpleMatch = line.match(/#\[serde\(rename\s*=\s*"([^"]+)"\)\]/);
            if (simpleMatch) {
                variants.push(simpleMatch[1]);
            }
        }

        // If no serde renames found, try simple variant names
        if (variants.length === 0) {
            const variantRegex = /(\w+)(?:\([^)]*\))?\s*,/g;
            let variantMatch;
            while ((variantMatch = variantRegex.exec(variantsText)) !== null) {
                variants.push(this.toSnakeCase(variantMatch[1]));
            }
        }

        return {
            name: enumName,
            kind: 'enum',
            variants,
            summary: `enum: ${variants.map(v => `'${v}'`).join(' | ')}`
        };
    }

    /**
     * Summarize struct as a string
     * @param {Object} structDef - Struct definition
     * @returns {string} - Summary string
     */
    summarizeStruct(structDef) {
        const fields = [];
        for (const [name, info] of Object.entries(structDef.fields)) {
            const optional = info.required ? '' : '?';
            fields.push(`${name}${optional}: ${info.type}`);
        }
        return `{ ${fields.join(', ')} }`;
    }

    /**
     * Convert CamelCase to snake_case
     * @param {string} str - CamelCase string
     * @returns {string} - snake_case string
     */
    toSnakeCase(str) {
        return str
            .replace(/([A-Z])/g, '_$1')
            .replace(/([0-9]+)/g, '_$1')
            .toLowerCase()
            .replace(/^_/, '');
    }
}

module.exports = RustParser;
