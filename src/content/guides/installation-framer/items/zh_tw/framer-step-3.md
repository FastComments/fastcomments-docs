下面是 Framer Live Comments 的 FastComments 程式碼片段。

[inline-code-attrs-start title = 'FastComments Framer 專用評論程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // 有些供應商會將程式碼片段改為非同步
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

或者，您也可以使用 Streaming Chat 小工具。下面是 Framer Streaming Chat 的 FastComments 程式碼片段：

[inline-code-attrs-start title = 'FastComments Framer 專用串流聊天程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // 有些供應商會將程式碼片段改為非同步
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
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

FastComments 支援 Framer 編輯器，因此將程式碼貼上後您應該會看到類似的畫面（您可能需要點擊 `Publish`）：

<div class="screenshot white-bg">
    <div class="title">評論小工具預覽</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="評論小工具預覽" />
</div>

現在當您檢視網站時，應該會看到評論區！在 Framer 的側邊欄您也可以將小工具設為全寬（如有需要）。

請注意 Framer 限制小工具的高度且不支援自動調整大小，因此我們在此選擇了 Live Chat
小工具，因為它具有固定高度。