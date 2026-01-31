Затим ћемо се спустити до реда `100`:

<div class="screenshot white-bg">
    <div class="title">Скролујте до реда 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Скролујте до реда 100" />
</div>

Сада копирајте следећи исјечак кода, који је дизајниран **изричито за Shopify - не користите исјечке кода из других туторијала**:

[inline-code-attrs-start title = 'FastComments исјечак за Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Сада треба да поставимо курсор на `line 101` - одмах након `</div>` - и налепимо. Требало би да имате нешто овако:

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