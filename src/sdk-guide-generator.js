const fs = require('fs');
const path = require('path');
const SDKCheckoutManager = require('./sdk-checkout-manager');
const ReadmeDocGenerator = require('./sdk-doc-generators/readme-generator');
const OpenAPIDocGenerator = require('./sdk-doc-generators/openapi-generator');

const GUIDES_DIR = path.join(__dirname, 'content', 'guides');

/**
 * Generates guide structure from SDK documentation
 */
class SDKGuideGenerator {
    constructor() {
        this.checkoutManager = new SDKCheckoutManager();
    }

    /**
     * Get the appropriate documentation generators for an SDK
     * @param {Object} sdk - SDK configuration
     * @param {string} repoPath - Path to checked out repository
     * @returns {Array<BaseDocGenerator>}
     */
    getDocGenerators(sdk, repoPath) {
        const generators = [];

        // Support both old single generator and new multiple generators
        const generatorTypes = sdk.docGenerators || [sdk.docGenerator || 'readme'];

        for (const generatorType of generatorTypes) {
            switch (generatorType) {
                case 'readme':
                    generators.push(new ReadmeDocGenerator(sdk, repoPath));
                    break;
                case 'openapi':
                    generators.push(new OpenAPIDocGenerator(sdk, repoPath));
                    break;
                case 'typescript-ai':
                    const TypeScriptAIGenerator = require('./sdk-doc-generators/typescript-ai-generator');
                    generators.push(new TypeScriptAIGenerator(sdk, repoPath));
                    break;
                case 'rust-ai':
                    const RustAIGenerator = require('./sdk-doc-generators/rust-ai-generator');
                    generators.push(new RustAIGenerator(sdk, repoPath));
                    break;
                case 'cpp-ai':
                    const CppAIGenerator = require('./sdk-doc-generators/cpp-ai-generator');
                    generators.push(new CppAIGenerator(sdk, repoPath));
                    break;
                case 'nim-ai':
                    const NimAIGenerator = require('./sdk-doc-generators/nim-ai-generator');
                    generators.push(new NimAIGenerator(sdk, repoPath));
                    break;
                default:
                    console.warn(`Unknown generator type: ${generatorType}`);
            }
        }

        return generators;
    }

    /**
     * Create guide directory structure for an SDK
     * @param {Object} sdk - SDK configuration
     */
    createGuideDirectory(sdk) {
        const guideDir = path.join(GUIDES_DIR, sdk.id);
        const itemsDir = path.join(guideDir, 'items');
        const enDir = path.join(itemsDir, 'en');

        // Create directories if they don't exist
        if (!fs.existsSync(guideDir)) {
            fs.mkdirSync(guideDir, { recursive: true });
        }
        if (!fs.existsSync(itemsDir)) {
            fs.mkdirSync(itemsDir, { recursive: true });
        }

        // Create en directory if it doesn't exist (default locale for generated docs)
        if (!fs.existsSync(enDir)) {
            fs.mkdirSync(enDir, { recursive: true });
        }

        return { guideDir, itemsDir, generatedDir: enDir };
    }

    /**
     * Generate meta.json for an SDK guide
     * @param {Object} sdk - SDK configuration
     * @param {Array<DocSection>} sections - Documentation sections
     * @returns {Object} - meta.json content
     */
    generateMeta(sdk, sections) {
        // Define priority categories (lower number = higher priority)
        const categoryPriority = {
            'Getting Started': 1,
            'Documentation': 2,
            'Usage': 3,
            'API Reference': 4
        };

        // Separate README and API sections
        const readmeSections = sections.filter(s => (s.type || 'readme') === 'readme');
        const apiSections = sections.filter(s => s.type === 'api');

        // README sections maintain their original order (no sorting)
        // API sections are sorted by category priority, then category name, then item name
        const sortedApiSections = apiSections.slice().sort((a, b) => {
            const catA = a.subCat || 'Documentation';
            const catB = b.subCat || 'Documentation';

            // Get priority (default to 999 for non-priority categories)
            const priorityA = categoryPriority[catA] || 999;
            const priorityB = categoryPriority[catB] || 999;

            // First sort by category priority
            if (priorityA !== priorityB) {
                return priorityA - priorityB;
            }

            // If same priority, sort by category name alphabetically
            if (catA !== catB) {
                return catA.localeCompare(catB);
            }

            // Within same category, sort by item name alphabetically
            return a.name.localeCompare(b.name);
        });

        // Combine: README sections first (in original order), then API sections (sorted)
        const sortedSections = [...readmeSections, ...sortedApiSections];

        const itemsOrdered = sortedSections.map(section => ({
            name: section.name,
            // Use section.file if provided (for generated docs), otherwise sanitize name
            file: section.file || (this.sanitizeFilename(section.name) + '.md'),
            subCat: section.subCat || 'Documentation',
            type: section.type || 'readme',
            ...(section.sidebarItemClasses ? { sidebarItemClasses: section.sidebarItemClasses } : {})
        }));

        return {
            name: sdk.name,
            pageHeader: sdk.pageHeader || sdk.name,
            icon: sdk.icon,
            itemsOrdered
        };
    }

    /**
     * Sanitize filename
     * @param {string} name - Original name
     * @returns {string} - Sanitized filename
     */
    sanitizeFilename(name) {
        return name
            .toLowerCase()
            .replace(/[^a-z0-9]+/g, '-')
            .replace(/^-+|-+$/g, '');
    }

    /**
     * Generate guide for a single SDK
     * @param {Object} sdk - SDK configuration
     * @param {string} repoPath - Path to checked out repository
     */
    async generateGuideForSDK(sdk, repoPath) {
        console.log(`Generating guide for ${sdk.name}...`);

        // Create guide directory structure FIRST (before generators run)
        const { guideDir, itemsDir, generatedDir } = this.createGuideDirectory(sdk);

        // Get all documentation generators
        const generators = this.getDocGenerators(sdk, repoPath);

        // Run all generators and merge their output
        let mergedIntro = '';
        let mergedConclusion = '';
        const allSections = [];

        for (const generator of generators) {
            try {
                console.log(`Running ${generator.constructor.name}...`);
                const docs = await generator.generate();

                // Use intro/conclusion from first generator that provides them
                if (docs.intro && !mergedIntro) {
                    mergedIntro = docs.intro;
                }
                if (docs.conclusion && !mergedConclusion) {
                    mergedConclusion = docs.conclusion;
                }

                // Merge sections
                allSections.push(...docs.sections);
            } catch (e) {
                console.error(`Error running ${generator.constructor.name}:`, e.message);
            }
        }

        // Write intro.md to en locale folder
        if (mergedIntro) {
            fs.writeFileSync(
                path.join(generatedDir, 'intro.md'),
                mergedIntro,
                'utf8'
            );
        }

        // Write conclusion.md to en locale folder
        if (mergedConclusion) {
            fs.writeFileSync(
                path.join(generatedDir, 'conclusion.md'),
                mergedConclusion,
                'utf8'
            );
        }

        // Write section files to the en locale subfolder
        for (const section of allSections) {
            // Use custom filename if provided, otherwise generate from name
            // Strip 'generated/' prefix if present (for backwards compatibility)
            let filename = section.file || (this.sanitizeFilename(section.name) + '.md');
            if (filename.startsWith('generated/')) {
                filename = filename.slice('generated/'.length);
            }
            const filePath = path.join(generatedDir, filename);

            // Only write if file doesn't exist (TypeScript AI generator writes as it goes)
            if (!fs.existsSync(filePath)) {
                fs.writeFileSync(filePath, section.content, 'utf8');
            }

            // Update section.file to just be the filename (no prefix needed for locale structure)
            section.file = filename;
        }

        // Generate and write meta.json
        const meta = this.generateMeta(sdk, allSections);
        fs.writeFileSync(
            path.join(guideDir, 'meta.json'),
            JSON.stringify(meta, null, 2),
            'utf8'
        );

        console.log(`Successfully generated guide for ${sdk.name} with ${allSections.length} sections`);
    }

    /**
     * Generate guides for all configured SDKs
     * @param {string} sdkFilter - Optional SDK ID to generate for a specific SDK only
     */
    async generateAll(sdkFilter = null) {
        console.log('Starting SDK guide generation...');

        // Checkout all SDKs
        let checkouts = this.checkoutManager.checkoutAll();

        // Filter to specific SDK if requested
        if (sdkFilter) {
            checkouts = checkouts.filter(({ sdk }) => sdk.id === sdkFilter);
            if (checkouts.length === 0) {
                console.error(`SDK "${sdkFilter}" not found. Available SDKs: ${this.checkoutManager.getSDKs().map(s => s.id).join(', ')}`);
                process.exit(1);
            }
            console.log(`Filtering to SDK: ${sdkFilter}`);
        }

        // Track validation errors across all SDKs
        const validationErrors = [];

        // Generate guides for each SDK
        for (const { sdk, path: repoPath } of checkouts) {
            try {
                await this.generateGuideForSDK(sdk, repoPath);
            } catch (e) {
                console.error(`Failed to generate guide for ${sdk.id}:`, e.message);
                console.error(e.stack);

                // Track validation errors separately
                if (e.validationErrors) {
                    validationErrors.push({
                        sdk: sdk.id,
                        errors: e.validationErrors
                    });
                }
            }
        }

        console.log('SDK guide generation complete!');

        // Fail build if there were validation errors
        if (validationErrors.length > 0) {
            console.error('\n=== SDK DOCUMENTATION VALIDATION ERRORS ===\n');

            for (const { sdk, errors } of validationErrors) {
                console.error(`\n${sdk}:`);
                console.error(`  ${errors.length} method(s) not found in SDK documentation:\n`);

                for (const error of errors) {
                    console.error(`    - ${error.operation}`);
                    if (error.operationId) {
                        console.error(`      Operation ID: ${error.operationId}`);
                    }
                    console.error(`      Lookup key: ${error.lookupKey}`);
                }
            }

            console.error(`\n=== BUILD FAILED: ${validationErrors.length} SDK(s) have missing methods ===\n`);
            process.exit(1);
        }
    }

    /**
     * Clean up generated SDK guides
     */
    cleanGeneratedGuides() {
        const sdks = this.checkoutManager.getSDKs();

        for (const sdk of sdks) {
            const guideDir = path.join(GUIDES_DIR, sdk.id);
            if (fs.existsSync(guideDir)) {
                console.log(`Removing generated guide: ${sdk.id}`);
                fs.rmSync(guideDir, { recursive: true, force: true });
            }
        }
    }
}

// Allow running directly from command line
if (require.main === module) {
    const generator = new SDKGuideGenerator();

    // Check for clean command
    if (process.argv.includes('--clean')) {
        generator.cleanGeneratedGuides();
    } else {
        // Check for SDK filter argument
        const sdkIndex = process.argv.indexOf('--sdk');
        const sdkFilter = sdkIndex !== -1 ? process.argv[sdkIndex + 1] : null;

        generator.generateAll(sdkFilter).catch(e => {
            console.error('Failed to generate SDK guides:', e);
            process.exit(1);
        });
    }
}

module.exports = SDKGuideGenerator;
