(function search() {
    if (!window.docIndex) {
        return console.error('Root of search index not loaded!');
    }

    const searchResults = document.getElementById('search-results');

    function makeRequest(url, method, body, cb) {
        const xhr = new XMLHttpRequest();
        xhr.open(method, url);
        xhr.onload = function () {
            if (xhr.status === 200) {
                cb && cb(xhr.responseText);
            }
        };
        if (method === 'POST') {
            xhr.setRequestHeader('Content-type', 'application/json');
        }
        if (body) {
            xhr.send(JSON.stringify(body));
        } else {
            xhr.send();
        }
    }

    function arrayValuesSame(a, b) {
        if (a.length !== b.length) {
            return false;
        }
        for (const index in a) {
            if (b[index] !== a[index]) {
                return false;
            }
        }
        return true;
    }

    let lastSearchedWords = [];

    /**
     * @typedef {Object} SearchRequest
     * @property {boolean} cancelled
     */

    /** @type {SearchRequest|null} **/
    let lastSearchRequest = null;

    function fetchAndRenderResults(wordIds) {
        if (wordIds.length === 0) {
            searchResults.innerHTML = '';
            lastSearchedWords = [];
            return;
        }
        if (arrayValuesSame(wordIds, lastSearchedWords)) {
            return;
        }
        lastSearchedWords = wordIds;
        searchResults.innerHTML = '';

        if (lastSearchRequest) {
            lastSearchRequest.cancelled = true;
        }

        lastSearchRequest = {
            cancelled: false
        }
        const searchFunction = function startCancellableSearch(searchRequest) {
            let resultingPages = [];
            const responses = [];
            wordIds.forEach(function (id) {
                makeRequest('/index-' + id + '.json', 'GET', null, function cb(responseText) {
                    if (searchRequest.cancelled) {
                        return;
                    }
                    try {
                        responses.push(JSON.parse(responseText));
                    } catch (e) {
                        console.error('Failure to parse index entry', e);
                    }
                    if (responses.length === wordIds.length) {
                        for (const json of responses) {
                            try {
                                json.sort(function (a, b) {
                                    if (a.count === b.count) {
                                        return 0;
                                    }
                                    return a.count < b.count ? 1 : -1;
                                });
                                json.forEach(function (entry) {
                                    console.log('entry', entry.url, entry.count);
                                    if (!resultingPages.includes(entry.url)) {
                                        searchResults.innerHTML += '<a class="search-result" href="' + entry.url + '"><div class="context-title">' + entry.title + '</div><div class="context-text">' + entry.aroundText + '</div><div class="context-link">Go to ' + entry.url + '</div></a>';
                                        resultingPages.push(entry.url);
                                    }
                                });
                            } catch (e) {
                                console.error('Failure to render index entry', e);
                            }
                        }
                    }
                });
            });
        };
        searchFunction(lastSearchRequest);
    }

    const input = document.getElementById('search');

    setInterval(function () {
        if (input.value && input.value.length > 2) {
            const valueTrimmed = input.value.trim().toLowerCase();

            if (!queriesTracked.includes(valueTrimmed)) {
                // We track searches made, so that we know if we're missing some documentation. This is so we don't have to use a third party which could introduce tracking behavior.
                makeRequest(location.hostname === 'localhost' ? 'http://localhost:3001/docs-search-event' : 'https://fastcomments.com/docs-search-event', 'POST', {
                    input: valueTrimmed
                });
                queriesTracked.push(valueTrimmed);
            }
        }
    }, 1500);

    let queriesTracked = [];
    input.addEventListener('input', function () {
        const wordIdsToSearch = [];

        input.value.toLowerCase().split(' ').forEach(function (word) {
            const indexEntry = window.docIndex[word];
            if (indexEntry) {
                wordIdsToSearch.push(indexEntry);
            }
        });
        if (input.value && input.value.length > 5 && wordIdsToSearch.length === 0) {
            searchResults.innerHTML = '<div class="no-results text-center">No results for those keywords.</div>';
            lastSearchedWords = [];
        } else {
            fetchAndRenderResults(wordIdsToSearch);
        }
    });
})();
