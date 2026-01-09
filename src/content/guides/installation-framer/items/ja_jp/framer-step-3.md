The Framer の Live Comments 用 FastComments スニペットは以下のとおりです。

[inline-code-attrs-start title = 'FastComments Framer専用コメントスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // 一部のプロバイダはコードスニペットを async に変更します
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

Or, alternatively, you can use the Streaming Chat widget. The Framer Streaming Chat FastComments snippet is:

[inline-code-attrs-start title = 'FastComments Framer専用ストリーミングチャットスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // 一部のプロバイダはコードスニペットを async に変更します
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

FastComments は Framer エディタをサポートしているため、コードを貼り付けると次のように表示されます（`Publish` をクリックする必要があるかもしれません）:

<div class="screenshot white-bg">
    <div class="title">コメントウィジェットのプレビュー</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="コメントウィジェットのプレビュー" />
</div>

サイトを表示するとコメント領域が表示されます。必要に応じて、Framer のサイドバーでウィジェットをフル幅に設定することもできます。

Framer はウィジェットの高さに制限があり自動リサイズをサポートしていないため、ここでは高さが固定されている Live Chat ウィジェットを選択しています。