---
Hieronder staat de Vanilla JS-code om de Samenvattingswidget te installeren. De React-bibliotheek bevat deze widget ook.

[inline-code-attrs-start title = 'Installatie van de Samenvattingswidget'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

De widget zal automatisch de vragen vinden die in de samenvatting worden weergegeven op basis van de bijbehorende widgetconfiguratie voor die pagina/site.

Als u de widget nodig hebt in een van onze andere bibliotheken die deze niet bevat, dient u dan een supportticket in zodat wij weten dat we deze moeten toevoegen.

---