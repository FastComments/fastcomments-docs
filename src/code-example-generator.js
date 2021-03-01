const path = require('path');
const vm = require('vm');
const hljs = require('highlight.js');
const {createCodeSnippetPage} = require('./code-page-generator');

const StartToken = '[code-example-start';
const EndToken = 'code-example-end]';

function getTemplateLinesWithHighlight(inputString, linesToHighlight) {
    let result = '';
    const inputSplitByLine = inputString.split('\n');
    for (let i = 0; i < inputSplitByLine.length; i++) {
        const lineContent = inputSplitByLine[i];
        const classes = ['line'];
        if (linesToHighlight.includes(i)) {
            classes.push('highlight');
        }
        if (lineContent.includes('tenantId')) {
            classes.push('has-tenant-id');
        }
        result += `<div class="${classes.join(' ')}"><span class="line-number">${i + 1}</span>${lineContent}</div>`;
    }

    return result;
}

function getTemplateWrapped(codeHTML, linesToHighlight, title, filePath, examplePageFileName) {
    let html = `<div class="code" id="${title.replace(new RegExp(' ', 'g'), '-').toLowerCase()}">`;
    html += `<div class="title">${title}</div>`;
    html += `<div class="contribute-code-snippet"><a href="/${examplePageFileName}" target="_blank"><img src="/images/link-external.png" alt="External Link" title="Run This Code Snippet"></a></div>`;

    html += getTemplateLinesWithHighlight(hljs.highlight('html', codeHTML).value, linesToHighlight);
    html += '</div>';

    return html;
}

function process(input, filePath) {
    let nextIndex = input.indexOf(StartToken);
    while (nextIndex > -1) {
        const endTokenIndex = input.indexOf(EndToken);
        if (endTokenIndex === -1) {
            throw new Error('Malformed input! Start token found, but not end.');
        }

        const code = input.substring(nextIndex + StartToken.length, endTokenIndex);
        const args = {};
        vm.createContext(args); // Contextify the object.
        try {
            vm.runInContext(code, args);
            console.log('args are', args);
        } catch (e) {
            console.error(e);
            throw new Error(`Malformed input! Value between start/end tokens should be valid javascript. ${code} given.`);
        }

        const codeHTML = `
<script src="https://cdn.fastcomments.com/js/embed.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), ${JSON.stringify({
            tenantId: 'demo',
            ...args.config
        }, null, '    ')});
</script>
`;

        const codeSnippetPageFileName = `code-${path.basename(filePath).replace('.md', '')}-${args.title.replace(new RegExp(' ', 'g'), '')}.html`;
        createCodeSnippetPage(codeHTML + (args.additionalDemoCode ? `\n${args.additionalDemoCode}` : ''), args.title, codeSnippetPageFileName, args.linesToHighlight);

        input = input.substring(0, nextIndex) + getTemplateWrapped(codeHTML, args.linesToHighlight, args.title, filePath, codeSnippetPageFileName) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

module.exports = process;
