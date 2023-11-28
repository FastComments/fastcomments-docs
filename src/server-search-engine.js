// We only have around 2.5mb of docs. ES ended up being a pain, too many knobs etc to get search right.
// So we just put it all in memory in one node worker.

const MiniSearch = require('minisearch');
const express = require('express');
const {
    getGuides,
    buildGuideItemForMeta,
    getGuideMeta,
    createGuideLink
} = require('./guides');
const fs = require('fs');
const marked = require('marked');
const axios = require('axios');

const AXIOS_CONFIG_NO_THROW = {
    validateStatus: function () {
        return true;
    }
};

(async function () {
    /** @type {Array.<Guide>} **/
    const guides = getGuides();
    const guidesFlat = [];

    for (const guide of guides) {
        const guideTitle = guide.pageHeader || guide.name;

        const meta = getGuideMeta(guide.id);
        if (guide.id.startsWith('installation-') && guide.id !== 'installation') {
            const preview = fs.existsSync(guide.introPath) ? marked(fs.readFileSync(guide.introPath, 'utf8')) : '';
            let bodyWithChildren = preview.trim();
            for (const item of meta.itemsOrdered) {
                const builtItem = await buildGuideItemForMeta(guide, item);
                bodyWithChildren += builtItem.content;
            }

            const subEntry = {
                id: guide.id,
                title: meta.pageHeader,
                icon: '/images/guide-icons/' + meta.icon,
                url: '/' + createGuideLink(guide.id),
                searchText: bodyWithChildren
            };
            guidesFlat.push(subEntry);
        } else if (meta.itemsOrdered) {
            for (const item of meta.itemsOrdered) {
                const builtItem = await buildGuideItemForMeta(guide, item);

                const subEntry = {
                    id: guide.id + '>' + builtItem.id,
                    parentTitle: guideTitle,
                    title: builtItem.title,
                    icon: '/images/guide-icons/' + meta.icon,
                    parentUrl: guide.url,
                    url: builtItem.fullUrl,
                    searchText: builtItem.content
                };
                guidesFlat.push(subEntry);
            }
        } else {
            // what guide wouldn't have this?
        }
    }

    const miniSearch = new MiniSearch({
        fields: ['title', 'parentTitle', 'searchText'], // fields to index for full-text search
        storeFields: ['title', 'parentTitle', 'url', 'parentUrl', 'icon'], // fields to return with search results
        searchOptions: {
            boost: {title: 2},
            fuzzy: 0.1
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
            const rawResults = miniSearch.search(req.query.query, {
                boostDocument: function (id, term) {
                    if (id.startsWith('installation>') && term === 'install') {
                        return 2;
                    }
                    return 1;
                }
            });
            console.log(rawResults.length, 'results for', req.query.query);
            res.send({
                status: 'success',
                results: rawResults.slice(0, 15)
            });
            try {
                const response = await axios.post('https://fastcomments.com/docs-search/track-search-event?API_KEY='
                    + encodeURIComponent(process.env.SEARCH_API_KEY)
                    + '&tenantId='
                    + encodeURIComponent(req.query.tenantId)
                    + '&searchInput=' + encodeURIComponent(req.query.query), AXIOS_CONFIG_NO_THROW
                );
                console.log('Tracker search event response', response);
            } catch (e) {
                console.error(e);
            }
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
