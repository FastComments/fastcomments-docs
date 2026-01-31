Nu kunnen we de volgende codefragment kopiëren. Gebruik de kopieerknop die rechtsboven in het codefragment verschijnt.

Er zijn een paar dingen die u in de code kunt configureren, zie regels 4 tot en met 7.

[inline-code-attrs-start title = 'Squarespace code voor enkele pagina'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // uw account-id
    }];
</script>
[inline-code-end]

Het zou er zo uit moeten zien:

<div class="screenshot white-bg">
    <div class="title">Plakken en opslaan</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Plakken en opslaan" />
</div>

Klik nu rechtsboven op Opslaan.

Houd er rekening mee dat de optie `Preview in Safe Mode` niet werkt, maar de widget verschijnt wanneer u uw site bezoekt.

Als u problemen ondervindt, zorg ervoor dat onderaan niet staat `"tenantId": "demo"`. Het zou uw tenant id moeten tonen als u bent ingelogd. Zo niet, neem contact op met ondersteuning.