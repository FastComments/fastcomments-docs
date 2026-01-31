Ahora vamos a copiar nuestro código. Si el fragmento de código dice `tenantId: "demo"` en la línea 6 entonces debes iniciar sesión en tu cuenta de FastComments
y luego actualizar esta página para que el fragmento de código copiado tenga el id de tu cuenta.

[inline-code-attrs-start title = 'Fragmento de Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Ahora pégalo en el editor y haz clic en Guardar:

<div class="screenshot white-bg">
    <div class="title">Agregar el código de FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Agregar el código de FastComments" />
</div>

... luego guarda tu sitio. ¡Eso es todo!