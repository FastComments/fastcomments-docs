const path = require('path');
const fs = require('fs');
const {ExampleTenantId} = require('./utils');
const {getCompiledTemplate} = require('./utils');

const first = (arr) => arr && arr.length > 0 ? arr[0] : undefined;
const last = (arr) => arr && arr.length > 0 ? arr[arr.length - 1] : undefined;

const STATIC_GENERATED_DIR = path.join(__dirname, 'static/generated');
const TEMPLATE_DIR = path.join(__dirname, 'templates');

function createCodeSnippetPage (codeHTML, snippetName, targetFileName, linesToHighlight) {
    fs.writeFileSync(path.join(STATIC_GENERATED_DIR, targetFileName), getCompiledTemplate(path.join(TEMPLATE_DIR, 'code.html'), {
        codeHTML,
        snippetName,
        highLightLineFrom: linesToHighlight ? first(linesToHighlight) : 0,
        highLightLineTo: linesToHighlight ? last(linesToHighlight) : 0,
        ExampleTenantId
    }), 'utf8');
}

module.exports = { createCodeSnippetPage };
