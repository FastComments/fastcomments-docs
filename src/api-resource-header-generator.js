const vm = require('vm');

const StartToken = '[api-resource-header-start';
const EndToken = 'api-resource-header-end]';

function getTemplate(name, route, costPerPageLoad) {
    return `<div class="api-resource-header">Resource: <span>${name}</span> <span class="as">as</span> <span>${route}</span> Cost: <span>1 Page Load = ${Number(costPerPageLoad)} ${name} Max</span></div>`;
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

        input = input.substring(0, nextIndex) + getTemplate(args.name, args.route, args.costPerPageLoad) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

module.exports = process;
