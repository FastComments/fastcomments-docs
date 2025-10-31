const fs = require('fs');
const path = require('path');

const CSS_URL = 'https://fastcomments.com/js/shared/css-v2.js';
const GUIDE_DIR = path.join(__dirname, 'content', 'guides', 'custom-styling');
const ITEMS_DIR = path.join(GUIDE_DIR, 'items');

/**
 * Fetch and extract CSS from the FastComments widget JavaScript file
 */
async function fetchCSS() {
    console.log(`Fetching CSS from ${CSS_URL}...`);

    const response = await fetch(CSS_URL);
    if (!response.ok) {
        throw new Error(`Failed to fetch CSS: ${response.status} ${response.statusText}`);
    }

    const jsContent = await response.text();

    // Extract CSS from: export default `<style>...css...</style>`
    const match = jsContent.match(/export\s+default\s+`<style>([\s\S]*?)<\/style>`/);

    if (!match) {
        throw new Error('Could not extract CSS from JavaScript file');
    }

    return match[1].trim();
}

/**
 * Generate markdown content with CSS
 */
function generateMarkdown(css) {
    const lines = [];

    lines.push('[inline-code-attrs-start title = \'Comment Widget Default CSS\'; type = \'css\'; isFunctional = false; inline-code-attrs-end]');
    lines.push('[inline-code-start]');
    lines.push(css);
    lines.push('[inline-code-end]');

    return lines.join('\n');
}

/**
 * Main generation function
 */
async function generate() {
    try {
        console.log('Starting CSS v2 guide generation...');

        // Create directories if they don't exist
        if (!fs.existsSync(GUIDE_DIR)) {
            fs.mkdirSync(GUIDE_DIR, { recursive: true });
        }
        if (!fs.existsSync(ITEMS_DIR)) {
            fs.mkdirSync(ITEMS_DIR, { recursive: true });
        }

        // Fetch CSS
        const css = await fetchCSS();
        console.log(`Extracted ${css.length} characters of CSS`);

        // Generate markdown
        const markdown = generateMarkdown(css);

        // Write file
        const outputFile = path.join(ITEMS_DIR, 'css-v2-generated.md');
        fs.writeFileSync(outputFile, markdown, 'utf8');
        console.log(`Successfully generated ${outputFile}`);

        console.log('CSS v2 guide generation complete!');
    } catch (error) {
        console.error('Failed to generate CSS v2 guide:', error.message);
        process.exit(1);
    }
}

// Allow running directly from command line
if (require.main === module) {
    generate().catch(e => {
        console.error('Fatal error:', e);
        process.exit(1);
    });
}

module.exports = { generate };
