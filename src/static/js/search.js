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
        console.log('fetchAndRenderResults', wordIds);
        if (wordIds.length === 0) {
            searchResults.innerHTML = '';
            return;
        }
        if (arrayValuesSame(wordIds, lastSearchedWords)) {
            return;
        }
        lastSearchedWords = wordIds;
        searchResults.innerHTML = '';

=        wordIds.forEach(function (id) {
            getContent('/index-' + id + '.json', function cb(responseText) {
                try {
                    const json = JSON.parse(responseText);
                    searchResults.innerHTML += json.map(function (entry) {
                        return '<div class="search-result">' + entry.aroundText + '</div>';
                    });
                } catch(e) {
                    console.error('Failure to parse index entry', e);
                }
            });
        });
    }

    const input = document.getElementById('search');
    input.addEventListener('input', function () {
        const wordIdsToSearch = [];
        input.value.split(' ').forEach(function (word) {
            const indexEntry = window.docIndex[word];
            if (indexEntry) {
                wordIdsToSearch.push(indexEntry);
            }
        });
        fetchAndRenderResults(wordIdsToSearch);
    });
})();
