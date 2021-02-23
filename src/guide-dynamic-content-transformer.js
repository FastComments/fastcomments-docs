// A fancy named file that takes things like:
// [code-example-start "{urlId: "https://example.com/page"}" code-example-end]
// And creates a code snippet that you can run with the given configuration added in.
// Processors can be async (for example, given a page and a selector, go take a screenshot)

const processors = [
    require('./code-example-generator'),
    require('./app-screenshot-generator'),
    require('./related-parameter-generator')
];

/**
 *
 * @param {string} markdown
 * @param {string} filePath
 * @return {Promise<string>}
 */
async function processDynamicContent(markdown, filePath) {
    for (const processor of processors) {
        try {
            markdown = await processor(markdown, filePath);
        } catch(e) {
            console.error('Processor failed', processor, markdown, e);
        }
    }

    return markdown;
}

async function dispose() {
    for (const processor of processors) {
        if (processor.dispose) {
            await processor.dispose();
        }
    }
}

module.exports = { processDynamicContent, dispose };
