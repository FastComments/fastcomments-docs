// We only have around 2.5mb of docs. ES ended up being a pain, even on a $5/mo instances, too many knobs etc to get search right.
// So we just put it all in memory in one node worker.

const MiniSearch = require('minisearch');
const express = require('express');
const {htmlToText} = require('html-to-text');
const htmlDecode = require('unescape');
const {getGuides, GUIDES_DIR, buildGuideItemForMeta} = require('./guides');
const fs = require('fs');
const path = require('path');
const marked = require('marked');

/** @type {Array.<Guide>} **/
const guides = getGuides();
const guidesFlat = [];
const entriesById = {};

(async function () {
    for (const guide of guides) {
        const guideTitle = guide.pageHeader || guide.name;
        const mainEntry = {
            id: guide.id,
            title: guideTitle,
            url: guide.url,
        };
        if (fs.existsSync(guide.introPath)) {
            mainEntry.body = marked(fs.readFileSync(guide.introPath, 'utf8')).replaceAll("", "");
        }
        entriesById[guide.id] = mainEntry;
        guidesFlat.push(mainEntry);
        const meta = JSON.parse(fs.readFileSync(path.join(GUIDES_DIR, guide.id, 'meta.json'), 'utf8'));
        if (meta.itemsOrdered) {
            for (const item of meta.itemsOrdered) {
                const builtItem = await buildGuideItemForMeta(guide, item);
                const subEntry = {
                    id: guide.id + builtItem.name + item.file,
                    title: `${builtItem.name} (from ${guideTitle} -> ${item.subCat})`,
                    body: builtItem.content,
                    url: builtItem.fullUrl,
                    parent: mainEntry
                };
                guidesFlat.push(subEntry);
            }
        }
    }

    let miniSearch = new MiniSearch({
        fields: ['title', 'body'], // fields to index for full-text search
        storeFields: ['title', 'body', 'url', 'parent'], // fields to return with search results
        searchOptions: {
            boost: {title: 2},
            fuzzy: 0.2
        }
    })

    miniSearch.addAll(guidesFlat);

    console.log('Starting server...');
    const app = express()
    const port = 5001;

    app.get('/search', (req, res) => {
        try {
            res.set('Access-Control-Allow-Origin', '*');
            res.set('Access-Control-Allow-Credentials', 'true');
            res.set('Access-Control-Allow-Headers', 'content-type');
            console.log('Searching for', req.query.query);
            const rawResults = miniSearch.search(req.query.query);
            console.log(rawResults.length, 'results for', req.query.query);
            const groupedByParentId = {};
            for (const result of rawResults) {
                if (result.parent) {
                    const child = {
                        id: result.id,
                        title: result.title,
                        body: result.body,
                        url: result.url,
                    };
                    if (!groupedByParentId[result.parent.id]) {
                        groupedByParentId[result.parent.id] = [child];
                    } else {
                        groupedByParentId[result.parent.id].push(child);
                    }
                } else {
                    groupedByParentId[result.id] = [result];
                }
            }
            const results = [];
            for (const parentId in groupedByParentId) {
                const children = groupedByParentId[parentId];
                const parent = entriesById[parentId];
                if (parent == children[0] && children.length === 1) {
                    results.push(parent);
                } else {
                    results.push({
                        ...parent,
                        children
                    });
                }
            }
            res.send({
                status: 'success',
                results: results.slice(0, 15)
            });
        } catch (e) {
            console.error('Failed to handle search request', req.query.query, e);
            res.status(500).send({
                status: 'failed'
            })
        }
    });

    app.options('/search', function (req, res) {
        res.set('Access-Control-Allow-Origin', '*');
        res.set('Access-Control-Allow-Credentials', 'true');
        res.set('Access-Control-Allow-Headers', 'content-type');
        res.status(200).end();
    });

    app.listen(port, () => {
        console.log(`Search server listening on port ${port}.`)
    });
})();
