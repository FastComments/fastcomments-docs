El snippet de FastComments para Live Comments en Framer se muestra a continuación.

[inline-code-attrs-start title = 'Snippet de Comentarios Específico para Framer de FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // some providers change the code snippet to be async
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

O, alternativamente, puedes usar el widget de Streaming Chat. El snippet de FastComments para Streaming Chat en Framer es:

[inline-code-attrs-start title = 'Snippet de Streaming Chat Específico para Framer de FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // some providers change the code snippet to be async
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            if (container.fastCommentsSetup) {
                return;
            }
            window.FastCommentsLiveChat(container, {
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

FastComments es compatible con el editor de Framer, por lo que deberías ver algo como esto una vez que pegues el código (es posible que tengas que hacer clic en `Publish`):

<div class="screenshot white-bg">
    <div class="title">Vista previa del Widget de Comentarios</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Vista previa del Widget de Comentarios" />
</div>

Ahora, cuando veas tu sitio, ¡deberías ver el área de comentarios! En la barra lateral de Framer también puedes configurar el widget a ancho completo, si lo deseas.

Ten en cuenta que Framer limita la altura de los widgets y no admite el redimensionamiento automático, por lo que aquí hemos elegido el widget de Live Chat ya que tiene una altura fija.