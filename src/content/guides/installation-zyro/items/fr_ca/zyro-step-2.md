Ajoutons maintenant le code de notre widget.

Copiez le code ci-dessous. Assurez-vous d'être connecté à [fastcomments.com](https://fastcomments.com) 
et rechargez cette page si ce n'est pas le cas, afin que le code soit prérempli avec les informations de votre compte, sinon, il affichera le code de démonstration.

Copions maintenant le code :

[inline-code-attrs-start title = 'Code de commentaires Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Retournez maintenant à votre constructeur de site et cliquez sur `Enter code` :

<div class="screenshot white-bg">
    <div class="title">Entrer le code</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Entrer le code" />
</div>

### Remarque !

Il est important d'utiliser le code ci‑dessous et non les extraits de code d'autres documentations, car cet extrait a été conçu spécifiquement
pour Zyro.

Vous devriez maintenant voir quelque chose comme ce qui suit, qui apparaît vide. C'est normal. Déplacez votre souris sur la zone où le widget devrait se trouver :

<div class="screenshot white-bg">
    <div class="title">Widget de code ajouté</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Widget de code ajouté" />
</div>

Faites maintenant glisser le widget pour lui donner la taille souhaitée ; vous le verrez apparaître :

<div class="screenshot white-bg">
    <div class="title">Redimensionnez-le</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Redimensionnez-le" />
</div>

...et maintenant prévisualisez et enregistrez !