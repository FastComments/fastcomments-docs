Para que a integração do Weebly com o FastComments funcione corretamente, precisamos adicionar **duas** pequenas partes de código.

O primeiro trecho serve para ocultar a mensagem do Weebly "Comments are Closed", e o segundo é para realmente carregar o FastComments.

Primeiro, copie este pequeno trecho de código:

[inline-code-attrs-start title = 'Trecho de código do cabeçalho do FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

Em seguida, na mesma página de configurações do `Step One`, clique no `+` ao lado de `Post header code`.

<div class="screenshot white-bg">
    <div class="title">Abrir Código de Cabeçalho da Postagem</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="Abrir Código de Cabeçalho da Postagem" />
</div>

Você verá uma caixa de texto aberta assim:

<div class="screenshot white-bg">
    <div class="title">Código do Cabeçalho da Postagem Aberto</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="Código do Cabeçalho da Postagem Aberto" />
</div>

Agora vamos colar nosso trecho de código:

<div class="screenshot white-bg">
    <div class="title">Trecho de Código do Cabeçalho Colado</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="Trecho de Código do Cabeçalho Colado" />
</div>

A seguir vem o código do rodapé para ativar o FastComments. Clique no sinal de mais ao lado de `Post footer code`:

<div class="screenshot white-bg">
    <div class="title">Abrir Código de Rodapé da Postagem</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="Abrir Código de Rodapé da Postagem" />
</div>

Copie este trecho de código que foi projetado **especificamente para o Weebly**:

[inline-code-attrs-start title = 'Trecho de código do rodapé do FastComments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // remover botão de exibir comentários
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

Agora vamos colar nosso código do rodapé:

<div class="screenshot white-bg">
    <div class="title">Código do Rodapé da Postagem Adicionado</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="Código do Rodapé da Postagem Adicionado" />
</div>

Pronto!

---