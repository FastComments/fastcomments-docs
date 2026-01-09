L'extrait FastComments pour les commentaires Framer est ci‑dessous.

[inline-code-attrs-start title = 'Extrait FastComments spécifique à Framer : commentaires'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // certains fournisseurs modifient l'extrait de code pour le rendre asynchrone
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

[inline-code-attrs-start title = 'Extrait FastComments spécifique à Framer : chat en direct'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // certains fournisseurs modifient l'extrait de code pour le rendre asynchrone
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

FastComments prend en charge l'éditeur Framer, donc vous devriez voir quelque chose comme ceci une fois que vous avez collé le code (vous devrez peut‑être cliquer sur `Publish`) :

<div class="screenshot white-bg">
    <div class="title">Aperçu du widget de commentaires</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Aperçu du widget de commentaires" />
</div>

Désormais, quand vous consultez votre site, vous devriez voir la zone de commentaires ! Dans la barre latérale de Framer, vous pouvez également définir le widget en pleine largeur, si vous le souhaitez.

Notez que Framer limite la hauteur des widgets et ne prend pas en charge le redimensionnement automatique, donc nous avons choisi le Chat en direct
widget ici puisque sa hauteur est fixe.