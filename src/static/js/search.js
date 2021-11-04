(function search() {
    const searchResultsWrapper = document.getElementById('search-results-wrapper');
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

    /**
     * @typedef {Object} SearchRequest
     * @property {boolean} cancelled
     */

    /** @type {SearchRequest|null} **/
    let lastSearchRequest = null;

    function setNoResults() {
        searchResults.innerHTML = '<div class="no-results text-center">No results for those keywords.</div>';
    }

    function fetchAndRenderResults(queryText) {
        if (lastSearchRequest) {
            lastSearchRequest.cancelled = true;
        }

        lastSearchRequest = {
            cancelled: false
        }
        const searchFunction = function startCancellableSearch(searchRequest) {
            searchResultsWrapper.classList.add('open');
            searchResultsWrapper.classList.add('loading');
            makeRequest('https://fastcomments.com/docs-search-index/search?query=' + queryText, 'GET', null, function cb(responseText) {
                if (searchRequest.cancelled) {
                    return;
                }
                try {
                    searchResultsWrapper.classList.remove('loading');
                    const response = JSON.parse(responseText);
                    if (!response.results || response.results.length === 0) {
                        setNoResults();
                    } else {
                        searchResults.innerHTML = '';
                        response.results.forEach(function (entry) {
                            searchResults.innerHTML += '<a class="search-result" href="' + entry.url + '"><div class="context-title">' + entry.title + '</div><div class="context-text">' + (entry.highlightedContent ? entry.highlightedContent : entry.content) + '</div><div class="context-link">Go to ' + entry.url + '</div></a>';
                        });
                    }
                } catch (e) {
                    console.error('Failure to parse index entry', e);
                }
            });
        };
        searchFunction(lastSearchRequest);
    }

    const input = document.getElementById('search');

    input.addEventListener('input', function () {
        if (!input.value) {
            setNoResults();
        } else if (input.value.length > 2) {
            fetchAndRenderResults(input.value);
        }
    });
})();
