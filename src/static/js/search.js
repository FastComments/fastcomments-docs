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

    function fetchAndRenderResults(queryText, queryCounter) {
        searchResultsWrapper.classList.add('open');
        searchResultsWrapper.classList.add('loading');
        makeRequest('http://localhost:5001/search?query=' + queryText, 'GET', null, function cb(responseText) {
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
                        html += '<div class="search-result"><a class="context-title" href="' + entry.url + '">' + entry.title + '</a>';
                        html += '<div class="context-text-and-sections">';
                        html += '<div class="context-text">' + entry.body + '</div>';
                        if (entry.children) {
                            html += '<div class="context-sections"><h2>Relevant Sections</h2>';
                            for (const child of entry.children) {
                                html += '<div class="section">' + child.title + '</div>';
                            }
                            html += '</div>';
                        }
                        html += '</div>';
                        html += '<a class="context-link" href="' + entry.url + '">Go to ' + entry.url + '</a>';
                        html += '</div>';
                        searchResults.innerHTML += html;
                    });
                }
            } catch (e) {
                console.error('Failure to parse index entry', e);
            }
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
