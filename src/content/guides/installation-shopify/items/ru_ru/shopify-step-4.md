Далее мы прокрутим вниз до строки `100`:

<div class="screenshot white-bg">
    <div class="title">Прокрутите до строки 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Прокрутите до строки 100" />
</div>

Теперь скопируйте следующий фрагмент кода, который разработан **специально для Shopify — не используйте фрагменты кода из других руководств**:

[inline-code-attrs-start title = 'Фрагмент FastComments для Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

Теперь поместите курсор на `line 101` — сразу после `</div>` — и вставьте. У вас должно получиться что-то вроде этого:

<div class="screenshot white-bg">
    <div class="title">Добавьте код FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Добавьте код FastComments" />
</div>

Теперь можно сохранить:

<div class="screenshot white-bg">
    <div class="title">Сохранить</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Сохранить" />
</div>

---