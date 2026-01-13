След това ще превъртим надолу до ред `100`:

<div class="screenshot white-bg">
    <div class="title">Превъртете до ред 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Превъртете до ред 100" />
</div>

Сега копирайте следния фрагмент от код, който е предназначен **специално за Shopify - не използвайте фрагменти от други уроци**:

[inline-code-attrs-start title = 'FastComments фрагмент за Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        urlId: window.location.pathname
    });
</script>
[inline-code-end]

Сега искаме да поставим курсора на `line 101` - точно след `</div>` - и да поставим кода. Трябва да имате нещо като това:

<div class="screenshot white-bg">
    <div class="title">Добавете кода на FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Добавете кода на FastComments" />
</div>

Сега можем да запазим:

<div class="screenshot white-bg">
    <div class="title">Запазване</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Запазване" />
</div>