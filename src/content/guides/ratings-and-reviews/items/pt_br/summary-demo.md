---
Aqui está uma pequena demonstração do widget de resumo:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // remover parâmetros de consulta
    });
</script>

### Cache

Observe que os resumos são armazenados em cache por 30 segundos, ou cinco minutos se houver um grande número de avaliações. Por isso, sua avaliação pode não aparecer imediatamente no resumo, mas isso nos permite reduzir o custo de disponibilizar o widget de resumo.

---