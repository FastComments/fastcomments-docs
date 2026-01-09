Maintenant que vous avez ajouté un bloc HTML personnalisé, nous pouvons ajouter le code du widget FastComments.

**Utilisez le code suivant pour Godaddy, pas le code provenant d'autres tutoriels. Ce code est spécifique à Godaddy.**

Copiez le code suivant :

[inline-code-attrs-start title = 'Extrait de code pour les commentaires Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Cet extrait de code est conçu pour être compatible avec Godaddy, et s'affichera uniquement sur vos articles de blog - pas sur la page d'accueil.

Collez maintenant le code dans la zone `Custom Code` mentionnée dans `Step One`.

<div class="screenshot white-bg">
    <div class="title">Copiez et collez le code</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Copiez et collez le code" />
</div>

Cliquez sur Terminé en haut à droite :

<div class="screenshot white-bg">
    <div class="title">Cliquez sur Terminé</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Cliquez sur Terminé" />
</div>

C'est tout pour l'étape deux !

---