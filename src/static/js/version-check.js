(function () {
    if (!window.buildId) {
        return console.error('Cannot watch for new version due to missing buildId on page.');
    }

    function getContent(url, cb, cbFailure) {
        const xhr = new XMLHttpRequest();
        xhr.open('GET', url);
        xhr.onload = function () {
            if (xhr.status === 200) {
                cb(xhr.responseText);
            } else {
                cbFailure();
            }
        };
        xhr.send();
    }

    function check() {
        getContent('/build-id', function success(responseText) {
            if (window.buildId !== responseText) {
                const alertContainer = document.createElement('div');
                alertContainer.classList.add('new-version-alert');
                alertContainer.innerHTML = 'A new version of these docs are available! <a href="javascript:window.location.reload()">Refresh</a> <a href="javascript:document.querySelector(\'.new-version-alert\').remove()">Ignore This Update</a>';
                document.body.appendChild(alertContainer);
            } else {
                setTimeout(check, 5000);
            }
        }, function failure() {
            setTimeout(check, 5000);
        });
    }

    // No need to check as soon as they load the page.
    setTimeout(check, 5000);
})();
