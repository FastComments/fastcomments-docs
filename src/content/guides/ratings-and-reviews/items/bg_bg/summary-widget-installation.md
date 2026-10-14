По-долу е Vanilla JS кодът за инсталиране на Summary Widget. React библиотеката също разполага с този widget.

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

Widget‑ът автоматично ще намери въпросите, които да се покажат в обобщението, въз основа на съответната конфигурация на widget‑а за тази страница/сайт.

### Филтриране на коментари по рейтинг

Когато comment widget‑ът за същата страница също се намира на страницата и е конфигуриран да показва отговора на всеки коментатор в неговия коментар, редовете на разбивката стават кликваеми. Кликването върху ред, например „5“, филтрира comment widget‑а до коментарите, които са дали този отговор. Кликването отново върху реда или върху X на чипа „Showing only“ в comment widget‑а показва отново всички коментари. Това не изисква конфигурация: Summary Widget‑ът намира comment widget‑а със същия `tenantId` и `urlId`.

Това работи за въпроси със звезди, палци, емоджи, скала и единичен избор. Разбивката, която comment widget‑ът показва над полето за коментари, може да се кликва по същия начин.

Ако се нуждаете от widget в една от нашите други библиотеки, която не го съдържа, подайте заявка за поддръжка, за да знаем да го добавим.