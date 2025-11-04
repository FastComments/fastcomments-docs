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

        // Create directories if they don't exist
        if (!fs.existsSync(guideDir)) {
            fs.mkdirSync(guideDir, { recursive: true });
        }
        if (!fs.existsSync(itemsDir)) {
            fs.mkdirSync(itemsDir, { recursive: true });
        }

        // Clean up old files in items directory (will be regenerated)
        if (fs.existsSync(itemsDir)) {
            fs.rmSync(itemsDir, { recursive: true, force: true });
            fs.mkdirSync(itemsDir, { recursive: true });
        }

        return { guideDir, itemsDir };
    }

    /**
     * Generate meta.json for an SDK guide
     * @param {Object} sdk - SDK configuration
     * @param {Array<DocSection>} sections - Documentation sections
     * @returns {Object} - meta.json content
     */
    generateMeta(sdk, sections) {
        const itemsOrdered = sections.map(section => ({
            name: section.name,
            // Use section.file if provided (for generated docs), otherwise sanitize name
            file: section.file || (this.sanitizeFilename(section.name) + '.md'),
            subCat: section.subCat || 'Documentation',
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
        const { guideDir, itemsDir } = this.createGuideDirectory(sdk);

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

        // Write intro.md
        if (mergedIntro) {
            fs.writeFileSync(
                path.join(guideDir, 'intro.md'),
                mergedIntro,
                'utf8'
            );
        }

        // Write conclusion.md
        if (mergedConclusion) {
            fs.writeFileSync(
                path.join(guideDir, 'conclusion.md'),
                mergedConclusion,
                'utf8'
            );
        }

        // Write section files
        for (const section of allSections) {
            // Use custom filename if provided, otherwise generate from name
            const filename = section.file || (this.sanitizeFilename(section.name) + '.md');
            const filePath = path.join(itemsDir, filename);

            // Only write if file doesn't exist (TypeScript AI generator writes as it goes)
            if (!fs.existsSync(filePath)) {
                fs.writeFileSync(filePath, section.content, 'utf8');
            }
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
     */
    async generateAll() {
        console.log('Starting SDK guide generation...');

        // Checkout all SDKs
        const checkouts = this.checkoutManager.checkoutAll();

        // Generate guides for each SDK
        for (const { sdk, path: repoPath } of checkouts) {
            try {
                await this.generateGuideForSDK(sdk, repoPath);
            } catch (e) {
                console.error(`Failed to generate guide for ${sdk.id}:`, e.message);
                console.error(e.stack);
            }
        }

        console.log('SDK guide generation complete!');
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
        generator.generateAll().catch(e => {
            console.error('Failed to generate SDK guides:', e);
            process.exit(1);
        });
    }
}

module.exports = SDKGuideGenerator;
