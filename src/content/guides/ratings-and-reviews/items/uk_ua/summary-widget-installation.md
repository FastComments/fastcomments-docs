Нижче наведено код Vanilla JS для встановлення віджета зведення. Бібліотека React також містить цей віджет.

[inline-code-attrs-start title = 'Встановлення віджета зведення'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Віджет автоматично знайде питання, які потрібно показати у зведенні, на основі відповідної конфігурації віджета для цієї сторінки/сайту.

Якщо вам потрібен цей віджет в одній із наших інших бібліотек, яка його не має, створіть запит у службу підтримки, щоб ми знали додати його.