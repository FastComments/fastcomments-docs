Now we can copy the following code snippet. Use the copy button that appears in the top right of the snippet.

There are a few things you can configure in the code, see lines 4 through 7.

[inline-code-attrs-start title = 'Squarespace All Pages Comments Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        const tenantId = 'demo'; // your account id
        const OPT_IN_NOTIFICATIONS = true; // should the user get email notifications for replies etc?
        const USE_SSO = true; // should we auto log the current user into the comments section?
        const LOGIN_REQUIRED = false; // a SquareSpace user session is required to comment

        function loadCommentsForUser(simpleSSO) {
            function tryLoad() {
                // try to load for different layouts
                let targetDiv = document.querySelector('.blog-item-comments-content');
                if (!targetDiv) {
                    const pageContainer = document.querySelector('#page.container');
                    if (!pageContainer) {
                        console.warn('FastComments Error - Page container not found, trying again.');
                        setTimeout(tryLoad, 500);
                        return;
                    }
                    const pageSection = document.createElement('section');
                    pageSection.classList.add('page-section', 'content-width-wide');
                    const contentWrapper = document.createElement('div');
                    contentWrapper.classList.add('content-wrapper');
                    const content = document.createElement('div');
                    content.classList.add('content');
                    const targetDiv = document.createElement('div');
                    pageContainer.appendChild(pageSection);
                    pageSection.appendChild(contentWrapper);
                    contentWrapper.appendChild(content);
                    content.appendChild(targetDiv);
                }
                window.FastCommentsUI(targetDiv, {
                    tenantId,
                    simpleSSO
                });
            }

            tryLoad();
        }
        
        if (!USE_SSO) {
            return loadCommentsForUser(LOGIN_REQUIRED ? null : undefined);
        }

        const cookieObject = document.cookie.split(';')
            .map(kv => kv.split('='))
            .map(kv => [kv[0].trim(), decodeURIComponent(kv[1])])
            .reduce((o, kv) => {
                o[kv[0]] = kv[1];
                return o;
            }, {});

        const rawSiteUserInfo = cookieObject["SiteUserInfo"];
        if (!rawSiteUserInfo) {
            return loadCommentsForUser(LOGIN_REQUIRED ? null : undefined);
        }
        const userSiteInfo = JSON.parse(rawSiteUserInfo);
        const userFirstName = userSiteInfo["firstName"];
        const userId = userSiteInfo["siteUserId"];
        const xsrf1 = cookieObject["crumb"];
        const xsrf2 = cookieObject["siteUserCrumb"];
        const profileURL = "/api/site-users/account/profile";
        const headers = {"x-csrf-token": xsrf1, "x-siteuser-xsrf-token": xsrf2};
        fetch(profileURL, {headers})
            .then(r => r.json())
            .then(j => {
                loadCommentsForUser({
                    id: userId,
                    username: `${userFirstName}.${j.name.lastName}`,
                    displayName: userFirstName,
                    email: j.email,
                    optedInNotifications: OPT_IN_NOTIFICATIONS,
                    optedInSubscriptionNotifications: OPT_IN_NOTIFICATIONS
                });
            });
    })();
</script>

[inline-code-end]

...then paste in the code area and click save. It should look like this, with the code in the `FOOTER` block:

<div class="screenshot white-bg">
    <div class="title">Paste and Save</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Paste and Save" />
</div>

If you're having issues, make sure near the bottom it doesn't say `"tenantId": "demo"`. It should show your tenant id if you are logged in. If not, reach out to support.
