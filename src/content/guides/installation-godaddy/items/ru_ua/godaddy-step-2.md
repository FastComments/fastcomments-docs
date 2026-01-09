Теперь, когда вы добавили пользовательский HTML-блок, мы можем добавить код виджета FastComments.

**Используйте следующий код для Godaddy, а не код из других руководств. Этот код специфичен для Godaddy.**

Скопируйте следующий код:

[inline-code-attrs-start title = 'Фрагмент кода комментариев для Godaddy'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Этот конкретный фрагмент кода разработан для совместимости с Godaddy и будет отображаться только в ваших сообщениях блога, а не на главной странице.

Теперь вставьте код в область `Custom Code`, упомянутую в `Step One`.

<div class="screenshot white-bg">
    <div class="title">Скопируйте и вставьте код</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-code-added.png" alt="Скопируйте и вставьте код" />
</div>

Нажмите Done в правом верхнем углу:

<div class="screenshot white-bg">
    <div class="title">Нажмите Done</div>
    <img class="screenshot-image" src="/images/installation-guides/godaddy-step-2-done.png" alt="Нажмите Done" />
</div>

Вот и всё для второго шага!