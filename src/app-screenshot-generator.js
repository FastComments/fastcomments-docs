const fs = require('fs');
const path = require('path');
const vm = require('vm');
const puppeteer = require('puppeteer');
const crypto = require('crypto');

const StartToken = '[app-screenshot-start';
const EndToken = 'app-screenshot-end]';

const HOST = 'https://fastcomments.com';

const DEFAULT_WIDTH = 1920;
const DEFAULT_HEIGHT = 1080;

const addProxySelectToPage = async (page) => {
    const scriptFile = fs.readFileSync(path.resolve(__dirname, 'static', 'js', 'proxy-select.js'), 'utf8');
    const styleFile = fs.readFileSync(path.resolve(__dirname, 'static', 'css', 'proxy-select.css'), 'utf8');
    await page.addScriptTag({content: scriptFile});
    await page.addStyleTag({content: styleFile});
}

/**
 *
 * @typedef {Object} BrowserPooled
 * @property {Object} browser
 * @property {Object} page
 */

const browserPool = [];

/**
 *
 * @callback BrowserPoolCallback
 * @param {Object} BrowserPooled
 */

/**
 * @async
 * @param {BrowserPoolCallback} callback
 * @return {Promise<*>}
 */
async function withPooledBrowser(callback) {
    let instance = browserPool.pop();

    if (!instance) {
        const browser = await puppeteer.launch({
            headless: true,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            args: ['--no-sandbox', '--disable-setuid-sandbox']
        });
        const page = await browser.newPage();
        await page.setViewport({width: DEFAULT_WIDTH, height: DEFAULT_HEIGHT});
        await page.goto(`${HOST}/auth/login`);
        await page.waitForSelector('form');
        await page.focus('input[name="username"]');
        await page.keyboard.type('demo');
        await page.focus('input[name="email"]');
        await page.keyboard.type('demo@fastcomments.com');
        await page.click('button[type="submit"]');
        await page.waitForSelector('body');

        instance = {
            browser,
            page
        };
    }

    let result = undefined;

    try {
        result = await callback(instance);
    } catch (e) {
        console.error(e);
    }
    browserPool.push(instance);

    return result;
}

async function getTemplate(url, actions, clickSelectors, selector, title, addProxySelect, delay, filePath) {
    console.log('app-screenshot-generator Creating', url, selector);

    return await withPooledBrowser(async ({browser, page}) => {
        console.log('app-screenshot-generator authenticated...');
        const remotePageUrl = `${HOST}${url}`;
        await page.goto(remotePageUrl);
        console.log('app-screenshot-generator loaded', url);

        if (addProxySelect) {
            await addProxySelectToPage(page);
            // await page.waitForNavigation({waitUntil: 'networkidle2'});
        }

        if (actions) {
            for (const action of actions) {
                console.log('next action', action);
                switch (action.type) {
                    case 'click':
                        await page.waitForSelector(action.selector);
                        await page.click(action.selector);
                        break;
                    case 'set-value':
                        await page.waitForSelector(action.selector);
                        await page.evaluate((selector, value) => {
                            document.querySelector(selector).value = value;
                        }, action.selector, action.value);
                        break;
                    default:
                        throw new Error(`Unsupported action type! ${action}`);
                }
            }
        }

        if (clickSelectors) {
            for (const clickSelector of clickSelectors) {
                console.log('Waiting for', clickSelector);
                await page.waitForSelector(clickSelector);
                try {
                    await page.click(clickSelector);
                } catch (e) {
                    console.error('Error for selector', clickSelector, e);
                }
            }
        }

        await page.waitForSelector(selector);
        console.log('app-screenshot-generator found', selector);

        if (delay) {
            await new Promise((resolve) => {
                setTimeout(resolve, delay);
            });
        }

        const element = await page.$(selector);

        const fileNameHash = crypto.createHash('md5').update(`${url}-${selector}-${title}`).digest('hex');
        const targetFileName = fileNameHash + '.png';
        const targetFolderPath = path.join(__dirname, 'static', 'generated', 'images');
        if (!fs.existsSync(targetFolderPath)) {
            fs.mkdirSync(targetFolderPath, {
                recursive: true
            });
        }
        const targetPath = path.join(targetFolderPath, targetFileName);
        await element.screenshot({path: targetPath});
        console.log('app-screenshot-generator Created', targetPath);

        return `<div class="screenshot">
        <div class="title">${title}</div>
        <div class="screenshot-link"><a href="${remotePageUrl}" target="_blank"><img src="/images/link-external.png" alt="External Link" title="Go to This Page"></a></div>
        <img src='/images/${targetFileName}' class="screenshot-image" >
    </div>`;
    });
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

        input = input.substring(0, nextIndex) + (await getTemplate(args.url, args.actions, args.clickSelector ? (args.clickSelector ? [args.clickSelector] : []) : args.clickSelectors, args.selector, args.title, args.addProxySelect, args.delay, filePath)) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

process.dispose = async function () {
    browserPool.forEach((instance) => instance.browser.close());
}

module.exports = process;
