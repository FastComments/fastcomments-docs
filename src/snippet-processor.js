// Processor that includes HTML snippets from the snippets folder
// Usage: [snippet id="filename-without-extension"]
// This processor runs AFTER markdown, replacing snippet placeholders with actual HTML content

const fs = require('fs');
const path = require('path');

const SNIPPETS_DIR = path.join(__dirname, 'snippets');
const SNIPPET_REGEX = /\[snippet\s+id=(?:&quot;|")([^"&]+)(?:&quot;|")\]/g;

function process(html, filePath) {
    // Replace all snippet placeholders with their content
    return html.replace(SNIPPET_REGEX, (match, snippetId) => {
        const snippetPath = path.join(SNIPPETS_DIR, `${snippetId}.html`);
        
        try {
            // Read the snippet file
            const snippetContent = fs.readFileSync(snippetPath, 'utf8');
            console.log(`Loaded snippet: ${snippetId}`);
            return snippetContent;
        } catch (error) {
            console.error(`Failed to load snippet '${snippetId}':`, error.message);
            // Return an error message in the HTML
            return `<div style="color: red; border: 1px solid red; padding: 10px;">
                Error: Could not load snippet '${snippetId}'
            </div>`;
        }
    });
}

module.exports = process;