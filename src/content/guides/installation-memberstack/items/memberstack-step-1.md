We can easily connect FastComments with Memberstack with a small code snippet:

[inline-code-attrs-start title = 'FastComments Memberstack Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    MemberStack.onReady.then(function (member) {
        FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: "demo",
            urlId: window.location.pathname,
            simpleSSO: member.loggedIn ? {
                username: member.name || member.email.replace(/@.+/, ''),
                email: member.email,
                avatar: member.avatar
            } : null
        });
    });
</script>
[inline-code-end]

When the user visits your site or application while logged in via Memberstack, they will automatically be logged into FastComments and their comments
will be marked `Verified`.

**They will also be able to comment by leaving their name and email, if they are not logged in.**

If you'd like to have **member-only commenting**, try the following code snippet, but change `https://example.com/login` to where you want users to go when they click the `Login` button:

[inline-code-attrs-start title = 'Exclusive FastComments Memberstack Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    MemberStack.onReady.then(function (member) {
        FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: member.loggedIn ? {
                username: member.name || member.email.replace(/@.+/, ''),
                email: member.email,
                avatar: member.avatar
            } : {
                loginURL: 'https://example.com/login'
            }
        });
    });
</script>
[inline-code-end]
