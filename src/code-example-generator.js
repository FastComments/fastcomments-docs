const vm = require('vm');
const {encode} = require('html-entities');

const Template = `some code goes here with...`;

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

function getTemplate(config, linesToHighlight) {
    let html = '<div class="code">';

    html += getTemplateLinesWithHighlight(encode(`
<script src="https://cdn.fastcomments.com/js/embed.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.FastCommentsUI(document.getElementById('fastcomments-widget'), ${JSON.stringify({
        tenantId: 'demo',
        ...config
    }, null, '    ')});
</script>
`), linesToHighlight);
    html += '</div>';

    return html;
}

function process(input) {
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

        input = input.substring(0, nextIndex) + getTemplate(args.config, args.linesToHighlight) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

module.exports = process;
