Das FastComments-Snippet für Framer Live Comments finden Sie unten.

[inline-code-attrs-start title = 'FastComments Framer-spezifisches Kommentare-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Alternativ können Sie auch das Streaming-Chat-Widget verwenden. Das FastComments-Snippet für Framer Streaming Chat lautet:

[inline-code-attrs-start title = 'FastComments Framer-spezifisches Streaming-Chat-Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

FastComments unterstützt den Framer-Editor, daher sollten Sie nach dem Einfügen des Codes etwa Folgendes sehen (möglicherweise müssen Sie auf `Publish` klicken):

<div class="screenshot white-bg">
    <div class="title">Vorschau des Kommentar-Widgets</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Vorschau des Kommentar-Widgets" />
</div>

Wenn Sie nun Ihre Website aufrufen, sollten Sie den Kommentarbereich sehen! In der Seitenleiste von Framer können Sie das Widget bei Bedarf auch auf die volle Breite einstellen.

Beachten Sie, dass Framer die Höhe von Widgets begrenzt und keine automatische Größenanpassung unterstützt. Daher haben wir uns hier für das Live-Chat-Widget entschieden, da es eine feste Höhe hat.