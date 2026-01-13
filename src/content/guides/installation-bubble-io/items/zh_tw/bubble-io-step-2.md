點擊您剛新增的 HTML 元素。在出現的屬性編輯器中，將以下程式碼貼到 HTML 欄位：

[inline-code-attrs-start title = 'Bubble.io 即時留言程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble 通常會將程式碼片段改為 async
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">插入 FastComments 程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="將 FastComments 程式碼插入 HTML 元素" />
</div>

注意：此程式碼包含重新嘗試機制，以確保 FastComments 在 Bubble 的動態環境中正確載入。其他程式碼片段將無法使用。

請記得在註冊後將 `'demo'` 替換為您實際的 FastComments 租戶 ID。如果您已登入 FastComments.com，應該會自動替換好。