Maintenant que nous sommes dans l'éditeur de modèles, nous devons décider où nous voulons afficher les commentaires, ou le chat en direct.

Dans cet exemple, nous l'ajouterons directement sous la vidéo. Survolez l'élément auquel vous voulez ajouter le widget, puis cliquez sur `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Ajouter un élément</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Add Element" />
</div>

Sélectionnez `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Sélectionnez CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Select CUSTOM JS/HTML" />
</div>

Ouvrons maintenant l'éditeur de code où nous collerons notre code.

ClickFunnels est un peu déroutant à l'étape suivante.

Il est important de *NE PAS* sélectionner `Code` lorsque vous survolez le nouvel élément. Sélectionnez plutôt `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Sélectionnez SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Select SETTINGS" />
</div>

Maintenant, sur la droite, nous pouvons cliquer sur `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Cliquez sur Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Click Open Code Editor" />
</div>

Une grande fenêtre s'ouvrira. C'est ici que nous pouvons coller notre code. Copiez l'extrait suivant (utilisez le bouton de copie en haut à droite):

[inline-code-attrs-start title = 'Extrait de code pour le chat en streaming ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
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
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Cet extrait de code est pour notre produit Streaming Chat, adapté aux vidéos. Si vous souhaitez plutôt l'extrait de code du widget Live Commenting, qui convient mieux aux pages classiques ou aux articles de blog, il se trouve à la fin de ce tutoriel.

Lorsque nous collons l'extrait de code dans la fenêtre, cela devrait ressembler à ceci :

<div class="screenshot white-bg">
    <div class="title">Coller le code</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Paste Code" />
</div>

Il ne reste plus qu'à fermer la fenêtre :

<div class="screenshot white-bg">
    <div class="title">Fermer</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Close" />
</div>

Vous pouvez maintenant prévisualiser vos modifications ! N'hésitez pas à déplacer le widget pour voir où il vous convient le mieux.

<div class="screenshot white-bg">
    <div class="title">Aperçu</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Preview" />
</div>

Succès ! N'oubliez pas de tester sur mobile !

<div class="screenshot white-bg">
    <div class="title">Succès !</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Success!" />
</div>