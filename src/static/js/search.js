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

    let searchCounter = 0;
    let searchRequest;
    let currentDisplayedSearchValue;

    function setNoResults() {
        searchResults.innerHTML = '<div class="no-results text-center">No results for those keywords.</div>';
    }

    let sessionInfo;

    function getSessionInfoCached(cb) {
        if (sessionInfo) {
            cb(sessionInfo);
        } else {
            window.getSessionInfo(function (newSessionInfo) {
                sessionInfo = newSessionInfo;
                cb(newSessionInfo);
            }, function failed() {
                cb({});
            });
        }
    }

    function fetchAndRenderResults(queryText, queryCounter) {
        searchResultsWrapper.classList.add('open');
        searchResultsWrapper.classList.add('loading');
        getSessionInfoCached(function (sessionInfo) {
            makeRequest((window.location.href.includes('localhost:5000') ? 'http://localhost:5001' : 'https://docs-search.fastcomments.com') + '/search?query=' + queryText + '&tenantId=' + sessionInfo.tenantId, 'GET', null, function cb(responseText) {
                if (searchCounter !== queryCounter) {
                    return;
                }
                currentDisplayedSearchValue = queryText;
                try {
                    searchResultsWrapper.classList.remove('loading');
                    const response = JSON.parse(responseText);
                    if (!response.results || response.results.length === 0) {
                        setNoResults();
                    } else {
                        searchResults.innerHTML = '';
                        response.results.forEach(function (entry) {
                            let html = '';
                            html += '<a class="search-result" href="' + entry.url + '">';
                            if (entry.icon) {
                                html += '<div class="icon-container">'
                                html += '<img src="' + entry.icon + '" alt="Icon" />';
                                html += '</div>';
                            }
                            html += '<div class="details">'
                            if (entry.parentTitle && entry.parentUrl) {
                                html += '<div class="context-title sm">From: ' + entry.parentTitle + '</div>';
                            }
                            html += '<div class="context-title">' + entry.title + '</div>';
                            html += '</div>'
                            html += '</a>';
                            searchResults.innerHTML += html;
                        });
                    }
                } catch (e) {
                    console.error('Failure to parse index entry', e);
                }
            });
        });
    }

    // simple debounce mechanism
    setInterval(function () {
        if (searchRequest) {
            searchCounter++
            fetchAndRenderResults(searchRequest, searchCounter);
            searchRequest = undefined;
        }
    }, 1000);

    const input = document.getElementById('search');

    input.addEventListener('input', function () {
        if (!input.value) {
            searchRequest = undefined;
            currentDisplayedSearchValue = undefined;
            searchCounter++;
            setNoResults();
        } else if (input.value.length > 2) {
            if (currentDisplayedSearchValue && input.value.trim() === currentDisplayedSearchValue.trim()) {
                return;
            }
            searchRequest = input.value;
        }
    });

    input.addEventListener('submit', function () {
        searchRequest = input.value;
    });
})();
