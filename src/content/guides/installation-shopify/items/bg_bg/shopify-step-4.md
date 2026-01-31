---
След това ще превъртим надолу до ред `100`:

<div class="screenshot white-bg">
    <div class="title">Превъртете до ред 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Превъртете до ред 100" />
</div>

Сега копирайте следния кодов фрагмент, който е предназначен **специално за Shopify - не използвайте фрагменти от други уроци**:

[inline-code-attrs-start title = 'Фрагмент FastComments за Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Сега поставете курсора на `line 101` - точно след `</div>` - и вмъкнете кода. Трябва да имате нещо подобно:

<div class="screenshot white-bg">
    <div class="title">Добавете кода на FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Добавете кода на FastComments" />
</div>

Сега можем да запазим:

<div class="screenshot white-bg">
    <div class="title">Запазване</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Запазване" />
</div>

---