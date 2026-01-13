Haz clic en el elemento HTML que acabas de añadir. En el editor de propiedades que aparece, pega el siguiente código en el campo HTML:

[inline-code-attrs-start title = 'Fragmento de código para comentarios en vivo de Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // Bubble suele cambiar el fragmento de código para que sea asíncrono
            const container = document.getElementById('fastcomments-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsUI) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsUI(container, {
                tenantId: 'demo',
                pageTitle: top.document.title,
                url: top.location.href,
                urlId: top.location.pathname
            });
            container.fastCommentsSetup = true;
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Insertar el código de FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Insertando el código de FastComments en el elemento HTML" />
</div>

Nota: Este código incluye un mecanismo de reintento para asegurar que FastComments se cargue correctamente en el entorno dinámico de Bubble. Otros fragmentos de código no funcionarán.

Recuerda reemplazar `'demo'` con el ID de tenant real de FastComments después de registrarte. Si has iniciado sesión en FastComments.com, ya debería estar reemplazado.