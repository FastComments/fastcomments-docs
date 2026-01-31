Não é recomendado adicionar o FastComments via Page Builder do BigCommerce, pois o código terá que ser adicionado manualmente a cada página desejada.

No entanto, se isso for desejado, o trecho de código a seguir deve ser usado. Trechos de código de outros tutoriais não funcionarão devido à natureza do BigCommerce:

[inline-code-attrs-start title = 'Trecho do Page Builder do BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]