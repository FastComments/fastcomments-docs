---
Below is the Vanilla-JS-Code zur Installation des Summary-Widgets. Die React-Bibliothek enthält dieses Widget ebenfalls.

[inline-code-attrs-start title = 'Installation des Summary-Widgets'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Das Widget findet automatisch die Fragen, die in der Zusammenfassung angezeigt werden sollen, basierend auf der entsprechenden Widget-Konfiguration für diese Seite/Website.

Wenn Sie das Widget in einer unserer anderen Bibliotheken benötigen, die es noch nicht enthält, reichen Sie bitte ein Support-Ticket ein, damit wir wissen, dass wir es hinzufügen sollen.

---