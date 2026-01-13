現在我們可以複製下列程式碼片段。使用出現在片段右上角的複製按鈕。

程式碼中有幾個可配置的項目，請參閱第 4 到第 7 行。

[inline-code-attrs-start title = 'Squarespace 全部頁面評論程式碼'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // 你的帳戶 ID

        function tryLoad() {
            // 嘗試為不同的版面載入
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...然後將其貼到程式碼區並點擊儲存。它應該看起來像這樣，程式碼位於 `FOOTER` 區塊：

<div class="screenshot white-bg">
    <div class="title">貼上並儲存</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="貼上並儲存" />
</div>

如果您遇到問題，請確認頁面底部附近不會顯示 `"tenantId": "demo"`。它應該顯示您的 tenant id，如果您已登入。若沒有，請聯絡支援。