/**
 * Utility functions for converting between naming conventions
 */

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

module.exports = {
    pascalToCamel,
    pascalToSnake,
    convertFromPascal
};
