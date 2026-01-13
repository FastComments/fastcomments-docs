Di seguito è riportato il codice Vanilla JS per installare il widget Riepilogo. La libreria React include anch'essa questo widget.

[inline-code-attrs-start title = 'Installazione del widget Riepilogo'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Il widget troverà automaticamente le domande da mostrare nel riepilogo in base alla configurazione corrispondente del widget per quella pagina/sito.

Se hai bisogno del widget in una delle nostre altre librerie che non lo include, apri un ticket di supporto in modo che possiamo aggiungerlo.