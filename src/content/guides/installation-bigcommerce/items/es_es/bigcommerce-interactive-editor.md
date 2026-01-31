---
No se recomienda añadir FastComments mediante el Page Builder de BigCommerce, ya que el código tendría que añadirse manualmente a cada página deseada.

Sin embargo, si esto se desea, debe usarse el siguiente fragmento de código. Los fragmentos de código de otros tutoriales no funcionarán debido a la naturaleza de BigCommerce:

[inline-code-attrs-start title = 'Fragmento de Page Builder de BigCommerce'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

---