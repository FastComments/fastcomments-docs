Следеће ћемо скроловати до реда `100`:

<div class="screenshot white-bg">
    <div class="title">Скроловати до реда 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Скроловати до реда 100" />
</div>

Сада копирајте сљедећи исјечак кода, који је дизајниран **исключиво за Shopify - не користите исјечке кода из других туторијала**:

[inline-code-attrs-start title = 'Исјечак FastComments за Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Сада желимо да поставимо курсор на `ред 101` - одмах након `</div>` - и да налепимо. Треба да имате нешто овако:

<div class="screenshot white-bg">
    <div class="title">Додајте FastComments код</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Додајте FastComments код" />
</div>

Сада можемо сачувати:

<div class="screenshot white-bg">
    <div class="title">Сачувај</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Сачувај" />
</div>

---