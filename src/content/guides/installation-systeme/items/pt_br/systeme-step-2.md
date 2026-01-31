Agora vamos copiar nosso código. Se o trecho de código mostrar `tenantId: "demo"` na linha 6 então você deve entrar na sua conta FastComments e então atualizar esta página para que o trecho copiado tenha o id da sua conta.

[inline-code-attrs-start title = 'Trecho do Systeme.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo"
    }];
</script>
[inline-code-end]

Agora cole-o no editor e clique em salvar:

<div class="screenshot white-bg">
    <div class="title">Adicione o código do FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/systeme-add-code.png" alt="Adicione o código do FastComments" />
</div>

... então salve seu site. É isso!