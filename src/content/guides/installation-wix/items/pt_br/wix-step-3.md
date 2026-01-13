Este exemplo usa código personalizado para ser compatível com o Wix. **Você não poderá usar os trechos de código do FastComments de outros tutoriais.**

Abra o formulário para adicionar nosso diálogo HTML personalizado clicando em `Enter Code` e selecionando `HTML`:

<div class="screenshot white-bg">
    <div class="title">Etapa 3: Abrir Diálogo HTML</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-open-form.png" alt="Etapa 3: Abrir Diálogo HTML" />
</div>

Copie o trecho HTML a seguir e cole-o no diálogo, e clique em Atualizar:

[inline-code-attrs-start title = 'Trecho de código dos comentários do Wix'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        const config = {
            tenantId: "demo"
        };
        const instance = FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        window.onmessage = (event) => {
            if (event.data) {
                if (event.data.action === 'reload') {
                    console.log('Updating FastComments:', event.data.url);
                    config.urlId = event.data.url;
                    config.url = event.data.url;
                    instance.update(config);
                }
            }
        }
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Etapa 3: Colar e Salvar</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-paste-and-save.png" alt="Etapa 3: Colar e Salvar" />
</div>

Agora você deverá ver uma versão bem pequena do widget de comentários:

<div class="screenshot white-bg">
    <div class="title">Etapa 3: O Resultado</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-3-result.png" alt="Etapa 3: O Resultado" />
</div>

Em seguida, iremos posicionar e dimensionar isso para caber em nossa página.