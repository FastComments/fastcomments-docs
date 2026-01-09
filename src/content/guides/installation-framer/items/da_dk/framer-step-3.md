Framer Live Comments FastComments-udsnittet er vist nedenfor.

[inline-code-attrs-start title = 'FastComments Framer-specifikt kommentarudsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // nogle udbydere ændrer kodeudsnittet til at være async
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

[inline-code-attrs-start title = 'FastComments Framer-specifikt streaming-chat-udsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // nogle udbydere ændrer kodeudsnittet til at være async
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

FastComments understøtter Framer-editoren, så du bør se noget lignende, når du indsætter koden (du skal muligvis klikke på `Publish`):

<div class="screenshot white-bg">
    <div class="title">Forhåndsvisning af kommentar-widget</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Forhåndsvisning af kommentar-widget" />
</div>

Når du nu besøger dit site, bør du se kommentarsektionen! I sidebjælken i Framer kan du også angive widget'en til fuld bredde, hvis du ønsker det.

Bemærk, at Framer begrænser højden af widgets og ikke understøtter automatisk ændring af størrelse, så vi har valgt Live Chat-widget'en her, da den har en fast højde.