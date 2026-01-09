For Step 2 we have to copy our code snippet. Check that line 50 does not say "demo" - you'll want to ensure this has your tenant id. It should be populated for you.

现在我们要复制 ThriveCart-Learn 专用的 FastComments 代码片段。

它相当大，因为与 ThriveCart 的集成有许多功能，所以只需点击代码片段右上角的复制按钮：

[inline-code-attrs-start title = 'ThriveCart Learn+ 评论代码'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
            // 广泛的电子邮件输入字段选择器，以防 ThriveCart 更改 id。
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
                // 使用原始 "img" 查询，以防 ThriveCart 更改图片类选择器。
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // 使用 innerText，以防 ThriveCart 更改配置文件名称的显示方式。
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // increase wait time after 5 attempts incase slow internet.
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // 有时 TC 在同一页面使用多个链接，因此我们去重它们。
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // 去除营销参数和域名
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
                // 去除营销参数和域名
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // 默认为当前页面，这样至少有时能起作用
            }
        }

    })();
</script>
[inline-code-end]

Now paste it into the code block on the left in the ThriveCart editor. It should look like this:

<div class="screenshot white-bg">
    <div class="title">代码已添加</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="代码已添加" />
</div>

That's it! Now we just have to publish:

<div class="screenshot white-bg">
    <div class="title">发布</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="发布" />
</div>

That's it! You should now see the comment box on your course when you preview, and real users will be able to leave comments **without signing in or leaving their username/email a second time**.

### Testing Note!

If you have anonymous commenting disabled, which it is by default, you won't be able to leave comments in `Preview` mode as the `John Smith` user. You will get an authentication
error as the default `John Smith` user has no email. If you want to test, we suggest you use a coupon code and go through your site like an actual user.