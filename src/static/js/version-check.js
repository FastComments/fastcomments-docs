(function () {

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
            if (responseText && window.buildId !== responseText) {
                const alertContainer = document.createElement('div');
                alertContainer.classList.add('new-version-alert');
                alertContainer.innerHTML = 'A new version of these docs are available! <a href="#" class="refresh">Refresh</a> <a href="#" class="ignore">Ignore This Update</a>';
                alertContainer.addEventListener('click', function (e) {
                    if (e.target.classList.contains('refresh')) {
                        e.preventDefault();

                        const url = new URL(window.location.href);
                        url.searchParams.set('b', responseText);

                        window.buildId = responseText;
                        window.location.href = url;
                    } else if (e.target.classList.contains('ignore')) {
                        e.preventDefault();
                        alertContainer.remove();
                        window.buildId = responseText;
                        setTimeout(check, 5000);
                    }
                });
                document.body.appendChild(alertContainer);
            } else {
                setTimeout(check, 5000);
            }
        }, function failure() {
            setTimeout(check, 5000);
        });
    }

    getContent('/build-id', function success(responseText) {
        window.buildId = responseText;
        setTimeout(check, 5000);
    });
})();
