For Step 2 we have to copy our code snippet. Check that line 50 does not say "demo" - you'll want to ensure this has your tenant id. It should be populated for you.

Now let's copy our ThriveCart-Learn-Specific FastComments code snippet.

It's quite large, because the integration with ThriveCart is involved, so just click the Copy button in the top right of the code snippet:

[inline-code-attrs-start title = 'ThriveCart Learn+ Comments Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let attemptsRemaining = 10;

        function tryLoad() {
            const simpleSSO = {};
            let isAuthenticated = false;
            let profileLink = document.querySelector('.thrivecart-courses-header-profile-link');
            if (!profileLink) {
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // class is different for preview.
            }
            // broad email input field selector incase ThriveCart changes id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // allow preview to work - no email available.
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts incase slow internet.
            }
            if (profileLink) {
                // use raw "img" query incase ThriveCart changes image class selector.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // use innerText incase ThriveCart changes how profile name is displayed.
                if (profileLink.innerText) {
                    isAuthenticated = true;
                    simpleSSO.username = profileLink.innerText;
                }
            } else {
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (user name/avatar). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user profile info found - waiting and trying again.');
                attemptsRemaining--;
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts incase slow internet.
            }

            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: 'demo',
                urlId: window.location.pathname, // trims marketing parameters and domain name
                simpleSSO: isAuthenticated ? simpleSSO : null
            });
        }

        tryLoad();

    })();
</script>
[inline-code-end]

Now paste it into the code block on the left in the ThriveCart editor. It should look like this:

<div class="screenshot white-bg">
    <div class="title">Code Added</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="Code Added" />
</div>

That's it! Now we just have to publish:

<div class="screenshot white-bg">
    <div class="title">Publish</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="Publish" />
</div>

That's it! You should now see the comment box on your course when you preview, and real users will be able to leave comments **without signing in or leaving their username/email a second time**.

### Testing Note!

If you have anonymous commenting disabled, which it is by default, you won't be able to leave comments in `Preview` mode as the `John Smith` user. You will get an authentication
error as the default `John Smith` user has no email. If you want to test, we suggest you use a coupon code and go through your site like an actual user.