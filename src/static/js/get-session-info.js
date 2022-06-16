(function () {
    window.getSessionInfo = async function getSessionInfo(cb) {
        const xhr = new XMLHttpRequest();
        xhr.withCredentials = true;
        // first try US/global session store and then check EU.
        const sessionEndpoints = location.hostname === 'localhost' ?
            ['http://localhost:3001/auth/session-info'] :
            [
                'https://fastcomments.com/auth/session-info',
                'https://eu.fastcomments.com/auth/session-info'
            ];

        function doGet(uri) {
            return new Promise((resolve, reject) => {
                xhr.open('GET', uri);
                xhr.onload = function () {
                    if (xhr.status === 200) {
                        try {
                            resolve(JSON.parse(xhr.responseText));
                        } catch (e) {
                            console.error(e);
                        }
                    }
                };
                xhr.onerror = reject;
                xhr.send();
            });
        }

        // check both at the same time for performance
        const sessionInfo = await Promise.all(sessionEndpoints.map(doGet));
        for (const info of sessionInfo) {
            if (info.tenantId) {
                return cb(info);
            }
        }
        cb(sessionInfo[0]);
    }
})();
