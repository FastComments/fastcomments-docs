Hieronder staat de Vanilla JS-code voor het installeren van de Summary Widget. De React-bibliotheek heeft deze widget ook.

[inline-code-attrs-start title = 'Summary Widget Installatie'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

De widget vindt automatisch de vragen die in de samenvatting moeten worden getoond op basis van de bijbehorende widgetconfiguratie voor die pagina/site.

### Reacties filteren op beoordeling

Wanneer de comment-widget voor dezelfde pagina ook op de pagina staat, en deze is geconfigureerd om het antwoord van elke reactiegever op diens reactie te tonen, worden de rijen van de uitsplitsing
klikbaar. Door op een rij te klikken, bijvoorbeeld "5", wordt de comment-widget gefilterd op de reacties die dat antwoord hebben gegeven. Door opnieuw op de rij te klikken, of op de X op de "Showing only"-chip
in de comment-widget, worden alle reacties weer getoond. Hiervoor is geen configuratie nodig: de Summary Widget vindt de comment-widget met dezelfde `tenantId` en `urlId`.

Dit werkt voor sterren, duimen, emoji, schaal- en meerkeuzevragen. De uitsplitsing die de comment-widget boven het reactievak toont, kan op dezelfde manier worden aangeklikt.

Als u de widget nodig heeft in een van onze andere bibliotheken die deze niet bevat, dien dan een supportticket in zodat wij weten dat we deze moeten toevoegen.