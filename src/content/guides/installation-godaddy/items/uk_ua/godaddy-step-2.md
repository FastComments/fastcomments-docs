---
Тепер, коли ви додали блок із власним HTML, ми можемо додати код віджета FastComments.

**Використовуйте наведений код саме для Godaddy, а не код з інших підручників. Цей код специфічний для Godaddy.**

Скопіюйте наступний код:

[inline-code-attrs-start title = 'Фрагмент коду коментарів Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        if (top.location.pathname && top.location.pathname.includes('/f')) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
        }
    })();
</script>
[inline-code-end]

Цей конкретний фрагмент коду розроблено для сумісності з Godaddy, і він відображатиметься лише в ваших блогових записах — не на головній сторінці.

Тепер вставте код у область `Custom Code`, зазначену в `Step One`.

<div class="screenshot white-bg">
    <div class="title">Скопіюйте та вставте код</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Скопіюйте та вставте код" />
</div>

Натисніть «Готово» у верхньому правому куті:

<div class="screenshot white-bg">
    <div class="title">Натисніть «Готово»</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Натисніть «Готово»" />
</div>

Ось і все для другого кроку!

---