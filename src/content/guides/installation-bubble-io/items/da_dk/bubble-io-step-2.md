Klik på det HTML-element, du lige har tilføjet. I egenskabseditoren, der vises, indsæt følgende kode i HTML-feltet:

[inline-code-attrs-start title = 'Bubble.io Live-kommentering kodeudsnit'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble har en tendens til at ændre kodeudsnittet til at være async
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Indsæt FastComments-kode</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Indsættelse af FastComments-kode i HTML-element" />
</div>

Bemærk: Denne kode indeholder en genforsøgs-mekanisme for at sikre, at FastComments indlæses korrekt i Bubbles dynamiske miljø.
Andre kodeudsnit vil ikke fungere.

Husk at udskifte `'demo'` med dit faktiske FastComments-tenant-ID efter tilmelding. Hvis du er logget ind på FastComments.com, burde det allerede være udskiftet.