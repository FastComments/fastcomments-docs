Ahora podemos copiar el siguiente fragmento de código. Use el botón de copiar que aparece en la esquina superior derecha del fragmento.

Hay algunas cosas que puede configurar en el código; vea las líneas 4 a 7.

[inline-code-attrs-start title = 'Código de comentarios para todas las páginas de Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // su ID de cuenta

        function tryLoad() {
            // intentar cargar para distintos diseños
            let targetDiv = document.querySelector('.blog-item-comments-content');
            if (!targetDiv) {
                targetDiv = document.getElementById('fastcomments-widget');
            }
            window.FastCommentsUI(targetDiv, {
                tenantId
            });
        }

        tryLoad();
    })();
</script>

[inline-code-end]

...luego pegue en el área de código y haga clic en guardar. Debería verse así, con el código en el bloque `FOOTER`:

<div class="screenshot white-bg">
    <div class="title">Pegar y guardar</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-pages-all-pages-step-2-1-add-code-and-save.png" alt="Pegar y guardar" />
</div>

Si tiene problemas, asegúrese de que cerca de la parte inferior no ponga `"tenantId": "demo"`. Debería mostrar su tenant id si ha iniciado sesión. Si no, póngase en contacto con el soporte.