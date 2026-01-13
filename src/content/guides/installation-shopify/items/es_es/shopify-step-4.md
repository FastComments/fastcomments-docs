---
A continuación vamos a desplazarnos hasta la línea `100`:

<div class="screenshot white-bg">
    <div class="title">Desplazarse hasta la línea 100</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-7-line-100.png" alt="Desplazarse hasta la línea 100" />
</div>

Ahora copie el siguiente fragmento de código, que está diseñado **específicamente para Shopify - no utilice fragmentos de código de otros tutoriales**:

[inline-code-attrs-start title = 'Fragmento de FastComments para Shopify'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ahora queremos colocar nuestro cursor en `line 101` - justo después del `</div>` - y pegar. Debería tener algo como esto:

<div class="screenshot white-bg">
    <div class="title">Agregar el código de FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-8-code-pasted.png" alt="Agregar el código de FastComments" />
</div>

Ahora podemos guardar:

<div class="screenshot white-bg">
    <div class="title">Guardar</div>
    <img class="screenshot-image" src="/images/installation-guides/shopify-step-2-9-save.png" alt="Guardar" />
</div>

---