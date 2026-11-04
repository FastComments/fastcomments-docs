Lo snippet FastComments per i Live Comments di Framer è riportato di seguito.

[inline-code-attrs-start title = 'Snippet Commenti FastComments Specifico per Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

In alternativa, puoi utilizzare il widget Streaming Chat. Lo snippet FastComments per lo Streaming Chat di Framer è:

[inline-code-attrs-start title = 'Snippet Streaming Chat FastComments Specifico per Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

FastComments supporta l'editor di Framer, quindi una volta incollato il codice dovresti vedere qualcosa di simile (potresti dover cliccare su `Publish`):

<div class="screenshot white-bg">
    <div class="title">Anteprima del Widget Commenti</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Anteprima del Widget Commenti" />
</div>

Ora, quando visualizzerai il tuo sito, dovresti vedere l'area dei commenti! Nella barra laterale di Framer puoi anche impostare il widget a larghezza piena, se desiderato.

Tieni presente che Framer limita l'altezza dei widget e non supporta il ridimensionamento automatico, quindi qui abbiamo scelto il widget Live Chat poiché ha un'altezza fissa.