const puppeteer = require('puppeteer');

/*
This would be used like:
[flow-diagram-start]
Title: Secure SSO Flow
Note over Your Frontend: User Loads Page
Your Frontend->Your Backend: Request SSO Payload
Note over Your Backend: Get User ID, username, email, avatar url
Note over Your Backend: Encrypt Via HMAC
Note right of Your Backend: Use API Secret and add timestamp
Your Backend->Your Frontend: SSO Payload
[flow-diagram-end]
 */

const StartToken = '[flow-diagram-start]';
const EndToken = '[flow-diagram-end]';

let authenticated = false;
let browser, page;

async function getTemplate(flowText) {
    if (!authenticated) {
        browser = await puppeteer.launch({headless: false});
        page = await browser.newPage();
        // Next step is to just host the library and run it this way, but ourselves, since there is no server side version.
        await page.goto('<todo>');
        authenticated = true;
    }

    await page.waitForSelector('#example1 textarea');
    await page.evaluate(() => document.querySelector('#example1 textarea').value = '');
    await page.focus('#example1 textarea');
    await page.keyboard.type(flowText);
    await page.waitForSelector('#example1 .diagram svg');
    const title = flowText.split('\n')[1].replace('Title: ', '');
    console.log('searching for', title);
    await page.waitForFunction((title) => {
        const html = document.querySelector('#example1 .diagram svg').innerHTML;

        return html && html.includes(title);
    }, {}, title);

    const svg = await page.evaluate(() => document.querySelector('#example1 .diagram svg').innerHTML);

    return `<div class="flow-diagram">${svg}</div>`;
}

async function process(input, filePath) {
    let nextIndex = input.indexOf(StartToken);
    while (nextIndex > -1) {
        const endTokenIndex = input.indexOf(EndToken);
        if (endTokenIndex === -1) {
            throw new Error('Malformed input! Start token found, but not end.');
        }

        const flowText = input.substring(nextIndex + StartToken.length, endTokenIndex);

        console.log('flow text', flowText);

        input = input.substring(0, nextIndex) + await getTemplate(flowText) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

process.dispose = async function () {
    if (browser) {
        return browser.close();
    }
}

module.exports = process;
