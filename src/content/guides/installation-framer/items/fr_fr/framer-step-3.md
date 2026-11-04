Le snippet FastComments pour les commentaires en direct de Framer est le suivant.

[inline-code-attrs-start title = 'Snippet de commentaires FastComments spécifique à Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ou, alternativement, vous pouvez utiliser le widget Streaming Chat. Le snippet FastComments Streaming Chat spécifique à Framer est :

[inline-code-attrs-start title = 'Snippet Streaming Chat FastComments spécifique à Framer'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

FastComments prend en charge l'éditeur Framer, vous devriez donc voir quelque chose comme ceci une fois le code collé (vous devrez peut-être cliquer sur `Publish`) :

<div class="screenshot white-bg">
    <div class="title">Aperçu du widget de commentaires</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Aperçu du widget de commentaires" />
</div>

Désormais, lorsque vous consultez votre site, vous devriez voir la zone de commentaires ! Dans la barre latérale de Framer, vous pouvez également définir le widget en pleine largeur, si vous le souhaitez.

Notez que Framer limite la hauteur des widgets et ne prend pas en charge le redimensionnement automatique, c'est pourquoi nous avons choisi ici le widget
Live Chat, car il a une hauteur fixe.