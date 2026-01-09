Небольшая демонстрация виджета сводки:

<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'L177BUDVvSe',
        urlId: window.location.origin + window.location.pathname // remove query params
    });
</script>

### Кэширование

Обратите внимание, что сводки кэшируются в течение 30 секунд, или пяти минут, если имеется большое количество отзывов. В связи с этим ваш отзыв может не отобразиться сразу в сводке, но это
позволяет нам снизить стоимость обслуживания виджета сводки.