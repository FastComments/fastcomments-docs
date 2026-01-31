Nu kan vi kopiere følgende kodeudsnit. Brug kopier-knappen, der vises øverst til højre i uddraget.

Der er et par ting, du kan konfigurere i koden, se linjerne 4 til 7.

[inline-code-attrs-start title = 'Squarespace kode til enkelt side'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // din konto-id
    }];
</script>
[inline-code-end]

Det skulle se sådan ud:

<div class="screenshot white-bg">
    <div class="title">Indsæt og gem</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Indsæt og gem" />
</div>

Klik nu på Gem øverst til højre.

Bemærk, at `Preview in Safe Mode`-indstillingen ikke vil fungere, men widgetten vises, når du besøger dit site.

Hvis du har problemer, skal du sikre dig, at der nær bunden ikke står `"tenantId": "demo"`. Det bør vise din tenant-id, hvis du er logget ind. Hvis ikke, kontakt support.