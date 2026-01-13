For Step 2 we have to copy our code snippet. Check that line 50 does not say "demo" - you'll want to ensure this has your tenant ID. It should be populated for you.

Now let's copy our ThriveCart-Learn-Specific FastComments code snippet.

It's quite large, because the integration with ThriveCart has a lot of features, so just click the Copy button in the top right of the code snippet:

[inline-code-attrs-start title = 'ThriveCart Learn+ Comments Code'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let attemptsRemaining = 10;

        function tryLoad() {
            const simpleSSO = {optedInNotifications: true, optedInSubscriptionNotifications: true};
            let isAuthenticated = false;
            let profileLink = document.querySelector('.thrivecart-courses-header-profile-link');
            if (!profileLink) {
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // class is different for preview.
            }
            // broad email input field selector in case ThriveCart changes ID.
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts in case of slow internet.
            }
            if (profileLink) {
                // use raw "img" query in case ThriveCart changes image class selector.
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // use innerText in case ThriveCart changes how the profile name is displayed.
                if (profileLink.innerText) {
                    isAuthenticated = true;
                    simpleSSO.username = profileLink.innerText;
                } else {
                    const bold = profileLink.querySelector('b');
                    if (bold && bold.innerText) {
                        isAuthenticated = true;
                        simpleSSO.username = bold.innerText;
                    }
                }
            } else {
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (user name/avatar). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user profile info found - waiting and trying again.');
                attemptsRemaining--;
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts in case of slow internet.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // sometimes TC uses multiple links to the same page, so let's de-duplicate them.
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // trim marketing parameters and domain name
                url = window.location.pathname;
            }

            if (url) {
                url = url.replace('/starte-hier', '');
                url = url.replace('/start-here', '');
            }

            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: 'demo',
                urlId: url,
                simpleSSO: isAuthenticated ? simpleSSO : null
            });
        }

        tryLoad();

        function getPathnameFromUrl(url) {
            try {
                const parsedUrl = new URL(url);
                // trim marketing parameters and domain name
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // default to current, so at least it works sometimes
            }
        }

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

That's it! You should now see the comment box on your course when you preview, and real users will be able to leave comments **without signing in again or re-entering their username or email**.

### Testing Note!

If you have anonymous commenting disabled, which it is by default, you won't be able to leave comments in `Preview` mode as the `John Smith` user. You will get an authentication
error as the default `John Smith` user has no email. If you want to test, we suggest you use a coupon code and go through your site like an actual user.

---