(function () {
    window.getSessionInfo = function getSessionInfo(cb) {
        const xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://fastcomments.com/auth/session-info');
        xhr.onload = function () {
            if (xhr.status === 200) {
                try {
                    cb(JSON.parse(xhr.responseText));
                } catch (e) {
                    console.error(e);
                }
            }
        };
        xhr.onerror = cb;
        xhr.send();
    }
})();
