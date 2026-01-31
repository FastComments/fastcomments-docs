Nous allons maintenant copier notre code. Si le extrait de code indique `tenantId: "demo"` à la ligne 6 alors vous devez vous connecter à votre compte FastComments et ensuite actualiser cette page afin que l'extrait de code copié contienne l'identifiant de votre compte.

[inline-code-attrs-start title = 'Extrait Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

Collez-le dans l'éditeur et cliquez sur Enregistrer :

<div class="screenshot white-bg">
    <div class="title">Ajouter le code FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Ajouter le code FastComments" />
</div>

... puis enregistrez votre site. C'est tout !

---