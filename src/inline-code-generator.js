const vm = require('vm');
const hljs = require('highlight.js');
const path = require('path');
const {createCodeSnippetPage} = require('./code-page-generator');

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
        result += '<div class="' + classes.join(' ') + '">' + '<span class="line-number">' + (i + 1) + '</span>' + inputSplitByLine[i] + '</div>';
    }

    return result;
}

function getTemplate(inlineCode, title, type, isFunctional, filePath, examplePageFileName, useDemoTenant) {
    let html = '<div class="code">';
    html += `<div class="title">${title}</div>`;

    if (isFunctional) {
        html += `<div class="contribute-code-snippet"><a href="/${examplePageFileName}" target="_blank"><img src="/images/link-external.png" alt="External Link" title="Run This Code Snippet"></a></div>`;
    }

    html += getTemplateLinesWithHighlight(hljs.highlight(type, inlineCode).value, [], useDemoTenant);

    html += '</div>';

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

        const codeSnippetPageFileName = `code-${path.basename(filePath).replace('.md', '')}-${args.title.replace(new RegExp(' ', 'g'), '')}.html`;
        const isFunctional = args.isFunctional === undefined || args.isFunctional === true;
        if (isFunctional) {
            createCodeSnippetPage(inlineCode, args.title, codeSnippetPageFileName);
        } else { // OPTIMIZATION
            inlineCode = inlineCode.replace(new RegExp('window.', 'g'), '');
        }

        input = input.substring(0, nextIndex) + getTemplate(inlineCode, args.title, args.type ? args.type : 'html', isFunctional, filePath, codeSnippetPageFileName, args.useDemoTenant) + input.substring(endTokenIndex + EndToken.length, input.length);
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
