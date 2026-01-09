Pour que l’intégration Weebly et FastComments fonctionne bien, nous devons ajouter **deux** petits extraits de code.

Le premier extrait sert à masquer le message Weebly "Comments are Closed", et le second sert à charger réellement FastComments.

D'abord, copiez ce petit extrait de code :

[inline-code-attrs-start title = 'Extrait de code - en-tête FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

Ensuite, sur la même page de paramètres que `Step One`, cliquez sur le `+` à côté de `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Ouvrir le code d'en-tête de la publication</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Ouvrir le code d'en-tête de la publication" />
</div>

Vous devriez voir une zone de texte s'ouvrir comme ceci :

<div class="screenshot white-bg">
    <div class="title">Code d'en-tête de la publication ouvert</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Code d'en-tête de la publication ouvert" />
</div>

Collez maintenant notre extrait de code :

<div class="screenshot white-bg">
    <div class="title">Extrait de code d'en-tête collé</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Extrait de code d'en-tête collé" />
</div>

Ensuite, il faut ajouter le code de pied de page pour activer FastComments. Cliquez sur le signe plus à côté de `Post footer code` :

<div class="screenshot white-bg">
    <div class="title">Ouvrir le code de pied de page de la publication</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Ouvrir le code de pied de page de la publication" />
</div>

Copiez cet extrait de code conçu **spécifiquement pour Weebly** :

[inline-code-attrs-start title = 'Extrait de code - pied de page FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // remove show comments button
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Collez maintenant notre code de pied de page :

<div class="screenshot white-bg">
    <div class="title">Code de pied de page de la publication ajouté</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Code de pied de page de la publication ajouté" />
</div>

C'est tout !