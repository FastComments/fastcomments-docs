Nedenfor er Vanilla JS-koden til installation af Summary Widget. React-biblioteket har også denne widget.

[inline-code-attrs-start title = 'Installation af Summary Widget'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Widgeten finder automatisk de spørgsmål, der skal vises i oversigten, baseret på den tilsvarende widget-konfiguration for den pågældende side/hjemmeside.

### Filtrering af kommentarer efter vurdering

Når kommentar-widgetten for den samme side også er på siden, og den er konfigureret til at vise hver kommentators svar på deres kommentar, bliver rækkerne i opdelingen
klikbare. Ved at klikke på en række, f.eks. "5", filtreres kommentar-widgetten til de kommentarer, der gav det svar. Ved at klikke på rækken igen, eller på X'et på "Viser kun"-chippen
i kommentar-widgetten, vises alle kommentarerne igen. Dette kræver ingen konfiguration: Summary Widget finder kommentar-widgetten med samme `tenantId` og `urlId`.

Dette virker for stjerne-, tommelfinger-, emoji-, skala- og enkeltvalg-spørgsmål. Opdelingen, som kommentar-widgetten viser over kommentarboksen, kan klikkes på på samme måde.

Hvis du har brug for widgetten i et af vores andre biblioteker, der ikke har den, så opret en supportticket, så vi ved, at vi skal tilføje den.