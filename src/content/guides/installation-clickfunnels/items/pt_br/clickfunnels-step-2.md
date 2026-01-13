Agora que estamos no editor de template, precisamos decidir onde queremos exibir os comentários, ou o chat ao vivo.

Neste exemplo vamos adicioná-lo diretamente abaixo do vídeo. Passe o cursor sobre o elemento para adicionar o widget ao final dele, e clique em `ADD ELEMENT`:

<div class="screenshot white-bg">
    <div class="title">Adicionar elemento</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-4-add-element.png" alt="Add Element" />
</div>

Selecione `CUSTOM JS/HTML`:

<div class="screenshot white-bg">
    <div class="title">Selecione CUSTOM JS/HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-5-custom-js-html.png" alt="Select CUSTOM JS/HTML" />
</div>

Agora vamos abrir o editor de código onde vamos colar nosso código.

O ClickFunnels fica um pouco confuso nesta próxima etapa.

É importante que você *NÃO* selecione `Code` quando passar o cursor sobre o novo elemento. Em vez disso, selecione `SETTINGS`:

<div class="screenshot white-bg">
    <div class="title">Selecione SETTINGS</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-6-settings.png" alt="Select SETTINGS" />
</div>

Agora, no lado direito, podemos clicar em `Open Code Editor`:

<div class="screenshot white-bg">
    <div class="title">Clique em Open Code Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-7-open-code-editor.png" alt="Click Open Code Editor" />
</div>

Você verá um grande quadrado se abrir. É aqui que podemos colar nosso código. Copie o trecho a seguir (use o botão de copiar no canto superior direito):

[inline-code-attrs-start title = 'Trecho de código do Chat de Streaming do ClickFunnels'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-live-chat.min.js"></script>
<div id="fastcomments-live-chat-widget" style="width: 500px;min-height: 780px;"></div>
<style>
    #fastcomments-live-chat-widget iframe {
        min-height: 780px;
    }
</style>
<script>
    (function fcLoad() {
        function tryLoad() {
            // alguns provedores alteram o trecho de código para ser assíncrono
            const container = document.getElementById('fastcomments-live-chat-widget');
            if (!container) {
                return waitRetry();
            }
            if (!window.FastCommentsLiveChat) {
                return waitRetry();
            }
            window.FastCommentsLiveChat(container, {
                tenantId: 'demo'
            });
        }
        function waitRetry() {
            setTimeout(tryLoad, 500);
        }
        tryLoad();
    })();
</script>
[inline-code-end]

Este trecho de código é para o nosso produto Streaming Chat, que combina bem com vídeos. Se você quiser o trecho de código do widget Live Commenting em vez disso, que
funciona melhor em páginas regulares ou posts de blog, ele está no final deste tutorial.

Quando colarmos o trecho de código na janela, deve ficar assim:

<div class="screenshot white-bg">
    <div class="title">Colar código</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-8-paste.png" alt="Paste Code" />
</div>

Agora só precisamos fechar a caixa:

<div class="screenshot white-bg">
    <div class="title">Fechar</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-9-close.png" alt="Close" />
</div>

Agora você pode visualizar suas alterações! Sinta-se à vontade para mover o widget e ver onde você mais gosta.

<div class="screenshot white-bg">
    <div class="title">Visualizar</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-10-preview.png" alt="Preview" />
</div>

Sucesso! Não se esqueça de testar no celular!

<div class="screenshot white-bg">
    <div class="title">Sucesso!</div>
    <img class="screenshot-image" src="/images/installation-guides/clickfunnels-step-11-success.png" alt="Success!" />
</div>