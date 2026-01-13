Ecco una piccola demo del widget di riepilogo:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // rimuovi i parametri della query
    });
</script>

### Memorizzazione nella cache

Nota che i riepiloghi vengono memorizzati nella cache per 30 secondi, o cinque minuti se c'è un gran numero di recensioni. Per questo motivo, la tua recensione potrebbe non comparire immediatamente nel riepilogo, ma questo ci permette di ridurre il costo di erogazione del widget di riepilogo.

---