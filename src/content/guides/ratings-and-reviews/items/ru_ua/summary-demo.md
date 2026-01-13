Вот небольшой демонстрационный пример виджета сводки:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // удалить параметры запроса
    });
</script>

### Кэширование

Обратите внимание, что сводки кэшируются в течение 30 секунд, или пяти минут при большом количестве отзывов. Из-за этого ваш отзыв может не появиться сразу в сводке, но это
позволяет нам снизить стоимость обслуживания виджета сводки.