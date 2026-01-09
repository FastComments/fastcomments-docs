Ensuite, nous allons faire défiler jusqu'à la ligne `100` :

<div class="screenshot white-bg">
    <div class="title">Faire défiler jusqu'à la ligne 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Faire défiler jusqu'à la ligne 100" />
</div>

Copiez maintenant l'extrait de code suivant, qui est conçu **spécifiquement pour Shopify - n'utilisez pas les extraits de code provenant d'autres tutoriels** :

[inline-code-attrs-start title = 'Extrait FastComments pour Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        urlId: window.location.pathname
    });
</script>
[inline-code-end]

Maintenant, nous voulons placer notre curseur sur `line 101` - juste après le `</div>` - et coller. Vous devriez avoir quelque chose comme ceci :

<div class="screenshot white-bg">
    <div class="title">Ajouter le code FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Ajouter le code FastComments" />
</div>

Maintenant, nous pouvons enregistrer :

<div class="screenshot white-bg">
    <div class="title">Enregistrer</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Enregistrer" />
</div>