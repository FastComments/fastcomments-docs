const vm = require('vm');
const hljs = require('highlight.js');
const path = require('path');
const {createCodeSnippetPage} = require('./code-page-generator');
const {storeCodeSnippet} = require('./code-snippet-storage');

const StartToken = '[inline-code-start]';
const EndToken = '[inline-code-end]';

const StartAttrsToken = '[inline-code-attrs-start';
const EndAttrsToken = 'inline-code-attrs-end]';

function getTemplateLinesWithHighlight(inputString, linesToHighlight, useDemoTenant) {
    let result = '';
    const inputSplitByLine = inputString.split('\n');
    for (let i = 0; i < inputSplitByLine.length; i++) {
        const lineContent = inputSplitByLine[i];
        const classes = ['line'];
        if (linesToHighlight && linesToHighlight.includes(i)) {
            classes.push('highlight');
        }
        if (lineContent.includes('tenantId') && !useDemoTenant) {
            classes.push('has-tenant-id'); // this is to optimize a query to the DOM that replaces the example tenant id with the real one
        }
        // using line-content as hack because for some reason span breaks
        result += '<div class="' + classes.join(' ') + '">' + '<span class="line-number">' + (i + 1) + '</span><line-content class="line-content">' + inputSplitByLine[i] + '</line-content></div>';
    }

    return result;
}

function getTemplate(inlineCode, title, type, isFunctional, filePath, codeSnippetName, examplePageFileName, useDemoTenant) {
    let html = `<div class="code language-${type}">`;
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
    html += getTemplateLinesWithHighlight(hljs.highlight(type, inlineCode).value, [], useDemoTenant);
    html += '</div>'; // code-content

    html += '</div>'; // code

    return html;
}

function process(input, filePath) {
    let nextIndex = input.indexOf(StartToken);
    while(nextIndex > -1) {
        const endTokenIndex = input.indexOf(EndToken);
        if (endTokenIndex === -1) {
            throw new Error('Malformed input! Start token found, but not end.');
        }

        let inlineCode = input.substring(nextIndex + StartToken.length, endTokenIndex);

        const attrsIndex = input.indexOf(StartAttrsToken);
        const attrsEndTokenIndex = input.indexOf(EndAttrsToken);

        const attrsCode = input.substring(attrsIndex + StartAttrsToken.length, attrsEndTokenIndex);
        const args = {};
        vm.createContext(args); // Contextify the object.
        try {
            args.globals = {};
            vm.runInContext(attrsCode, args);
            console.log('args are', args);
        } catch(e) {
            console.error(e);
            throw new Error(`Malformed input! Value between start/end tokens should be valid javascript. ${attrsCode} given.`);
        }

        delete args.globals;

        const codeSnippetName = `code-${path.basename(filePath).replace('.md', '')}-${args.title.replace(new RegExp(' ', 'g'), '')}`;
        const codeSnippetPageFileName = `${codeSnippetName}.html`;
        const isFunctional = args.isFunctional === undefined || args.isFunctional === true;
        if (isFunctional) {
            createCodeSnippetPage(inlineCode, args.title, codeSnippetPageFileName);
        } else { // OPTIMIZATION
            inlineCode = inlineCode.replace(new RegExp('window.', 'g'), '');
        }

        input = input.substring(0, nextIndex) + getTemplate(inlineCode, args.title, args.type ? args.type : 'html', isFunctional, filePath, codeSnippetName, codeSnippetPageFileName, args.useDemoTenant) + input.substring(endTokenIndex + EndToken.length, input.length);
        if (attrsIndex > -1) {
            // remove the StartAttrsToken + EndAttrsToken
            // but do it after other string manipulation, so logic is simpler
            input = input.substring(0, input.indexOf(StartAttrsToken)) + input.substring(input.indexOf(EndAttrsToken) + EndAttrsToken.length, input.length);
        }

        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

module.exports = process;
