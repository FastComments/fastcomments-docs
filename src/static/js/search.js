(function search() {
    if (!window.docIndex) {
        return console.error('Root of search index not loaded!');
    }

    const searchResults = document.getElementById('search-results');

    function getContent(url, cb) {
        const xhr = new XMLHttpRequest();
        xhr.open('GET', url);
        xhr.onload = function () {
            if (xhr.status === 200) {
                cb(xhr.responseText);
            }
        };
        xhr.send();
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

    function fetchAndRenderResults(wordIds) {
        if (wordIds.length === 0) {
            searchResults.innerHTML = '';
            return;
        }
        if (arrayValuesSame(wordIds, lastSearchedWords)) {
            return;
        }
        lastSearchedWords = wordIds;
        searchResults.innerHTML = '';

        let resultingPages = [];
        wordIds.forEach(function (id) {
            getContent('/index-' + id + '.json', function cb(responseText) {
                try {
                    const json = JSON.parse(responseText);
                    json.forEach(function (entry) {
                        if (!resultingPages.includes(entry.url)) {
                            searchResults.innerHTML += '<a class="search-result" href="' + entry.url + '"><div class="context-title">' + entry.title + '</div><div class="context-text">' + entry.aroundText + '</div><div class="context-link">Go to ' + entry.url + '</div></a>';
                            resultingPages.push(entry.url);
                        }
                    });
                } catch (e) {
                    console.error('Failure to parse index entry', e);
                }
            });
        });
    }

    const input = document.getElementById('search');
    input.addEventListener('input', function () {
        const wordIdsToSearch = [];
        input.value.toLowerCase().split(' ').forEach(function (word) {
            const indexEntry = window.docIndex[word];
            if (indexEntry) {
                wordIdsToSearch.push(indexEntry);
            }
        });
        fetchAndRenderResults(wordIdsToSearch);
    });
})();
