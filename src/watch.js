// watch for changes to a particular guide, rebuild that file, and update the whole guide

const path = require('path');
const fs = require('fs');
const filewatcher = require('filewatcher');
const {buildGuide, getGuides} = require('./guides');

const GUIDES_DIR_NAME = 'guides';

const GUIDES_DIR = path.join(__dirname, 'content', GUIDES_DIR_NAME);

const watcher = filewatcher();

fs.readdirSync(GUIDES_DIR).forEach((guide) => {
    watcher.add(path.join(GUIDES_DIR, guide, 'items'));
});

watcher.on('change', async function (file, stat) {
    console.log('File modified: %s', file);
    if (!stat) {
        console.log('deleted', file);
    }
    const guide = getGuides().find((guide) => {
        return guide.itemsPath === file;
    });
    if (!guide) {
        return console.error('Guide not found for path', file);
    }
    console.log('Rebuilding...', guide);
    await buildGuide(guide, {});
});

console.log('Watching...');
