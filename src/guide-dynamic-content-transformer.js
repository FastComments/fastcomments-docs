// A fancy named file that takes things like:
// [code-example-start "{urlId: "https://example.com/page"}" code-example-end]
// And creates a code snippet that you can run with the given configuration added in.
// Processors can be async (for example, given a page and a selector, go take a screenshot)

const processors = [
    require('./code-example-generator')
];

async function processDynamicContent(markdown) {
    for (const processor of processors) {
        markdown = await processor(markdown);
    }

    return markdown;
}

module.exports = { processDynamicContent };
