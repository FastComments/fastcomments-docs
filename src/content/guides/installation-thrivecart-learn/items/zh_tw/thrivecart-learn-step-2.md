在步驟 2 中，我們需要複製我們的程式碼片段。請確認第 50 行沒有顯示 "demo" - 你需要確保這裡填入的是你的租戶 ID。它應該已經為你自動填好了。

現在讓我們複製 ThriveCart-Learn 專用的 FastComments 程式碼片段。

這段程式碼相當長，因為與 ThriveCart 的整合包含許多功能，所以只需點擊程式碼片段右上角的「複製」按鈕：

[inline-code-attrs-start title = 'ThriveCart Learn+ 留言程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
                profileLink = document.querySelector('.thrivecart-courses-header-profile'); // 預覽時的 class 不同。
            }
            // 寬泛的電子郵件輸入欄位選擇器，以防 ThriveCart 更改 id。
            const emailInputField = document.querySelector('input[type=email]');
            if (emailInputField && emailInputField.value) {
                isAuthenticated = true;
                simpleSSO.email = emailInputField.value;
            } else if (profileLink && !profileLink.innerText.includes('John Smith')) { // 讓預覽可以運作 - 沒有可用的電子郵件。
                attemptsRemaining--;
                if (!attemptsRemaining) {
                    return console.error('Could not load FastComments - could not determine user information (email). Please reach out to FastComments support.');
                }
                console.warn('FastComments: No user email found - waiting and trying again.');
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // 5 次嘗試後增加等待時間，以防網路緩慢。
            }
            if (profileLink) {
                // 使用原始的 "img" 查詢，以防 ThriveCart 更改圖片 class 選擇器。
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
                return setTimeout(tryLoad, attemptsRemaining < 5 ? 3000 : 100); // 5 次嘗試後增加等待時間，以防網路緩慢。
            }

            let url;
            const selectedNavLink = document.querySelector('.tcc-browse-lesson.active a');

            if (selectedNavLink) {
                // 有時 TC 會在同一頁面使用多個連結，所以我們要去除重複。
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
                return window.location.pathname; // 預設使用目前路徑，至少有時能正常運作
            }
        }

    })();
</script>
[inline-code-end]

現在將它貼到 ThriveCart 編輯器左側的程式碼區塊中。它應該看起來像這樣：

<div class="screenshot white-bg">
    <div class="title">已新增程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-1-paste-code.png" alt="已新增程式碼" />
</div>

就是這樣！現在我們只需要發佈：

<div class="screenshot white-bg">
    <div class="title">發佈</div>
    <img class="screenshot-image" src="/images/installation-guides/thrivecart-learn-step-2-2-publish.png" alt="發佈" />
</div>

完成了！現在當你預覽課程時，應該會看到留言框，而且真實使用者將能夠**無需登入或再次輸入使用者名稱/電子郵件**即可留言。

### 測試注意事項！

如果你停用了匿名留言功能（預設為停用），你將無法在 `Preview` 模式下以 `John Smith` 使用者身分留言。你會收到驗證錯誤，因為預設的 `John Smith` 使用者沒有電子郵件。如果你想進行測試，我們建議你使用優惠碼，並像真實使用者一樣瀏覽你的網站。