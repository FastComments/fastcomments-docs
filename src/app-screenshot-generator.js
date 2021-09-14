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
const MAX_BROWSERS = process.env.MAX_BROWSERS ? parseInt(process.env.MAX_BROWSERS, 10) : 1;

async function addProxySelectToPage(page) {
    const scriptFile = fs.readFileSync(path.resolve(__dirname, 'static', 'js', 'proxy-select.js'), 'utf8');
    const styleFile = fs.readFileSync(path.resolve(__dirname, 'static', 'css', 'proxy-select.css'), 'utf8');
    await page.addScriptTag({content: scriptFile});
    await page.addStyleTag({content: styleFile});
}

const imageCacheFolder = path.join(__dirname, 'static', 'generated', 'image-cache');

function isImageCacheStale(args, fullPath, fileName) {
    if (!fs.existsSync(fullPath)) {
        return true;
    }
    if (!fs.existsSync(imageCacheFolder)) {
        fs.mkdirSync(imageCacheFolder, {
            recursive: true
        });
        return true;
    }
    const imageCacheFilePath = path.join(imageCacheFolder, `${fileName}.json`);
    if (!fs.existsSync(imageCacheFilePath)) {
        return true;
    }

    const imageCacheContent = JSON.parse(fs.readFileSync(imageCacheFilePath, 'utf8'));
    if (!imageCacheContent) {
        return true;
    }

    return imageCacheContent === JSON.stringify(args);
}

function updateImageCache(args, fileName) {
    const imageCacheFilePath = path.join(imageCacheFolder, `${fileName}.json`);
    fs.writeFileSync(imageCacheFilePath, JSON.stringify(args), 'utf8');
}

/**
 *
 * @typedef {Object} BrowserPooled
 * @property {Object} browser
 * @property {Object} page
 * @property {boolean} inUse
 */

const browserPool = [];

/**
 *
 * @callback BrowserPoolCallback
 * @param {Object} BrowserPooled
 */

/**
 *
 * @param {number|null|undefined} width
 * @param {number|null|undefined} height
 * @return {Promise<BrowserPooled>}
 */
async function createBrowser(width, height) {
    const browser = await puppeteer.launch({
        headless: true,
        width: width,
        height: height,
        args: ['--no-sandbox', '--disable-setuid-sandbox', '--enable-font-antialiasing']
    });
    const page = await browser.newPage();
    await page.setViewport({width: width, height: height});
    await page.goto(`${HOST}/auth/login`);
    await page.waitForSelector('form');
    await page.focus('input[name="username"]');
    await page.keyboard.type('demo');
    await page.focus('input[name="email"]');
    await page.keyboard.type('demo@fastcomments.com');
    await page.click('button[type="submit"]');
    await page.waitForSelector('body');

    return {
        browser,
        page,
        inUse: true
    };
}

/**
 * @param {number|null|undefined} width
 * @return {Promise<BrowserPooled>}
 */
async function getOrCreateAvailableBrowser(width) {
    if (width) {
        return createBrowser(width, DEFAULT_HEIGHT);
    }

    const available = browserPool.find((browser) => {
        return !browser.inUse;
    });

    if (available) {
        return available;
    }

    if (browserPool.length < MAX_BROWSERS) {
        const next = createBrowser(DEFAULT_WIDTH, DEFAULT_HEIGHT);

        browserPool.push(next);

        return next;
    }

    return new Promise((resolve) => {
        const interval = setInterval(function () {
            const available = browserPool.find((browser) => {
                return !browser.inUse;
            });

            if (available) {
                clearInterval(interval);
                resolve(available);
            }
        }, 300);
    });
}

/**
 * @async
 * @param {number|null|undefined} width
 * @param {BrowserPoolCallback} callback
 * @return {Promise<*>}
 */
async function withPooledBrowser(width, callback) {
    const instance = await getOrCreateAvailableBrowser(width);

    let result = undefined;

    try {
        instance.inUse = true;
        result = await callback(instance);
        instance.inUse = false;
    } catch (e) {
        instance.inUse = false;
        console.error(e);
        throw e;
    }

    return result;
}

function ensureHost(url) {
    if (!url) {
        return;
    }
    if (url.startsWith('http')) {
        return url;
    }
    return `${HOST}${url}`;
}

async function getTemplate(url, linkUrl, width, actions, clickSelectors, selector, title, addProxySelect, delay, filePath) {
    console.log('app-screenshot-generator Creating', url, selector, width);

    const cacheKey = {url, linkUrl, width, actions, clickSelectors, selector, title, addProxySelect};
    const fileNameHash = crypto.createHash('md5').update(`${url}-${selector}-${title}`).digest('hex');
    const targetFileName = fileNameHash + '.png';
    const targetFolderPath = path.join(__dirname, 'static', 'generated', 'images');
    const targetPath = path.join(targetFolderPath, targetFileName);
    const remotePageUrl = ensureHost(url);

    const imageTemplate = `<div class="screenshot">
        <div class="title">${title}</div>
        ` + (linkUrl !== false ? `<div class="screenshot-link"><a href="${linkUrl ? ensureHost(linkUrl) : remotePageUrl}" target="_blank"><img src="/images/link-external.png" alt="External Link" title="Go to This Page"></a></div>` : '') +
        `<img src='/images/${targetFileName}' class="screenshot-image" >
    </div>`;

    if (!isImageCacheStale(cacheKey, targetPath, targetFileName)) {
        return imageTemplate;
    }

    return await withPooledBrowser(width, async ({browser, page}) => {
        console.log('app-screenshot-generator authenticated...');
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
                        await page.bringToFront();
                        await page.waitForSelector(action.selector);
                        await page.bringToFront();
                        await page.click(action.selector);
                        break;
                    case 'set-value':
                        await page.bringToFront();
                        await page.waitForSelector(action.selector);
                        await page.bringToFront();
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
                await page.bringToFront();
                await page.waitForSelector(clickSelector);
                try {
                    await page.bringToFront();
                    await page.click(clickSelector);
                } catch (e) {
                    console.error('Error for selector', clickSelector, e);
                }
            }
        }

        console.log('Waiting for (to click)', selector);
        await page.bringToFront();
        await page.waitForSelector(selector);
        console.log('app-screenshot-generator found', selector);

        if (delay) {
            await new Promise((resolve) => {
                setTimeout(resolve, delay);
            });
        }

        const element = await page.$(selector);

        if (!fs.existsSync(targetFolderPath)) {
            fs.mkdirSync(targetFolderPath, {
                recursive: true
            });
        }
        await element.screenshot({path: targetPath});
        updateImageCache(cacheKey, targetFileName);
        console.log('app-screenshot-generator Created', targetPath);

        return imageTemplate;
    });
}

async function processInput(input, filePath) {
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

        input = input.substring(0, nextIndex) + (await getTemplate(args.url, args.linkUrl, args.width, args.actions, args.clickSelector ? (args.clickSelector ? [args.clickSelector] : []) : args.clickSelectors, args.selector, args.title, args.addProxySelect, args.delay, filePath)) + input.substring(endTokenIndex + EndToken.length, input.length);
        nextIndex = input.indexOf(StartToken);
    }
    return input;
}

processInput.dispose = async function () {
    browserPool.forEach((instance) => instance.browser && instance.browser.close());
}

module.exports = processInput;
