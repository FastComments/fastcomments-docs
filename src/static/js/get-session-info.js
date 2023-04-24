(function () {
    window.getSessionInfo = async function getSessionInfo(cb, failedCB) {
        // first try US/global session store and then check EU.
        const sessionEndpoints = location.hostname === 'localhost' ?
            ['http://localhost:3001/auth/session-info'] :
            [
                'https://fastcomments.com/auth/session-info',
                'https://eu.fastcomments.com/auth/session-info'
            ];

        function doGet(uri) {
            return new Promise((resolve, reject) => {
                const xhr = new XMLHttpRequest();
                xhr.withCredentials = true;
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

        try {
            const urlParams = new URLSearchParams(window.location.search);
            const tenantId = urlParams.get('tId');
            const region = urlParams.get('region');
            if (tenantId) {
                const isProd = location.hostname !== 'localhost';
                const DomainsByRegion = {
                    'us-west': 'fastcomments.com',
                    'eu': 'eu.fastcomments.com'
                };
                const CDNByRegion = {
                    'us-west': 'cdn.fastcomments.com',
                    'eu': 'cdn-eu.fastcomments.com'
                };
                return cb({
                    authenticated: true,
                    tenantId,
                    FC_CDN: region && CDNByRegion[region] ? CDNByRegion[region] : (isProd ? 'https://cdn.fastcomments.com' : 'http://localhost:3001'),
                    FC_SITE: region && DomainsByRegion[region] ? DomainsByRegion[region] : (isProd ? 'https://fastcomments.com' : 'http://localhost:3001')
                });
            }
            // check both at the same time for performance
            const sessionInfo = await Promise.all(sessionEndpoints.map(doGet));
            for (const info of sessionInfo) {
                if (info.tenantId) {
                    return cb(info);
                }
            }
            return cb(sessionInfo[0]);
        } catch (e) {
            console.error(e);
        }
    }
})();
