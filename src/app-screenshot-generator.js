const vm = require('vm');
const path = require('path');
const puppeteer = require('puppeteer');

const StartToken = '[app-screenshot-start';
const EndToken = 'app-screenshot-end]';

const HOST = 'https://fastcomments.com';

let authenticated = false;
let browser, page;

async function getTemplate(url, selector, title, filePath) {
    console.log('app-screenshot-generator Creating', url, selector);
    if (!authenticated) {
        browser = await puppeteer.launch({ headless: true });
        page = await browser.newPage();
        await page.goto(`${HOST}/auth/login`);
        await page.waitForSelector('form');
        await page.focus('input[name="username"]');
        await page.keyboard.type('demo');
        await page.focus('input[name="email"]');
        await page.keyboard.type('demo@fastcomments.com');
        await page.click('button[type="submit"]');
        await page.waitForSelector('body');
    }
    console.log('app-screenshot-generator authenticated...');
    await page.goto(`${HOST}${url}`);
    console.log('app-screenshot-generator loaded', url);
    await page.waitForSelector(selector);
    console.log('app-screenshot-generator found', selector);
    const element = await page.$(selector);

    const targetFileName = url.replace(new RegExp('/', 'g'), '') + selector.replace(new RegExp('\\.', 'g'), 'DOT').replace(new RegExp('#', 'g'), 'HASH') + '.png';
    const targetPath = path.join(__dirname, 'static', 'generated', 'images', targetFileName);
    await element.screenshot({ path: targetPath });
    console.log('app-screenshot-generator Created', targetPath);

    return `<div class="screenshot"><div class="title">${title}</div><img src='/images/${targetFileName}' ></div>`;
}

async function process(input, filePath) {
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

        input = input.substring(0, nextIndex) + (await getTemplate(args.url, args.selector, args.title, filePath)) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

process.dispose = async function() {
    return browser.close();
}

module.exports = process;
