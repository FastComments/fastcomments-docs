Next ми прокрутимо вниз до рядка `100`:

<div class="screenshot white-bg">
    <div class="title">Прокрутіть до рядка 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Прокрутіть до рядка 100" />
</div>

Тепер скопіюйте наступний фрагмент коду, який розроблено **спеціально для Shopify - не використовуйте фрагменти коду з інших підручників**:

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

Тепер помістіть курсор на `рядок 101` - відразу після `</div>` - і вставте. У вас має вийти приблизно таке:

<div class="screenshot white-bg">
    <div class="title">Додайте код FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Додайте код FastComments" />
</div>

Тепер ми можемо зберегти:

<div class="screenshot white-bg">
    <div class="title">Зберегти</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Зберегти" />
</div>