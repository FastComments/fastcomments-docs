O trecho do FastComments para Comentários Ao Vivo no Framer está abaixo.

[inline-code-attrs-start title = 'Trecho FastComments específico para Framer - Comentários'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // alguns provedores alteram o trecho de código para torná-lo assíncrono
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

Ou, alternativamente, você pode usar o widget de Chat em Tempo Real. O trecho do FastComments para Chat em Tempo Real no Framer é:

[inline-code-attrs-start title = 'Trecho FastComments específico para Framer - Chat em Tempo Real'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {

        function tryLoad() {
            // alguns provedores alteram o trecho de código para torná-lo assíncrono
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

O FastComments oferece suporte ao editor Framer, então você deve ver algo assim depois de colar o código (pode ser que você precise clicar em `Publish`):

<div class="screenshot white-bg">
    <div class="title">Visualização do Widget de Comentários</div>
    <img class="screenshot-image" src="/images/installation-guides/framer-step-3-paste.png" alt="Visualização do Widget de Comentários" />
</div>

Agora, quando você visualizar seu site, deve ver a área de comentários! Na barra lateral do Framer você também pode definir o widget como largura total, se desejar.

Observe que o Framer limita a altura dos widgets e não suporta redimensionamento automático, então escolhemos aqui o widget de Chat ao Vivo, pois ele tem altura fixa.