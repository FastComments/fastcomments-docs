---
Het FastComments-fragment voor Framer Live Comments staat hieronder.

[inline-code-attrs-start title = 'FastComments Framer-specifiek Reactiesfragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // sommige providers wijzigen het codefragment zodat het asynchroon wordt
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

Of je kunt in plaats daarvan de Streaming Chat-widget gebruiken. Het FastComments-fragment voor Framer Streaming Chat is:

[inline-code-attrs-start title = 'FastComments Framer-specifiek Streaming Chat-fragment'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // sommige providers wijzigen het codefragment zodat het asynchroon wordt
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

FastComments ondersteunt de Framer-editor, dus je zou iets dergelijks moeten zien zodra je de code plakt (mogelijk moet je op `Publish` klikken):

<div class="screenshot white-bg">
    <div class="title">Voorbeeld van de reacties-widget</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Voorbeeld van de reacties-widget" />
</div>

Wanneer je nu je site bekijkt, zou je het reactiegebied moeten zien! In de zijbalk van Framer kun je het widget ook op volledige breedte instellen, indien gewenst.

Houd er rekening mee dat Framer de hoogte van widgets beperkt en geen automatische grootte-aanpassing ondersteunt, daarom hebben we hier gekozen voor de Live Chat-widget omdat deze een vaste hoogte heeft.

---