Nu kan vi kopiere følgende kodeuddrag. Brug kopier-knappen, der vises i øverste højre hjørne af uddraget.

Der er et par ting, du kan konfigurere i koden, se linjerne 4 til 7.

[inline-code-attrs-start title = 'Squarespace Kommentarkode for alle sider'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // dit konto-id

        function tryLoad() {
            // prøv at indlæse for forskellige layouts
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

...indsæt derefter i kodeområdet og klik gem. Det skal se sådan ud, med koden i `FOOTER`-blokken:

<div class="screenshot white-bg">
    <div class="title">Indsæt og gem</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Indsæt og gem" />
</div>

Hvis du har problemer, skal du sørge for, at der ikke står `"tenantId": "demo"` nær bunden. Det skulle vise din tenant id, hvis du er logget ind. Hvis ikke, kontakt support.