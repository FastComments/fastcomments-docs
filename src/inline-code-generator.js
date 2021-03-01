const vm = require('vm');
const hljs = require('highlight.js');
const path = require('path');
const {createCodeSnippetPage} = require('./code-page-generator');

const StartToken = '[inline-code-start]';
const EndToken = '[inline-code-end]';

const StartAttrsToken = '[inline-code-attrs-start';
const EndAttrsToken = 'inline-code-attrs-end]';

function getTemplateLinesWithHighlight(inputString, linesToHighlight) {
    let result = '';
    const inputSplitByLine = inputString.split('\n');
    for (let i = 0; i < inputSplitByLine.length; i++) {
        result += '<div class="line' + (linesToHighlight.includes(i) ? ' highlight' : '') + '">' + '<span class="line-number">' + (i + 1) + '</span>' + inputSplitByLine[i] + '</div>';
    }

    return result;
}

function getTemplate(inlineCode, title, filePath, examplePageFileName) {
    let html = '<div class="code">';
    html += `<div class="title">${title}</div>`;
    html += `<div class="contribute-code-snippet"><a href="/${examplePageFileName}" target="_blank"><img src="/images/link-external.png" alt="External Link" title="Run This Code Snippet"></a></div>`;

    html += getTemplateLinesWithHighlight(hljs.highlight('html', inlineCode).value, []);

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

        const inlineCode = input.substring(nextIndex + StartToken.length, endTokenIndex);

        const attrsIndex = input.indexOf(StartAttrsToken);
        const attrsEndTokenIndex = input.indexOf(EndAttrsToken);

        const attrsCode = input.substring(attrsIndex + StartAttrsToken.length, attrsEndTokenIndex);
        const args = {};
        vm.createContext(args); // Contextify the object.
        try {
            vm.runInContext(attrsCode, args);
            console.log('args are', args);
        } catch(e) {
            console.error(e);
            throw new Error(`Malformed input! Value between start/end tokens should be valid javascript. ${attrsCode} given.`);
        }

        const codeSnippetPageFileName = `code-${path.basename(filePath).replace('.md', '')}-${args.title.replace(new RegExp(' ', 'g'), '')}.html`;
        createCodeSnippetPage(inlineCode, args.title, codeSnippetPageFileName);

        input = input.substring(0, nextIndex) + getTemplate(inlineCode, args.title, filePath, codeSnippetPageFileName) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

module.exports = process;
