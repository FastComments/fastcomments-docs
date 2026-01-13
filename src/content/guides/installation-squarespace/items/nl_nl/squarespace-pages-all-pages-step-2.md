Nu kunnen we het volgende codefragment kopiëren. Gebruik de kopieerknop die rechtsboven in het fragment verschijnt.

Er zijn een paar dingen die u in de code kunt configureren, zie regels 4 tot en met 7.

[inline-code-attrs-start title = "Squarespace Reacties op alle pagina's"; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // uw account-id

        function tryLoad() {
            // probeer te laden voor verschillende indelingen
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

...plak het vervolgens in het codegebied en klik op opslaan. Het zou er zo uit moeten zien, met de code in het `FOOTER`-blok:

<div class="screenshot white-bg">
    <div class="title">Plakken en Opslaan</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Plakken en Opslaan" />
</div>

Als u problemen ondervindt, controleer dan onderaan of er niet `"tenantId": "demo"` staat. Het zou uw tenant id moeten tonen als u bent ingelogd. Zo niet, neem contact op met de ondersteuning.