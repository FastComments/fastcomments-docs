---
Nous allons maintenant copier notre code. Si l'extrait de code indique `tenantId: "demo"` à la ligne 6, vous devez vous connecter à votre compte FastComments, puis actualiser cette page afin que l'extrait copié contienne l'ID de votre compte.

[inline-code-attrs-start title = 'Extrait Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo"
    });
</script>
[inline-code-end]

Collez-le maintenant dans l'éditeur et cliquez sur Enregistrer :

<div class="screenshot white-bg">
    <div class="title">Ajouter le code FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Ajouter le code FastComments" />
</div>

... puis enregistrez votre site. Et voilà !

---