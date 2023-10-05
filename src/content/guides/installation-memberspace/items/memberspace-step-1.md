We can easily connect FastComments with MemberSpace with a small code snippet:

[inline-code-attrs-start title = 'FastComments MemberSpace Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo';
        const ALLOW_ANON = false;
        const LOGIN_URL = 'https://example.com/login';
        const PLAN_DISPLAY_LABELS = {
            'VIP Plan': 'VIP'
        };
        let lastInstance;
    
        function tick() {
            if (!window.MemberSpace) {
                return setTimeout(tick, 100);
            }
            if (!window.FastCommentsUI) {
                return setTimeout(tick, 100);
            }
            const target = document.getElementById('fastcomments-widget');
            if (!target) {
                return setTimeout(tick, 100);
            }
            const data = MemberSpace.getMemberInfo();
            if (data.isLoggedIn && data.memberInfo) {
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: {
                        displayLabel: getDisplayLabel(data.memberInfo),
                        username: data.memberInfo.name,
                        email: data.memberInfo.email,
                        avatar: data.memberInfo.profileImageUrl
                    }
                });
            } else if (lastInstance) {
                lastInstance.destroy();
                lastInstance = FastCommentsUI(target, {
                    tenantId: tenantId,
                    urlId: window.location.pathname,
                    simpleSSO: getAnonSSOConfig()
                });
            }
        }
    
        function getAnonSSOConfig() {
            if (ALLOW_ANON) {
                return undefined;
            }
            return {
                loginURL: LOGIN_URL
            };
        }
        
        function getDisplayLabel(memberInfo) {
            if (!memberInfo.memberships) {
                return;
            }
            for (const membership of memberInfo.memberships) {
                const label = PLAN_DISPLAY_LABELS[membership.name];
                if (label) {
                    return label
                }
            }
        }
    
        tick();
    })();
</script>
[inline-code-end]

When the user visits your site or application while logged in via MemberStack, they will automatically be logged into FastComments and their comments
will be marked `Verified`.

Additionally, in the above example if you have a subscription plan called `VIP Plan` then we'll display a `VIP` badge next to the user's name. You can edit the example to
add more plans. Reach out to support if you have questions.

**They will also be able to comment by leaving their name and email, if they are not logged in.**

### Allow Anonymous Commenting

If you'd like to also have **anonymous commenting**, set ALLOW_ANON to true like this:

    const ALLOW_ANON = true;

Also remember to change `https://example.com/login` to where you want users to go when they click the `Login` button:
