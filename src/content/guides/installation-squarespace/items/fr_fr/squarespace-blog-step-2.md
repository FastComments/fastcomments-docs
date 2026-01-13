Nous pouvons maintenant copier le fragment de code suivant (utilisez le bouton de copie en haut à droite du fragment) :

[inline-code-attrs-start title = 'Code des commentaires du blog Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...puis collez-le dans la zone de code et cliquez sur Enregistrer :

<div class="screenshot white-bg">
    <div class="title">Coller et enregistrer</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Coller et enregistrer" />
</div>

---