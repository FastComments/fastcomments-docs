const path = require('path');
const fs = require('fs');
const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated/snippets');

if (!fs.existsSync(STATIC_GENERATED_DIR)) {
    fs.mkdirSync(STATIC_GENERATED_DIR, { recursive: true});
}

function storeCodeSnippet (snippet, targetFileName) {
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, targetFileName), snippet, 'utf8');
}

module.exports = { storeCodeSnippet };
