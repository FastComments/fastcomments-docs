// We only have around 2.5mb of docs. ES ended up being a pain, too many knobs etc to get search right.
// So we just put it all in memory in one node worker.

const MiniSearch = require('minisearch');
const express = require('express');
const {getGuides, buildGuideItemForMeta, createGuideItemIdFromPath, getGuideMeta} = require('./guides');
const fs = require('fs');
const marked = require('marked');
const axios = require('axios');

(async function () {
    /** @type {Array.<Guide>} **/
    const guides = getGuides();
    const guidesFlat = [];

    for (const guide of guides) {
        const guideTitle = guide.pageHeader || guide.name;

        const meta = getGuideMeta(guide.id);
        let preview = fs.existsSync(guide.introPath) ? marked(fs.readFileSync(guide.introPath, 'utf8')) : '';
        let bodyWithChildren = preview.trim();
        if (meta.itemsOrdered) {
            for (const item of meta.itemsOrdered) {
                const builtItem = await buildGuideItemForMeta(guide, item);
                if (!preview.length) { // important we do after bodyWithChildren = so we don't duplicate content that could impact ranking.
                    // use first section as preview.
                    preview = builtItem.content;
                }
                bodyWithChildren += builtItem.content + '\n';
            }
        }
        if (!preview.length) {
            // probably a placeholder guide - does it refer to an other guide?
            if (guide.url && guide.url.startsWith('/guide-')) {
                let [path, subId] = guide.url.split('#');
                if (subId) {
                    path = path.replace('.html', '').replace('/guide-', '');
                    const linkedGuide = guides.find((guide) => guide.id === path);
                    if (linkedGuide) {
                        const linkedGuideMeta = getGuideMeta(linkedGuide.id);
                        if (linkedGuideMeta.itemsOrdered) {
                            const subItem = linkedGuideMeta.itemsOrdered.find((item) => {
                                return createGuideItemIdFromPath(item.file) === subId;
                            });
                            if (subItem) {
                                const builtItem = await buildGuideItemForMeta(linkedGuide, subItem);
                                preview = builtItem.content;
                            }
                        }
                    }
                } else {
                    console.warn('No sub entry to link to to create preview!', guide);
                }
            }
        }

        const previewSplit = preview.split('\n');

        if (previewSplit.length > 10) {
            preview = previewSplit.slice(0, 10).join('\n');
        }

        const mainEntry = {
            id: guide.id,
            title: guideTitle,
            url: guide.url,
            preview,
            bodyWithChildren
        };
        guidesFlat.push(mainEntry);
    }

    const miniSearch = new MiniSearch({
        fields: ['title', 'titleWithMeta', 'preview', 'bodyWithChildren'], // fields to index for full-text search
        storeFields: ['title', 'preview', 'url', 'parent'], // fields to return with search results
        searchOptions: {
            boost: {title: 2},
            fuzzy: 0.2
        }
    })

    miniSearch.addAll(guidesFlat);

    console.log('Starting server...');
    const app = express()
    const port = 5001;

    app.get('/search', async (req, res) => {
        try {
            res.set('Access-Control-Allow-Origin', '*');
            res.set('Access-Control-Allow-Credentials', 'true');
            res.set('Access-Control-Allow-Headers', 'content-type');
            console.log('Searching for', req.query.query);
            const rawResults = miniSearch.search(req.query.query);
            console.log(rawResults.length, 'results for', req.query.query);
            res.send({
                status: 'success',
                results: rawResults.slice(0, 15)
            });
            await axios.post('https://fastcomments.com/docs-search/track-search-event?API_KEY='
                + encodeURIComponent(process.env.SEARCH_API_KEY)
                + '&tenantId='
                + encodeURIComponent(req.query.tenantId)
                + '&searchInput=' + encodeURIComponent(req.query.query)
            );
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
