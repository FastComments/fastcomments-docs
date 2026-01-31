Nous pouvons maintenant copier l'extrait de code suivant. Utilisez le bouton de copie qui apparaît en haut à droite de l'extrait.

Il y a quelques éléments que vous pouvez configurer dans le code, voir les lignes 4 à 7.

[inline-code-attrs-start title = 'Code pour une seule page Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // votre identifiant de compte
    }];
</script>
[inline-code-end]

Cela devrait ressembler à ceci :

<div class="screenshot white-bg">
    <div class="title">Coller et enregistrer</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Coller et enregistrer" />
</div>

Cliquez maintenant sur Enregistrer en haut à droite.

Notez que l'option `Preview in Safe Mode` ne fonctionnera pas, mais le widget apparaîtra lorsque vous visiterez votre site.

Si vous rencontrez des problèmes, assurez-vous qu'en bas il n'est pas indiqué `"tenantId": "demo"`. Il devrait afficher votre tenant id si vous êtes connecté. Sinon, contactez le support.