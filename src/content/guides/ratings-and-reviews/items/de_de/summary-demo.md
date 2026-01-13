Hier ist eine kleine Demo des Zusammenfassungs-Widgets:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // Entferne Abfrageparameter
    });
</script>

### Zwischenspeicherung

Beachte, dass die Zusammenfassungen für 30 Sekunden zwischengespeichert werden, oder fünf Minuten, wenn es eine große Anzahl von Bewertungen gibt. Aufgrund dessen kann Ihre Bewertung daher möglicherweise nicht sofort in der Zusammenfassung angezeigt werden, aber dies
ermöglicht es uns, die Kosten für das Bereitstellen des Zusammenfassungs-Widgets zu senken.