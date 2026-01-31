Ahora podemos copiar el siguiente fragmento de código. Usa el botón de copiar que aparece en la esquina superior derecha del fragmento.

Hay algunas cosas que puedes configurar en el código, consulta las líneas 4 a 7.

[inline-code-attrs-start title = 'Código de Squarespace (página única)'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: 'demo' // tu id de cuenta
    }];
</script>
[inline-code-end]

It should look like this:

<div class="screenshot white-bg">
    <div class="title">Pegar y Guardar</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Pegar y Guardar" />
</div>

Ahora haz clic en guardar en la esquina superior derecha.

Ten en cuenta que la opción `Preview in Safe Mode` no funcionará, pero el widget aparecerá cuando visites tu sitio.

Si tienes problemas, asegúrate de que cerca de la parte inferior no diga "tenantId": "demo". Debe mostrar tu ID de tenant si has iniciado sesión. Si no, ponte en contacto con el soporte.