const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const OPENAI_API_KEY = process.env.OPENAI_API_KEY;
const OPENAI_MODEL = process.env.OPENAI_MODEL || 'gpt-5-mini';

/**
 * Client for generating code examples using OpenAI API
 */
class OpenAIClient {
    constructor(cachePath) {
        this.cachePath = cachePath;
        this.apiKey = OPENAI_API_KEY;
        this.model = OPENAI_MODEL;

        // Create cache directory if it doesn't exist
        if (!fs.existsSync(cachePath)) {
            fs.mkdirSync(cachePath, { recursive: true });
        }
    }

    /**
     * Generate cache key from method signature
     * @param {Object} method - Method metadata
     * @returns {string} - SHA256 hash
     */
    generateCacheKey(method) {
        const data = {
            methodName: method.name,
            parameters: method.parameters,
            responseType: method.responseType,
            nestedTypes: method.nestedTypes,
            httpMethod: method.httpMethod,
            path: method.path
        };
        return crypto.createHash('sha256')
            .update(JSON.stringify(data))
            .digest('hex');
    }

    /**
     * Get cached code example if available
     * @param {string} cacheKey - Cache key
     * @returns {string|null} - Cached code example or null
     */
    getCachedExample(cacheKey) {
        const cacheFile = path.join(this.cachePath, `${cacheKey}.json`);

        if (fs.existsSync(cacheFile)) {
            try {
                const cached = JSON.parse(fs.readFileSync(cacheFile, 'utf8'));
                console.log(`Using cached example for ${cached.method}`);
                return cached.codeExample;
            } catch (e) {
                console.warn(`Failed to read cache: ${e.message}`);
            }
        }

        return null;
    }

    /**
     * Save code example to cache
     * @param {string} cacheKey - Cache key
     * @param {Object} data - Data to cache
     */
    saveToCache(cacheKey, data) {
        const cacheFile = path.join(this.cachePath, `${cacheKey}.json`);

        try {
            fs.writeFileSync(cacheFile, JSON.stringify(data, null, 2), 'utf8');
        } catch (e) {
            console.warn(`Failed to write cache: ${e.message}`);
        }
    }

    /**
     * Build prompt for OpenAI
     * @param {Object} method - Method metadata
     * @returns {string} - Prompt text
     */
    buildPrompt(method) {
        const lines = [];

        lines.push(`Create a TypeScript code example that calls the async function "${method.name}".`);
        lines.push('');
        lines.push('Pretend this function is globally available (do not import it or create an API instance).');
        lines.push('');

        lines.push('Function Parameters:');
        if (method.parameters && Object.keys(method.parameters).length > 0) {
            for (const [name, info] of Object.entries(method.parameters)) {
                const required = info.required ? 'required' : 'optional';
                lines.push(`  - ${name}: ${info.type} (${required})`);
            }
        } else {
            lines.push('  (none)');
        }

        lines.push('');
        lines.push(`Return Type: ${method.responseType || 'void'}`);

        if (method.nestedTypes && Object.keys(method.nestedTypes).length > 0) {
            lines.push('');
            lines.push('Type Definitions:');
            for (const [typeName, typeDef] of Object.entries(method.nestedTypes)) {
                lines.push(`  ${typeName}: ${typeDef}`);
            }
        }

        lines.push('');
        lines.push('Requirements:');
        lines.push('1. Do NOT include any imports');
        lines.push('2. Do NOT create an API instance or configuration');
        lines.push('3. Do NOT include error handling (no try/catch)');
        lines.push('4. Use realistic parameter values (not just \'example\' strings)');
        lines.push('5. Show the function call with await and assign the result to a typed variable');
        lines.push('6. Use the correct return type for the result variable (from Return Type above)');
        lines.push('7. Use TypeScript type annotations for all variables');
        lines.push('8. Demonstrate optional parameters where relevant');
        lines.push('9. Keep example very concise (< 20 lines)');
        lines.push('');
        lines.push('Return only the TypeScript code, no explanations or markdown formatting.');

        return lines.join('\n');
    }

    /**
     * Call OpenAI API to generate code example
     * @param {Object} method - Method metadata
     * @returns {Promise<string>} - Generated code example
     */
    async generateCodeExample(method) {
        // Check cache first
        const cacheKey = this.generateCacheKey(method);
        const cached = this.getCachedExample(cacheKey);
        if (cached) {
            return cached;
        }

        console.log(`Generating code example for ${method.name} using ${this.model}...`);

        const prompt = this.buildPrompt(method);

        try {
            const response = await fetch('https://api.openai.com/v1/chat/completions', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${this.apiKey}`
                },
                body: JSON.stringify({
                    model: this.model,
                    messages: [
                        {
                            role: 'system',
                            content: 'You are an expert TypeScript developer generating realistic API usage examples for the FastComments API.'
                        },
                        {
                            role: 'user',
                            content: prompt
                        }
                    ],
                    max_completion_tokens: 8000
                })
            });

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`OpenAI API error: ${response.status} ${errorText}`);
            }

            const data = await response.json();
            const codeExample = data.choices?.[0]?.message?.content?.trim() || '';

            // Remove markdown code fences if present
            const cleanCode = codeExample
                .replace(/^```typescript\n/, '')
                .replace(/^```ts\n/, '')
                .replace(/^```\n/, '')
                .replace(/\n```$/, '');

            // Save to cache
            this.saveToCache(cacheKey, {
                method: method.name,
                signatureHash: cacheKey,
                generatedAt: new Date().toISOString(),
                codeExample: cleanCode,
                model: this.model,
                promptTokens: data.usage?.prompt_tokens,
                completionTokens: data.usage?.completion_tokens
            });

            console.log(`  ✓ Generated (${data.usage?.total_tokens || 0} tokens)`);

            return cleanCode;
        } catch (error) {
            console.error(`  ✗ Failed: ${error.message}`);
            return null;
        }
    }

    /**
     * Generate code examples for multiple methods with concurrent processing
     * @param {Array<Object>} methods - Array of method metadata
     * @param {number} concurrency - Number of concurrent workers (default: 10)
     * @returns {Promise<Map<string, string>>} - Map of method name to code example
     */
    async generateCodeExamples(methods, concurrency = 10) {
        const results = new Map();
        let currentIndex = 0;

        // Function to get next method
        const next = () => {
            if (currentIndex < methods.length) {
                const method = methods[currentIndex];
                currentIndex++;
                return method;
            }
            return null;
        };

        // Worker function that processes methods until none left
        const worker = async () => {
            while (true) {
                const method = next();
                if (!method) break;

                const example = await this.generateCodeExample(method);
                if (example) {
                    results.set(method.name, example);
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

        return results;
    }
}

module.exports = OpenAIClient;
