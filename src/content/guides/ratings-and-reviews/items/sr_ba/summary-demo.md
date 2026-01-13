Ево малог демо примера видгета за сажетак:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // уклони параметре упита
    });
</script>

### Кеширање

Имајте у виду да се подаци за сажетак кеширају 30 секунди, или пет минута ако има велики број рецензија. Због тога ваша рецензија можда неће одмах бити видљива у сажетку, али то нам омогућава да смањимо трошкове приказивања видгета за сажетак.