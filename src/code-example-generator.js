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
        if (linesToHighlight && linesToHighlight.includes(i)) {
            classes.push('highlight');
        }
        if (lineContent.includes('tenantId')) {
            classes.push('has-tenant-id'); // this is to optimize a query to the DOM that replaces the example tenant id with the real one
        }
        if (lineContent.includes('http')) {
            classes.push('has-url');
        }
        // using line-content as hack because for some reason span breaks
        result += `<div class="${classes.join(' ')}"><span class="line-number">${i + 1}</span><line-content class="line-content">${lineContent}</line-content></div>`;
    }

    return result;
}

function getTemplateWrapped(codeHTML, linesToHighlight, title, isFunctional, filePath, codeSnippetName, examplePageFileName) {
    let html = `<div class="code" id="${title.replace(new RegExp(' ', 'g'), '-').toLowerCase()}">`;
    html += `<div class="code-head">`;
    html += `<div class="title">${title}</div>`;
    html += '<div class="top-right">';
    html += `<span class="copy" data-snippet-id="${codeSnippetName}"><span>Copy</span> <img src="/images/copy-white-24px.png" alt="Copy" title="Copy"></span>`;
    if (isFunctional) {
        html += `<a href="/${examplePageFileName}" target="_blank"><span>Run</span> <img src="/images/link-external.png" alt="External Link" title="Run This Code Snippet"></a>`;
    }
    html += '</div>'; // top-right
    html += '</div>'; // code-head

    html += `<div class="code-content">`;
    html += getTemplateLinesWithHighlight(hljs.highlight('html', codeHTML).value, linesToHighlight);
    html += '</div>'; // code-content
    html += '</div>'; // code

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
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
FastCommentsUI(document.getElementById('fastcomments-widget'), ${JSON.stringify({
            tenantId: 'demo',
            ...args.config
        }, null, '    ')});
</script>
`;
        const isFunctional = args.isFunctional === undefined || args.isFunctional === true;
        const codeSnippetName = `code-${path.basename(filePath).replace('.md', '')}-${args.title.replace(new RegExp(' ', 'g'), '')}`;
        const codeSnippetPageFileName = `${codeSnippetName}.html`;
        const fullCode = codeHTML + (args.additionalDemoCode ? `\n${args.additionalDemoCode}` : '');
        if (isFunctional) {
            createCodeSnippetPage(fullCode, args.title, codeSnippetPageFileName, args.linesToHighlight);
        }

        input = input.substring(0, nextIndex) + getTemplateWrapped(codeHTML, args.linesToHighlight, args.title, isFunctional, filePath, codeSnippetName, codeSnippetPageFileName) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

module.exports = process;
