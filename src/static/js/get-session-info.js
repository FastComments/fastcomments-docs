(function () {
    window.getSessionInfo = function getSessionInfo(cb) {
        const xhr = new XMLHttpRequest();
        xhr.withCredentials = true;
        xhr.open('GET',  location.hostname === 'localhost' ? 'http://localhost:3001/auth/session-info' : 'https://fastcomments.com/auth/session-info');
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
