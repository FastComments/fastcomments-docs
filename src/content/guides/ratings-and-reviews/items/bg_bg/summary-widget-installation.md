---
Подолу е Vanilla JS кодът за инсталиране на Summary Widget. React библиотеката също разполага с този widget.

[inline-code-attrs-start title = 'Инсталиране на Summary Widget'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

Widget‑ът автоматично ще намери въпросите, които да се покажат в резюмето, въз основа на съответната конфигурация на widget‑а за тази страница/сайт.

Ако имате нужда от widget в една от нашите други библиотеки, която не го съдържа, подайте заявка за поддръжка, за да знаем, че трябва да го добавим.

---