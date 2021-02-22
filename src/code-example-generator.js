const vm = require('vm');
const hljs = require('highlight.js');

const StartToken = '[code-example-start';
const EndToken = 'code-example-end]';

function getTemplateLinesWithHighlight(inputString, linesToHighlight) {
    let result = '';
    const inputSplitByLine = inputString.split('\n');
    for (let i = 0; i < inputSplitByLine.length; i++) {
        result += '<div class="line' + (linesToHighlight.includes(i) ? ' highlight' : '') + '">' + '<span class="line-number">' + (i + 1) + '</span>' + inputSplitByLine[i] + '</div>';
    }

    return result;
}

function getTemplate(config, linesToHighlight, filePath) {
    let html = '<div class="code">';
    html += `<div class="contribute-code-snippet"><a href="https://github.com/FastComments/fastcomments-docs/tree/main/${filePath}" target="_blank"><img src="/images/link-external.png" alt="External Link" title="Improve This Code Snippet"></a></div>`;

    const templateWithConfig = `
<script src="https://cdn.fastcomments.com/js/embed.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), ${JSON.stringify({
        tenantId: 'demo',
        ...config
    }, null, '    ')});
</script>
`;

    html += getTemplateLinesWithHighlight(hljs.highlight('html', templateWithConfig).value, linesToHighlight);
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

        const code = input.substring(nextIndex + StartToken.length, endTokenIndex);
        const args = {};
        vm.createContext(args); // Contextify the object.
        try {
            vm.runInContext(code, args);
            console.log('args are', args);
        } catch(e) {
            console.error(e);
            throw new Error(`Malformed input! Value between start/end tokens should be valid javascript. ${code} given.`);
        }

        input = input.substring(0, nextIndex) + getTemplate(args.config, args.linesToHighlight, filePath) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

module.exports = process;
