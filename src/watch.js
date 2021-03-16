// watch for changes to a particular guide, rebuild that file, and update the whole guide

const path = require('path');
const fs = require('fs');
const filewatcher = require('filewatcher');
const {
    TEMPLATE_DIR,
    GUIDE_LAYOUT_PATH,
    buildGuide,
    getGuides
} = require('./guides');

const GUIDES_DIR_NAME = 'guides';

const GUIDES_DIR = path.join(__dirname, 'content', GUIDES_DIR_NAME);

const watcher = filewatcher();

const GUIDE_ROOT_FILES = [TEMPLATE_DIR, GUIDE_LAYOUT_PATH];
GUIDE_ROOT_FILES.forEach((file) => watcher.add(file));

const STATIC_ROOT_DIRS = [
    path.join(__dirname, 'static', 'css'),
    path.join(__dirname, 'static', 'images'),
    path.join(__dirname, 'static', 'js'),
];
STATIC_ROOT_DIRS.forEach((dir) => {
    fs.readdirSync(dir).forEach((dirEntry) => {
        watcher.add(path.join(dir, dirEntry));
    });
});

const STATIC_ROOT_DIRS_TO_GENERATED = {
    [path.join(__dirname, 'static', 'css')]: path.join(__dirname, 'static', 'generated', 'css'),
    [path.join(__dirname, 'static', 'images')]: path.join(__dirname, 'static', 'generated', 'images'),
    [path.join(__dirname, 'static', 'js')]: path.join(__dirname, 'static', 'generated', 'js'),
};

fs.readdirSync(GUIDES_DIR).forEach((guide) => {
    watcher.add(path.join(GUIDES_DIR, guide, 'meta.json'));
    watcher.add(path.join(GUIDES_DIR, guide, 'index.md.html'));
    watcher.add(path.join(GUIDES_DIR, guide, 'items'));
});

async function rebuildGuide(guide) {
    console.log('Rebuilding...', guide);
    await buildGuide(guide, {});
}

let jobsInQueue = [];
let processing = false;

async function processNext() {
    if (!processing && jobsInQueue.length > 0) {
        console.log('Building next job...');
        const next = jobsInQueue.pop();
        processing = true;
        await next();
        processing = false;
        console.log('Watch rebuild done...', jobsInQueue.length, 'more to do...');
    }
}

watcher.on('change', async function (file, stat) {
    console.log('File modified: %s', file);
    if (!stat) {
        console.log('deleted', file);
    }
    jobsInQueue.push(async function () {
        const staticRootStart = STATIC_ROOT_DIRS.find((dir) => file.startsWith(dir));
        if (staticRootStart) {
            const targetFile = path.join(STATIC_ROOT_DIRS_TO_GENERATED[staticRootStart], path.basename(file));
            console.log(`Copying... from=[${file}] to=[${targetFile}]`);
            fs.copyFileSync(file, targetFile);
            console.log(`Copy done! from=[${file}] to=[${targetFile}]`);
        } else if (GUIDE_ROOT_FILES.includes(file)) {
            for (const guide of getGuides()) {
                await rebuildGuide(guide);
            }
        } else {
            const guide = getGuides().find((guide) => {
                return guide.itemsPath === file ||
                    guide.metaJSONPath === file ||
                    guide.indexTemplatePath === file ||
                    guide.conclusionPath === file ||
                    guide.introPath === file;
            });
            if (!guide) {
                return console.error('Guide not found for path', file);
            }
            await rebuildGuide(guide);
        }
    });
    console.log('Watch rebuild enqueued...');
    await processNext();
});

console.log('Watching...');
