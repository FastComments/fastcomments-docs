Nous pouvons maintenant copier l'extrait de code suivant. Utilisez le bouton de copie qui apparaît en haut à droite de l'extrait.

Il y a quelques éléments que vous pouvez configurer dans le code, voir les lignes 4 à 7.

[inline-code-attrs-start title = 'Code des commentaires pour toutes les pages Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // votre identifiant de compte

        function tryLoad() {
            // essayer de charger pour différentes mises en page
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...puis collez-le dans la zone de code et cliquez sur Enregistrer. Cela devrait ressembler à ceci, avec le code dans le bloc `FOOTER` :

<div class="screenshot white-bg">
    <div class="title">Coller et enregistrer</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Coller et enregistrer" />
</div>

Si vous rencontrez des problèmes, vérifiez qu'en bas il n'indique pas `"tenantId": "demo"`. Il devrait afficher votre tenant id si vous êtes connecté. Sinon, contactez le support.