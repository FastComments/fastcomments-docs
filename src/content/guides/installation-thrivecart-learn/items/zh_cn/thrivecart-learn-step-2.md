对于第 2 步，我们需要复制代码片段。请检查第 50 行是否显示为 "demo"——你需要确保这里填入的是你的租户 ID。它应该已经自动为你填好了。

现在让我们复制 ThriveCart-Learn 专用的 FastComments 代码片段。

这个片段相当长，因为与 ThriveCart 的集成包含很多功能，所以只需点击代码片段右上角的“复制”按钮：

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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // 预览时的 class 不同。
            }
            // 宽泛的邮箱输入框选择器，以防 ThriveCart 更改 id。
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // 允许预览正常工作 - 没有可用的邮箱。
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // 5 次尝试后增加等待时间，以防网络较慢。
            }
            if (profileLink) {
                // 使用原始的 "img" 查询，以防 ThriveCart 更改图片 class 选择器。
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // 使用 innerText，以防 ThriveCart 更改个人资料名称的显示方式。
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // 5 次尝试后增加等待时间，以防网络较慢。
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // 有时 TC 会为同一页面使用多个链接，所以让我们去重。
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
                return window.location.pathname; // 默认使用当前路径，这样至少有时能正常工作
            }
        }

    })();
</script>
[inline-code-end]

现在将其粘贴到 ThriveCart 编辑器左侧的代码块中。它应该看起来像这样：

<div class="screenshot white-bg">
    <div class="title">代码已添加</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="代码已添加" />
</div>

就是这样！现在我们只需要发布：

<div class="screenshot white-bg">
    <div class="title">发布</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="发布" />
</div>

完成了！现在你在预览时应该能在课程中看到评论框，真实用户将能够**无需登录或再次输入用户名/邮箱**即可发表评论。

### 测试注意事项！

如果你已禁用匿名评论（默认情况下是禁用的），你将无法在 `Preview` 模式下以 `John Smith` 用户身份发表评论。你会收到一个身份验证错误，因为默认的 `John Smith` 用户没有邮箱。如果你想进行测试，我们建议你使用优惠券代码，像真实用户一样浏览你的网站。