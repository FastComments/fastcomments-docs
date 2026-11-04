FastComments Framer Live Comments-snippeten er nedenfor.

[inline-code-attrs-start title = 'FastComments Framer-specifik kommentar-snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Alternativt kan du bruge Streaming Chat-widgetten. FastComments Framer Streaming Chat-snippeten er:

[inline-code-attrs-start title = 'FastComments Framer-specifik Streaming Chat-snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

FastComments understøtter Framer-editoren, så du bør se noget i stil med dette, når du indsætter koden (du skal muligvis klikke på `Publish`):

<div class="screenshot white-bg">
    <div class="title">Forhåndsvisning af kommentar-widget</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Forhåndsvisning af kommentar-widget" />
</div>

Når du nu ser dit websted, bør du se kommentarområdet! I Framers sidebjælke kan du desuden indstille widgetten til fuld bredde, hvis ønsket.

Bemærk, at Framer begrænser højden på widgets og ikke understøtter automatisk tilpasning af størrelsen, så vi har valgt Live Chat-widgetten her, da den har fast højde.