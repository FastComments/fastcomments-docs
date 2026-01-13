Ahora podemos copiar el siguiente fragmento de código. Usa el botón de copiar que aparece en la esquina superior derecha del fragmento.

Hay algunas cosas que puedes configurar en el código, consulta las líneas 4 a 7.

[inline-code-attrs-start title = 'Código de página única de Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // tu id de cuenta

        function tryLoad() {
            window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

Debería verse así:

<div class="screenshot white-bg">
    <div class="title">Pegar y guardar</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-specific-page-step-2-1-add-code-and-save.png" alt="Pegar y guardar" />
</div>

Ahora haz clic en Guardar en la esquina superior derecha.

Ten en cuenta que la opción `Preview in Safe Mode` no funcionará, pero el widget aparecerá cuando visites tu sitio.

Si tienes problemas, asegúrate de que en la parte inferior no diga `"tenantId": "demo"`. Debería mostrar tu tenant id si has iniciado sesión. Si no, contacta con el soporte.