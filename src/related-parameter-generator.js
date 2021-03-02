const vm = require('vm');
const {encode} = require('html-entities');

const StartToken = '[related-parameter-start';
const EndToken = 'related-parameter-end]';

function getTemplate(name, type, filePath) {
    return `<div class="related-parameter">Related Parameter in Code: <span>${name}</span> <span class="as">as</span> <span>${encode(type)}</span></div>`;
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

        input = input.substring(0, nextIndex) + getTemplate(args.name, args.type, filePath) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

module.exports = process;
