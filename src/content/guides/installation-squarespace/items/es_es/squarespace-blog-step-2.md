Ahora podemos copiar el siguiente fragmento de código (usa el botón de copiar en la esquina superior derecha del fragmento):

[inline-code-attrs-start title = 'Código de comentarios del blog de Squarespace'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const tenantId = 'demo'; // tu ID de cuenta

        function tryLoad() {
            // intentar cargar para diferentes diseños
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

...luego pega en el área de código y haz clic en guardar:

<div class="screenshot white-bg">
    <div class="title">Pegar y Guardar</div>
    <img class="screenshot-image" src="/images/installation-guides/squarespace-step-2-1-add-code-and-save.png" alt="Pegar y Guardar" />
</div>