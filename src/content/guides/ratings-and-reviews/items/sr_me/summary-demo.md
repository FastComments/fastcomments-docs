Ево малог демо примјера резиме виджета:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // уклони параметре упита
    });
</script>

### Кеширање

Имајте у виду да се резимеи кеширају 30 секунди, или пет минута ако постоји велики број рецензија. Због тога, ваша рецензија можда се неће одмах појавити у резимеу, али ово
нам омогућава да смањимо трошкове сервирања резиме виджета.