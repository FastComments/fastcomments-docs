Hier is een kleine demo van de samenvattingswidget:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // verwijder queryparameters
    });
</script>

### Cache

Let op dat de samenvattingen gedurende 30 seconden worden gecached, of vijf minuten als er een groot aantal beoordelingen is. Hierdoor kan uw beoordeling mogelijk niet direct in de samenvatting verschijnen, maar dit stelt ons in staat de kosten voor het leveren van de samenvattingswidget te verlagen.