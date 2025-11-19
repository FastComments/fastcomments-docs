const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const OPENAI_API_KEY = process.env.OPENAI_API_KEY;
const OPENAI_MODEL = process.env.OPENAI_MODEL || 'gpt-5-mini';

/**
 * Client for generating code examples using OpenAI API
 */
class OpenAIClient {
    constructor(cachePath, language = 'typescript') {
        this.cachePath = cachePath;
        this.apiKey = OPENAI_API_KEY;
        this.model = OPENAI_MODEL;
        this.language = language;

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
     * Get system message based on language
     * @returns {string} - System message
     */
    getSystemMessage() {
        if (this.language === 'rust') {
            return 'You are an expert Rust developer generating realistic, idiomatic API usage examples for the FastComments API.';
        }
        if (this.language === 'cpp') {
            return 'You are an expert C++ developer generating realistic, idiomatic API usage examples for the FastComments API.';
        }
        if (this.language === 'nim') {
            return 'You are an expert Nim developer generating realistic, idiomatic API usage examples for the FastComments API.';
        }
        return 'You are an expert TypeScript developer generating realistic API usage examples for the FastComments API.';
    }

    /**
     * Build prompt for OpenAI
     * @param {Object} method - Method metadata
     * @returns {string} - Prompt text
     */
    buildPrompt(method) {
        if (this.language === 'rust') {
            return this.buildRustPrompt(method);
        }
        if (this.language === 'cpp') {
            return this.buildCppPrompt(method);
        }
        if (this.language === 'nim') {
            return this.buildNimPrompt(method);
        }
        return this.buildTypeScriptPrompt(method);
    }

    /**
     * Build TypeScript-specific prompt
     * @param {Object} method - Method metadata
     * @returns {string} - Prompt text
     */
    buildTypeScriptPrompt(method) {
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
     * Build Rust-specific prompt
     * @param {Object} method - Method metadata
     * @returns {string} - Prompt text
     */
    buildRustPrompt(method) {
        const lines = [];

        lines.push(`Create an idiomatic Rust code example that calls the async function "${method.name}".`);
        lines.push('');
        lines.push('The function signature is:');
        lines.push(`pub async fn ${method.name}(configuration: &configuration::Configuration, params: ${method.paramsType}) -> Result<${method.responseType}, Error>`);
        lines.push('');

        lines.push('Function Parameters (inside the params struct):');
        if (method.parameters && Object.keys(method.parameters).length > 0) {
            for (const [name, info] of Object.entries(method.parameters)) {
                const required = info.required ? 'required' : 'optional (Option<T>)';
                lines.push(`  - ${name}: ${info.type} (${required})`);
            }
        } else {
            lines.push('  (none)');
        }

        lines.push('');
        lines.push(`Return Type: Result<${method.responseType || 'unit'}, Error>`);

        if (method.nestedTypes && Object.keys(method.nestedTypes).length > 0) {
            lines.push('');
            lines.push('Type Definitions:');
            for (const [typeName, typeDef] of Object.entries(method.nestedTypes)) {
                lines.push(`  ${typeName}: ${typeDef}`);
            }
        }

        lines.push('');
        lines.push('Requirements:');
        lines.push('1. Do NOT include any use statements or imports');
        lines.push('2. Assume configuration and all types are already in scope');
        lines.push('3. Create a params struct instance with realistic values');
        lines.push('4. Use realistic parameter values (not "example_string" - use actual realistic values like "acme-corp-tenant", "news/article", etc.)');
        lines.push('5. Call the function with .await and use ? operator to unwrap the Result');
        lines.push('6. Use proper Rust type annotations and ownership semantics');
        lines.push('7. Demonstrate optional parameters with Some(...) where relevant');
        lines.push('8. Keep example very concise (< 25 lines)');
        lines.push('9. Use idiomatic Rust style (snake_case, proper formatting)');
        lines.push('10. Do NOT add any comments or explanations in the code');
        lines.push('');
        lines.push('Return only the Rust code, no explanations or markdown formatting.');

        return lines.join('\n');
    }

    /**
     * Build C++-specific prompt
     * @param {Object} method - Method metadata
     * @returns {string} - Prompt text
     */
    buildCppPrompt(method) {
        const lines = [];

        lines.push(`Create an idiomatic C++ code example that calls the async method "${method.name}" from the FastComments C++ SDK.`);
        lines.push('');
        lines.push('The method returns:');
        lines.push(`pplx::task<std::shared_ptr<${method.responseType || 'void'}>>`);
        lines.push('');

        lines.push('Function Parameters:');
        if (method.parameters && Object.keys(method.parameters).length > 0) {
            for (const [name, info] of Object.entries(method.parameters)) {
                const required = info.required ? 'required' : 'optional (boost::optional<T>)';
                lines.push(`  - ${name}: ${info.type} (${required})`);
            }
        } else {
            lines.push('  (none)');
        }

        lines.push('');
        lines.push(`Return Type: pplx::task<std::shared_ptr<${method.responseType || 'void'}>>`);

        if (method.nestedTypes && Object.keys(method.nestedTypes).length > 0) {
            lines.push('');
            lines.push('Type Definitions:');
            for (const [typeName, typeDef] of Object.entries(method.nestedTypes)) {
                lines.push(`  ${typeName}: ${typeDef.summary || 'Type definition'}`);
            }
        }

        lines.push('');
        lines.push('Requirements:');
        lines.push('1. Do NOT include any #include statements or namespace declarations');
        lines.push('2. Assume an API client instance named "api" is already created and in scope');
        lines.push('3. Use realistic parameter values (not "example_string" - use actual realistic values like "my-tenant-123", "user@example.com", etc.)');
        lines.push('4. Call the method using api->${method.name}(...) and chain with .then() to handle the result');
        lines.push('5. Use proper C++ types: utility::string_t for strings, std::make_shared for shared pointers');
        lines.push('6. Demonstrate optional parameters with boost::optional where relevant');
        lines.push('7. Keep example very concise (< 25 lines)');
        lines.push('8. Use idiomatic C++ style and proper formatting');
        lines.push('9. Do NOT add any comments or explanations in the code');
        lines.push('10. The SDK uses cpprest (Microsoft C++ REST SDK), so use utility::string_t for strings');
        lines.push('');
        lines.push('Return only the C++ code, no explanations or markdown formatting.');

        return lines.join('\n');
    }

    /**
     * Build Nim-specific prompt
     * @param {Object} method - Method metadata
     * @returns {string} - Prompt text
     */
    buildNimPrompt(method) {
        const lines = [];

        lines.push(`Create an idiomatic Nim code example that calls the function "${method.name}" from the FastComments Nim SDK.`);
        lines.push('');
        lines.push('The function returns:');
        lines.push(`(Option[${method.responseType || 'void'}], Response)`);
        lines.push('');

        lines.push('Function Parameters:');
        if (method.parameters && Object.keys(method.parameters).length > 0) {
            for (const [name, info] of Object.entries(method.parameters)) {
                // Skip httpClient parameter as it's just the client instance
                if (name === 'httpClient') continue;
                const required = info.required ? 'required' : 'optional';
                lines.push(`  - ${name}: ${info.type} (${required})`);
            }
        } else {
            lines.push('  (none)');
        }

        lines.push('');
        lines.push(`Return Type: (Option[${method.responseType || 'void'}], Response)`);

        if (method.nestedTypes && Object.keys(method.nestedTypes).length > 0) {
            lines.push('');
            lines.push('Type Definitions:');
            for (const [typeName, typeDef] of Object.entries(method.nestedTypes)) {
                lines.push(`  ${typeName}: ${typeDef.summary || 'Type definition'}`);
            }
        }

        lines.push('');
        lines.push('Requirements:');
        lines.push('1. Do NOT include any import statements');
        lines.push('2. Assume an HttpClient instance named "client" is already created and in scope');
        lines.push('3. CRITICAL: You MUST use named arguments for ALL function parameters (e.g., client.getCommentsPublic(tenantId = "...", urlId = "...", page = 0, ...))');
        lines.push('4. Use realistic parameter values (not "example_string" - use actual realistic values like "my-tenant-123", "news/article-title", etc.)');
        lines.push('5. Call the function and destructure the result tuple like: let (response, httpResponse) = client.${method.name}(...)');
        lines.push('6. Check if response.isSome and access the value with response.get()');
        lines.push('7. Use proper Nim types: string, int, bool, seq[string] for arrays');
        lines.push('8. For optional/default parameters, pass appropriate default values (0 for int, "" for string, false for bool, @[] for seq, etc.)');
        lines.push('9. Keep example very concise (< 30 lines)');
        lines.push('10. Use idiomatic Nim style (camelCase for variables, proper indentation with 2 spaces)');
        lines.push('11. Do NOT add any comments or explanations in the code');
        lines.push('12. IMPORTANT: ALWAYS use named arguments like paramName = value for every parameter');
        lines.push('');
        lines.push('Return only the Nim code, no explanations or markdown formatting.');

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
                            content: this.getSystemMessage()
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
                .replace(/^```rust\n/, '')
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
