現在我們可以複製以下程式碼片段。請使用片段右上角出現的複製按鈕。

程式碼中有幾個可設定的項目，請參閱第 4 至第 7 行。

[inline-code-attrs-start title = 'Squarespace 單一頁面程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // 您的帳戶 ID

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

It should look like this:

<div class="screenshot white-bg">
    <div class="title">貼上並儲存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="貼上並儲存" />
</div>

Now click save in the top right.

Note that the `Preview in Safe Mode` option will not work, but the widget will appear when you visit your site.

If you're having issues, make sure near the bottom it doesn't say `"tenantId": "demo"`. It should show your tenant id if you are logged in. If not, reach out to support.