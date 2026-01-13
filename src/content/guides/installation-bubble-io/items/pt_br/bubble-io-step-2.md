Clique no elemento HTML que você acabou de adicionar. No editor de propriedades que aparecer, cole o código a seguir no campo HTML:

[inline-code-attrs-start title = 'Trecho de Código de Comentários ao Vivo do Bubble.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget" style="width: 100%;height: 100%;"></div>
<script>
    (function fcLoad() {
        function tryLoad() {
            // o Bubble tende a alterar o trecho de código para ser assíncrono
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
    <div class="title">Inserir o Código do FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/bubble-io-step-2-insert-code.png" alt="Inserindo o Código do FastComments no Elemento HTML" />
</div>

Observação: Este código inclui um mecanismo de tentativa/repetição para garantir que o FastComments seja carregado corretamente no ambiente dinâmico do Bubble. Outros trechos de código não funcionarão.

Lembre-se de substituir 'demo' pelo ID do seu tenant do FastComments após se cadastrar. Se você estiver logado no FastComments.com, isso já deverá ter sido substituído.