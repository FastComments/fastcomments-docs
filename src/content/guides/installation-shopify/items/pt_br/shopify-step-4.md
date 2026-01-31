A seguir vamos rolar até a linha `100`:

<div class="screenshot white-bg">
    <div class="title">Rolar até a linha 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Rolar até a linha 100" />
</div>

Agora copie o trecho de código a seguir, que foi projetado **especificamente para Shopify - não use trechos de código de outros tutoriais**:

[inline-code-attrs-start title = 'Trecho do FastComments para Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Agora posicione o cursor na `linha 101` - logo após o `</div>` - e cole. Você deverá ver algo assim:

<div class="screenshot white-bg">
    <div class="title">Adicione o código do FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Adicione o código do FastComments" />
</div>

Agora podemos salvar:

<div class="screenshot white-bg">
    <div class="title">Salvar</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Salvar" />
</div>

---