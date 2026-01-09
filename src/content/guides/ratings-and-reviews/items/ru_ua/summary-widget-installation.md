---
Ниже приведён код на Vanilla JS для установки виджета Summary. Библиотека React также содержит этот виджет.

[inline-code-attrs-start title = 'Установка виджета Summary'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Виджет автоматически найдёт вопросы, которые нужно показать в сводке, на основе соответствующей конфигурации виджета для этой страницы/сайта.

Если вам нужен этот виджет в одной из наших других библиотек, где он отсутствует, отправьте запрос в службу поддержки, чтобы мы знали о необходимости добавить его.

---