Nu kan du kopiere følgende kodeudsnit. Brug kopier-knappen, der vises øverst til højre i uddraget.

Der er et par ting, du kan konfigurere i koden, se linjer 4 til 7.

[inline-code-attrs-start title = 'Squarespace-kode til enkelt side'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // din konto-id

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Det skulle se sådan ud:

<div class="screenshot white-bg">
    <div class="title">Indsæt og gem</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Indsæt og gem" />
</div>

Klik nu på Gem øverst til højre.

Bemærk, at indstillingen `Preview in Safe Mode` ikke vil fungere, men widget'en vil dukke op, når du besøger dit site.

Hvis du oplever problemer, så sørg for, at der nær bunden ikke står `"tenantId": "demo"`. Det burde vise dit tenant id, hvis du er logget ind. Hvis ikke, kontakt support.