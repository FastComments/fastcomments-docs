Di seguito trovi il codice Vanilla JS per installare il Summary Widget. Anche la libreria React dispone di questo widget.

[inline-code-attrs-start title = 'Installazione del Widget di riepilogo'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Il widget troverà automaticamente le domande da mostrare nel riepilogo in base alla configurazione del widget corrispondente per quella pagina/sito.

### Filtrare i commenti per valutazione

Quando il widget dei commenti per la stessa pagina è anch'esso presente sulla pagina, e viene configurato per mostrare la risposta di ciascun commentatore nel proprio commento, le righe della suddivisione diventano cliccabili. Cliccando su una riga, ad esempio "5", il widget dei commenti viene filtrato per mostrare solo i commenti che hanno dato quella risposta. Cliccando nuovamente sulla riga, o sulla X del chip "Showing only" nel widget dei commenti, vengono mostrati nuovamente tutti i commenti. Non è necessaria alcuna configurazione: il Summary Widget trova il widget dei commenti con lo stesso `tenantId` e `urlId`.

Questo funziona per domande a stelle, pollici, emoji, scala e scelta singola. La suddivisione che il widget dei commenti mostra sopra la casella del commento può essere cliccata allo stesso modo.

Se hai bisogno del widget in una delle nostre altre librerie che non lo include, apri un ticket di supporto in modo che possiamo aggiungerlo.