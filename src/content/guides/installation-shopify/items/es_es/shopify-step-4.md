---
A continuación vamos a desplazarnos hasta la línea `100`:

<div class="screenshot white-bg">
    <div class="title">Desplázate hasta la línea 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Scroll to Line 100" />
</div>

Ahora copia el siguiente fragmento de código, que está diseñado **específicamente para Shopify - no uses fragmentos de otros tutoriales**:

[inline-code-attrs-start title = 'Fragmento de FastComments para Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ahora queremos colocar el cursor en `line 101` - justo después del `</div>` - y pegar. Deberías tener algo como esto:

<div class="screenshot white-bg">
    <div class="title">Añadir el código de FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Add The FastComments Code" />
</div>

Ahora podemos guardar:

<div class="screenshot white-bg">
    <div class="title">Guardar</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Save" />
</div>

---