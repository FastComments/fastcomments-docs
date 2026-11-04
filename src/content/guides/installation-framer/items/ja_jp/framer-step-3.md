Framer Live Comments 用の FastComments スニペットは以下の通りです。

[inline-code-attrs-start title = 'FastComments Framer 専用コメントスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
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

または、代わりに Streaming Chat ウィジェットを使用することもできます。Framer 用の FastComments Streaming Chat スニペットは以下の通りです:

[inline-code-attrs-start title = 'FastComments Framer 専用 Streaming Chat スニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // some providers change the code snippet to be async
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

FastComments は Framer エディターに対応しているため、コードを貼り付けると以下のような表示がされるはずです(`Publish` をクリックする必要がある場合があります):

<div class="screenshot white-bg">
    <div class="title">コメントウィジェットのプレビュー</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="コメントウィジェットのプレビュー" />
</div>

サイトを表示すると、コメントエリアが表示されるはずです。Framer のサイドバーで、必要に応じてウィジェットを全幅に設定することもできます。

なお、Framer はウィジェットの高さを制限しており、自動リサイズに対応していないため、ここでは固定高さの Live Chat ウィジェットを選択しています。