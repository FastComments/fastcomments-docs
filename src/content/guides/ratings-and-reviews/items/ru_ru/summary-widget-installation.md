Ниже приведён код Vanilla JS для установки виджета сводки. Библиотека React также содержит этот виджет.

[inline-code-attrs-start title = 'Установка виджета сводки'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Виджет автоматически найдёт вопросы, которые необходимо показать в сводке, на основе соответствующей конфигурации виджета для этой страницы/сайта.

Если вам нужен этот виджет в одной из наших других библиотек, где он отсутствует, отправьте запрос в службу поддержки, чтобы мы знали, что нужно добавить его.

---