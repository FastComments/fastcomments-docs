---
Следеће ћемо се померити надоле до реда `100`:

<div class="screenshot white-bg">
    <div class="title">Померите се до реда 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Померите се до реда 100" />
</div>

Сада копирајте следећи исечак кода, који је дизајниран **посебно за Shopify - не користите исечке кода из других туторијала**:

[inline-code-attrs-start title = 'Shopify FastComments исечак'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Сада желимо да поставимо курсор на `ред 101` - одмах после `</div>` - и налепимо. Требало би да имате нешто овако:

<div class="screenshot white-bg">
    <div class="title">Додајте FastComments код</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Додајте FastComments код" />
</div>

Сада можемо да сачувамо:

<div class="screenshot white-bg">
    <div class="title">Сачувај</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Сачувај" />
</div>

---