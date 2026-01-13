Cet exemple utilise du code personnalisé pour être compatible avec Wix. **Vous ne pourrez pas utiliser les extraits de code FastComments des autres tutoriels.**

Ouvrez le formulaire pour ajouter notre boîte de dialogue HTML personnalisée en cliquant sur `Enter Code` et en sélectionnant `HTML`:

<div class="screenshot white-bg">
    <div class="title">Étape 3 : Ouvrir la boîte de dialogue HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Étape 3 : Ouvrir la boîte de dialogue HTML" />
</div>

Copiez l'extrait HTML suivant et collez-le dans la boîte de dialogue, puis cliquez sur Mettre à jour :

[inline-code-attrs-start title = 'Extrait de code des commentaires Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Étape 3 : Coller et enregistrer</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Étape 3 : Coller et enregistrer" />
</div>

Vous devriez maintenant voir une version très petite du widget de commentaires :

<div class="screenshot white-bg">
    <div class="title">Étape 3 : Le résultat</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Étape 3 : Le résultat" />
</div>

Ensuite, nous positionnerons et redimensionnerons ceci pour l'adapter à notre page.