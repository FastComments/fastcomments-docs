For Step 2 we have to copy our code snippet. Check that line 50 does not say "demo" - you'll want to ensure this has your tenant id. It should be populated for you.

現在讓我們複製專為 ThriveCart-Learn 設計的 FastComments 程式碼片段。

它相當龐大，因為與 ThriveCart 的整合有許多功能，所以只要點擊程式碼片段右上角的 Copy 按鈕：

[inline-code-attrs-start title = 'ThriveCart Learn+ 評論程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // 預覽模式時 class 不同。
            }
            // 廣泛的 email 輸入欄選擇器，以防 ThriveCart 更改 id.
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // 允許預覽運作 - 沒有可用的電子郵件。
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // 在嘗試 5 次後增加等待時間，以防網速較慢。
            }
            if (profileLink) {
                // 使用原始 "img" 查詢，以防 ThriveCart 更改圖片 class 選擇器。
                const avatarImg = profileLink.querySelector('img');
                if (avatarImg && avatarImg.src) {
                    isAuthenticated = true;
                    simpleSSO.avatar = avatarImg.src;
                }
                // 使用 innerText，以防 ThriveCart 更改個人資料名稱的顯示方式。
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // 在嘗試 5 次後增加等待時間，以防網速較慢。
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // 有時 TC 在同一頁使用多個連結，因此我們要去重複處理。
                url = getPathnameFromUrl(selectedNavLink.href);
            } else {
                // 去除行銷參數與網域名稱
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
                // 去除行銷參數與網域名稱
                return parsedUrl.pathname;
            } catch (error) {
                console.error("Invalid URL", url, error);
                return window.location.pathname; // 預設為目前頁面，至少在某些情況下仍可運作
            }
        }

    })();
</script>
[inline-code-end]

Now paste it into the code block on the left in the ThriveCart editor. It should look like this:

<div class="screenshot white-bg">
    <div class="title">已新增程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="已新增程式碼" />
</div>

That's it! Now we just have to publish:

<div class="screenshot white-bg">
    <div class="title">發佈</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="發佈" />
</div>

That's it! You should now see the comment box on your course when you preview, and real users will be able to leave comments **without signing in or leaving their username/email a second time**.

### 測試注意！

If you have anonymous commenting disabled, which it is by default, you won't be able to leave comments in `Preview` mode as the `John Smith` user. You will get an authentication
error as the default `John Smith` user has no email. If you want to test, we suggest you use a coupon code and go through your site like an actual user.